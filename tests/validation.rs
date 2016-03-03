extern crate hyper;
extern crate tus;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::Headers;

#[test]
fn bad_version_good_method_return_412() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.1".to_vec()]);

    let response = client.head("http://localhost:4000").headers(headers).send().unwrap();
    let raw_tus_version = response.headers.get_raw("Tus-Version").unwrap();

    assert_eq!(response.status, StatusCode::PreconditionFailed);
    assert_eq!(raw_tus_version, &[b"1.0.0"])
}

#[test]
fn good_version_bad_method_return_405() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    let response = client.put("http://localhost:4000").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::MethodNotAllowed)
}

#[test]
fn bad_version_bad_method_return_405() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.1".to_vec()]);

    let response = client.put("http://localhost:4000").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::MethodNotAllowed)
}
