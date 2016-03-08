use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::header::ContentType;
use hyper::uri::RequestUri;
use hyper::mime::{TopLevel, SubLevel, Mime};

use headers::{TusResumable, UploadOffset};

use std::path::Path;
use std::env;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Read, Write, BufReader, BufWriter};

pub fn handle_patch_method(request: &mut Request, response: &mut Response) {
    let uri = match request.uri.clone() {
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
        Some(upload_offset) => upload_offset.clone(),
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
    }

    let mut file_path = env::temp_dir();
    file_path.push(file_name);

    if file_path.is_file() {
        let file_offset = file_path.metadata().unwrap().len();
        if file_offset != upload_offset.offset {
            *response.status_mut() = StatusCode::Conflict;
            return;
        }

    } else {
        *response.status_mut() = StatusCode::NotFound;
        return;
    }

    let open_file = match OpenOptions::new().write(true).open(file_path) {
        Ok(file_descriptor) => file_descriptor,
        Err(_) => {
            *response.status_mut() = StatusCode::InternalServerError;
            return;
        }
    };

    let mut reader = BufReader::new(request);
    let mut writer = BufWriter::new(open_file);
    let _ = writer.seek(SeekFrom::Start(upload_offset.offset));

    let mut counter = 0;

    loop {
        let mut buffer: [u8; 256] = [0; 256];
        let bytes_read = match reader.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(_) => {
                break;
            }
        };

        if bytes_read != 0 {
            let bytes_written = match writer.write(&buffer[0..bytes_read]) {
                Ok(bytes_written) => bytes_written,
                Err(_) => {
                    break;
                }
            };
            counter = counter + bytes_written as u64;
        } else {
            break;
        }
    }

    response.headers_mut().set(UploadOffset::new(counter as u64 + upload_offset.offset));
    response.headers_mut().set(TusResumable::new());
    *response.status_mut() = StatusCode::NoContent;
}
