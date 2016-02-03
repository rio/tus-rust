use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use hyper::header::{CacheDirective, CacheControl};

use std::path::Path;
use std::env;

use headers::{UploadOffset, TusResumable};

pub fn handle_head_method(request: Request, response: &mut Response) {
    let uri = match request.uri {
        RequestUri::AbsolutePath(uri) => uri,
        _ => {
            *response.status_mut() = StatusCode::NotFound;
            return;
        }
    };

    let file_name = match Path::new(&uri).file_name() {
        Some(file_name) => file_name,
        None => {
            *response.status_mut() = StatusCode::Forbidden;
            return;
        }
    };

    let mut file_path = env::temp_dir();
    file_path.push(file_name);

    if file_path.is_file() {
        let offset = file_path.metadata().unwrap().len();
        response.headers_mut().set(UploadOffset::new(offset))

    } else {
        *response.status_mut() = StatusCode::NotFound;
    }

    // Always set version header and disable cache
    response.headers_mut().set(TusResumable::new());
    response.headers_mut().set(CacheControl(vec![CacheDirective::NoStore]))
}
