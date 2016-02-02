extern crate hyper;

use hyper::server::{Request, Response};
use hyper::status::StatusCode;
use hyper::method::Method;

mod headers;
use headers::TusVersion;

mod utils;

// use hyper::header::{Headers, CacheControl, CacheDirective};
// use hyper::uri::RequestUri;

// use std::path::{Path, PathBuf};

pub fn tus_handler(request: Request, mut response: Response) {
    let log_string = format!("{remote_addr} | {version} | {method:<7} | {uri}",
                             remote_addr = request.remote_addr,
                             version = request.version,
                             method = request.method.to_string(),
                             uri = request.uri);

    if !utils::validate_method(&request.method) {
        // Check if the method is allowed
        *response.status_mut() = StatusCode::MethodNotAllowed;

    } else if !utils::version_check(&request.headers) {
        // Check if the version is supported
        response.headers_mut().set(TusVersion::new());
        *response.status_mut() = StatusCode::PreconditionFailed;

    } else {
        // handle the method
        match request.method {
            // Method::Head => handle_head_method(request, &mut response),
            // Method::Patch => handle_patch_method(request, &mut response),
            // Method::Options => handle_options_method(&mut response),
            Method::Head => println!("head"),
            Method::Patch => println!("patch"),
            Method::Options => println!("options"),
            _ => (),
        }
    }

    println!("{request} | {response_status}",
             request = log_string,
             response_status = response.status());
}
