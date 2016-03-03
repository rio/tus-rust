use headers::TusVersion;

use hyper::status::StatusCode;
use hyper::server::Response;

pub fn handle_options_method(response: &mut Response) {
    // TODO: add Tus-Extension and Tus-Max-Size headers
    response.headers_mut().set(TusVersion::new());
    *response.status_mut() = StatusCode::NoContent
}
