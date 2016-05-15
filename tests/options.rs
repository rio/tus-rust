extern crate hyper;
extern crate tus;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::Headers;
use hyper::method::Method;

use tus::headers::{TusResumable, TusVersion};

#[test]
fn return_204() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    let response = client.request(Method::Options, "http://localhost:4000/")
        .headers(headers)
        .send()
        .unwrap();

    let tus_version = response.headers.get::<TusVersion>().unwrap();

    assert!(!response.headers.has::<TusResumable>());
    assert_eq!(tus_version.versions, TusVersion::new().versions);
    assert_eq!(response.status, StatusCode::NoContent)
}
