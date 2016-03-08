extern crate hyper;
extern crate tus;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::Headers;

use tus::headers::UploadOffset;

mod common;

#[test]
fn return_403_on_missing_file_id() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);
    headers.set_raw("Content-Type", vec![b": application/offset+octet-stream".to_vec()]);
    headers.set_raw("Upload-Offset", vec![b"100".to_vec()]);
    headers.set_raw("Content-Length", vec![b"20".to_vec()]);

    let response = client.patch("http://localhost:4000/").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::Forbidden)
}

#[test]
fn return_412_on_missing_upload_offset_header() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);
    headers.set_raw("Content-Type", vec![b"application/offset+octet-stream".to_vec()]);
    headers.set_raw("Content-Length", vec![b"20".to_vec()]);

    let response = client.patch("http://localhost:4000/random_file").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::PreconditionFailed)
}

#[test]
fn return_412_on_missing_content_type_header() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);
    headers.set_raw("Upload-Offset", vec![b"100".to_vec()]);
    headers.set_raw("Content-Length", vec![b"20".to_vec()]);

    let response = client.patch("http://localhost:4000/random_file").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::PreconditionFailed)
}

#[test]
fn return_415_on_wrong_content_type_header() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);
    headers.set_raw("Upload-Offset", vec![b"100".to_vec()]);
    headers.set_raw("Content-Type", vec![b"application/json".to_vec()]);
    headers.set_raw("Content-Length", vec![b"20".to_vec()]);

    let response = client.patch("http://localhost:4000/random_file").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::UnsupportedMediaType)
}

#[test]
fn return_409_on_wrong_offset() {
    let file_name = "non_empty_temp_file";
    common::create_temp_file(file_name, 100);

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);
    headers.set_raw("Upload-Offset", vec![b"38".to_vec()]);
    headers.set_raw("Content-Type", vec![b"application/offset+octet-stream".to_vec()]);
    headers.set_raw("Content-Length", vec![b"20".to_vec()]);

    let response = client.patch("http://localhost:4000/non_empty_temp_file").headers(headers).send().unwrap();

    common::remove_temp_file(file_name);

    assert_eq!(response.status, StatusCode::Conflict)
}

#[test]
fn return_204_with_upload_offset_successful_patch() {
    let file_name = "non_empty_temp_file_that_grows";
    common::create_temp_file(file_name, 100);

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);
    headers.set_raw("Upload-Offset", vec![b"100".to_vec()]);
    headers.set_raw("Content-Type", vec![b"application/offset+octet-stream".to_vec()]);

    let response = client.patch("http://localhost:4000/non_empty_temp_file_that_grows").headers(headers).body("0123456789abcdefghij").send().unwrap();

    common::remove_temp_file(file_name);

    let returned_upload_offset = response.headers.get::<UploadOffset>().unwrap();

    assert_eq!(returned_upload_offset.offset, 120);
    assert_eq!(response.status, StatusCode::NoContent);
}
