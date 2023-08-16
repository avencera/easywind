pub mod error;
pub mod no_cache;
pub mod port;
pub mod reload;

use axum::{
    body::{self, Empty, Full},
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use eyre::Result;
use log::{error, info};
use notify_debouncer_mini::{notify::RecursiveMode, DebounceEventResult};
use tower_livereload::LiveReloadLayer;

use std::{
    fmt::Display,
    fs::{DirEntry, File as StdFile},
    io::Read,
    net::SocketAddr,
    path::PathBuf,
    time::Duration,
};

use self::error::Error;
use crate::template::{TemplateName, TEMPLATE};

#[cfg(not(feature = "dev"))]
static APP_CSS: &str = include_str!("../static/app.css");

#[derive(Clone)]
struct AppState {
    root_dir: PathBuf,
}

pub struct ServerArgs {
    pub root_dir: PathBuf,
    pub port: u16,
    pub open: bool,
}

#[derive(Debug)]
enum File {
    Dir(PathBuf),
    File(PathBuf),
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Self::Dir(path) => path.to_string_lossy(),
                Self::File(path) => path.to_string_lossy(),
            }
            .as_ref(),
        )
    }
}

impl From<DirEntry> for File {
    fn from(dir_entry: DirEntry) -> Self {
        let path = dir_entry.path();
        if path.is_dir() {
            Self::Dir(path)
        } else {
            Self::File(path)
        }
    }
}

fn canonicalize(path: &PathBuf) -> Result<PathBuf, Error> {
    std::fs::canonicalize(path).map_err(|_| Error::InvalidRootDir(path.clone()))
}

async fn root(State(state): State<AppState>) -> Result<Html<String>, Error> {
    info!("GET /");
    let root = canonicalize(&state.root_dir)?;

    // if is a directory with an "index.html" file, serve that
    if root.is_dir() {
        if let Ok(index) = std::fs::read_to_string(root.join("index.html")) {
            return Ok(Html(index));
        }
    }

    index_template(&root, root.clone())
}

async fn path(
    State(state): State<AppState>,
    Path(path): Path<PathBuf>,
) -> Result<impl IntoResponse, Error> {
    info!("GET {}", path.to_string_lossy());

    let root = canonicalize(&state.root_dir)?;

    let mut path_to_serve = root.clone();
    path_to_serve.push(path);

    // directory list all files
    if path_to_serve.is_dir() {
        return Ok(index_template(&root, path_to_serve).into_response());
    };

    // serve html files
    if path_to_serve.ends_with(".html") {
        return Ok(std::fs::read_to_string(path_to_serve)?.into_response());
    }

    // any other file, create response depending on mime type
    Ok(static_path(path_to_serve).into_response())
}

async fn serve_internal_css() -> impl IntoResponse {
    let mut headers = http::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());

    #[cfg(feature = "dev")]
    return (headers, std::fs::read_to_string("static/app.css").unwrap());

    #[cfg(not(feature = "dev"))]
    (headers, APP_CSS)
}

fn index_template(root_dir: &PathBuf, path: PathBuf) -> Result<Html<String>, Error> {
    let root = canonicalize(root_dir)?;

    let mut current_dir = path
        .strip_prefix(&root)
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    current_dir.push('/');

    let paths: Vec<PathBuf> = std::fs::read_dir(&path)?
        .filter_map(Result::ok)
        .map(|dir_entry| dir_entry.path())
        .collect::<Vec<_>>();

    let mut links = paths
        .into_iter()
        .filter_map(|path| {
            Some((
                path.is_dir(),
                path.strip_prefix(&root).ok()?.to_string_lossy().to_string(),
            ))
        })
        .collect::<Vec<_>>();

    links.sort_by(|(is_dir_a, path_a), (is_dir_b, path_b)| {
        if *is_dir_a == *is_dir_b {
            path_a.cmp(path_b)
        } else if *is_dir_a {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let ctx: minijinja::Value = minijinja::context! {links => links, current_dir => current_dir};
    let template = TEMPLATE.render(TemplateName::Index, &ctx);

    Ok(template.into())
}

fn static_path(path: PathBuf) -> Result<impl IntoResponse, Error> {
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();

    match StdFile::open(&path).ok() {
        None => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body::boxed(Empty::new()))?;

            Ok(response)
        }

        Some(mut file) => {
            let mut buffer = vec![
                0;
                file.metadata()
                    .map_err(|_| Error::FileMetadataError(path.clone()))?
                    .len() as usize
            ];

            file.read_exact(&mut buffer)
                .map_err(|_| Error::FileReadBufferOverflow(path))?;

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(body::boxed(Full::from(buffer)))?;

            Ok(response)
        }
    }
}

pub async fn start(args: ServerArgs) -> Result<()> {
    if args.open {
        tokio::task::spawn(async move {
            tokio::time::sleep(Duration::from_millis(200)).await;
            if let Err(error) = open::that(format!("http://localhost:{}", args.port)) {
                error!("Unable to open browser: {error:?}");
            }
        });
    }

    let state = AppState {
        root_dir: args.root_dir.clone(),
    };

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let mut debouncer = notify_debouncer_mini::new_debouncer(
        Duration::from_millis(80),
        None,
        move |event: DebounceEventResult| {
            let _ = reload::handle_reload(event, &reloader);
        },
    )
    .unwrap();

    debouncer
        .watcher()
        .watch(&args.root_dir, RecursiveMode::Recursive)?;

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/__internal_only_easywind_css_file__.css",
            get(serve_internal_css),
        )
        .route("/*path", get(path))
        .with_state(state.clone())
        .layer(livereload)
        .layer(no_cache::layer());

    let port = port::default_or_available(args.port).expect("Unable to find available port");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Serving html from {}", state.root_dir.to_string_lossy());
    info!("Starting server at {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
