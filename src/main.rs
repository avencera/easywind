pub mod consts;

use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use eyre::Result;
use sailfish::TemplateOnce;
use std::{fmt::Display, fs::DirEntry, io::Read, net::SocketAddr, path::PathBuf};

#[derive(Debug)]
enum File {
    Dir(PathBuf),
    File(PathBuf),
}

impl File {
    fn to_path_buf(self) -> PathBuf {
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

async fn root() -> Html<String> {
    let root = std::fs::canonicalize(consts::ROOT_DIR).unwrap();
    index_template(root)
}

async fn path(Path(path): Path<PathBuf>) -> impl IntoResponse {
    let mut root = std::fs::canonicalize(consts::ROOT_DIR).unwrap();
    root.push(path);

    let new_path = root;

    if new_path.is_dir() {
        index_template(new_path).into_response()
    } else {
        if new_path.ends_with(".html") {
            std::fs::read_to_string(new_path).unwrap().into_response()
        } else {
            static_path(new_path).into_response()
        }
    }
}

fn index_template(path: PathBuf) -> Html<String> {
    let root = std::fs::canonicalize(consts::ROOT_DIR)
        .unwrap()
        .to_string_lossy()
        .to_string();

    let paths: Vec<File> = std::fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(Into::into)
        .collect::<Vec<_>>();

    let links = paths
        .into_iter()
        .map(|path| path.to_path_buf())
        .map(|path| {
            path.strip_prefix(&root)
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();

    let template = IndexTemplate { links }.render_once().unwrap();

    template.into()
}

fn static_path(path: PathBuf) -> impl IntoResponse {
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();

    match std::fs::File::open(path).ok() {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(mut file) => {
            let mut buffer =
                vec![0; file.metadata().expect("unable to get metadata").len() as usize];

            file.read_exact(&mut buffer).expect("buffer overflow");

            Response::builder()
                .status(StatusCode::OK)
                .header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_str(mime_type.as_ref()).unwrap(),
                )
                .body(body::boxed(Full::from(buffer)))
                .unwrap()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize logging
    pretty_env_logger::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/*path", get(path));

    // run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
