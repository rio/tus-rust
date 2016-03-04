use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::uri::RequestUri;
use hyper::mime::{TopLevel, SubLevel, Mime};

use headers::{TusResumable, UploadOffset};

use std::path::Path;
use std::env;

pub fn handle_patch_method(request: Request, response: &mut Response) {
    let uri = match request.uri {
        RequestUri::AbsolutePath(uri) => uri,
        _ => {
            *response.status_mut() = StatusCode::Forbidden;
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

    let upload_offset = match request.headers.get::<UploadOffset>() {
        Some(upload_offset) => upload_offset,
        None => {
            *response.status_mut() = StatusCode::PreconditionFailed;
            return;
        }
    };

    match request.headers.get::<ContentType>() {
        Some(&ContentType(Mime(TopLevel::Application, SubLevel::Ext(ref kind), _))) => {
            // TODO: Figure out what this ref thing is and why i need to do this...
            match &kind[..] {
                "offset+octet-stream" => (),
                _ => {
                    *response.status_mut() = StatusCode::UnsupportedMediaType;
                    return;
                }
            }
        }
        None => {
            *response.status_mut() = StatusCode::PreconditionFailed;
            return;
        }
        _ => {
            *response.status_mut() = StatusCode::UnsupportedMediaType;
            return;
        }
    };

    let mut file_path = env::temp_dir();
    file_path.push(file_name);

    if file_path.is_file() {
        let file_offset = file_path.metadata().unwrap().len();
        if file_offset != upload_offset.offset {
            *response.status_mut() = StatusCode::Conflict;
        }

    } else {
        *response.status_mut() = StatusCode::NotFound;
    }

    response.headers_mut().set(TusResumable::new());
}
