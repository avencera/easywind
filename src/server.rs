pub mod error;
pub mod port;

use axum::{
    body::{self, Empty, Full},
    extract::{Path, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use eyre::Result;
use log::info;
use sailfish::TemplateOnce;
use std::{
    fmt::Display,
    fs::{DirEntry, File as StdFile},
    io::Read,
    net::SocketAddr,
    path::PathBuf,
};

use self::error::Error;

#[derive(Clone)]
struct AppState {
    root_dir: PathBuf,
}

#[derive(Debug)]
enum File {
    Dir(PathBuf),
    File(PathBuf),
}

impl File {
    fn into_path_buf(self) -> PathBuf {
        match self {
            Self::Dir(path) => path,
            Self::File(path) => path,
        }
    }
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

#[derive(TemplateOnce)]
#[template(path = "index.html.stpl")]
struct IndexTemplate {
    links: Vec<String>,
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
    let root = canonicalize(&state.root_dir)?;
    index_template(&root, root.clone())
}

async fn path(
    State(state): State<AppState>,
    Path(path): Path<PathBuf>,
) -> Result<impl IntoResponse, Error> {
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

fn index_template(root_dir: &PathBuf, path: PathBuf) -> Result<Html<String>, Error> {
    let root = canonicalize(root_dir)?;

    let paths: Vec<File> = std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(Into::into)
        .collect::<Vec<_>>();

    let links = paths
        .into_iter()
        .map(|path| path.into_path_buf())
        .filter_map(|path| Some(path.strip_prefix(&root).ok()?.to_string_lossy().to_string()))
        .collect::<Vec<_>>();

    let template = IndexTemplate { links }.render_once()?;

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

pub async fn start(root_dir: PathBuf) -> Result<()> {
    let state = AppState { root_dir };

    let app = Router::new()
        .route("/", get(root))
        .route("/*path", get(path))
        .with_state(state.clone());

    let port = port::default_or_available(3000).expect("Unable to find available port");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("serving html from {}", state.root_dir.to_string_lossy());
    info!("starting server at {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
