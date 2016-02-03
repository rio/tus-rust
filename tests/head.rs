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

    let response = client.head("http://localhost:4000/").headers(headers).send().unwrap();

    assert_eq!(response.status, StatusCode::Forbidden)
}

#[test]
fn return_404_on_missing_file() {
    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    let response = client.head("http://localhost:4000/missingfile")
        .headers(headers)
        .send()
        .unwrap();

    assert_eq!(response.status, StatusCode::NotFound)
}

#[test]
fn return_200_with_offset_empty_file() {
    // create a temporary file
    let file_name = "empty_temp_file";
    common::create_temp_file(file_name, 0);

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    let result = client.head("http://localhost:4000/empty_temp_file")
        .headers(headers)
        .send();

    common::remove_temp_file(file_name);

    match result {
        Ok(response) => {
            let upload_offset = response.headers.get::<UploadOffset>().unwrap();
            assert_eq!(response.status, StatusCode::Ok);
            assert_eq!(upload_offset.offset, UploadOffset::new(0).offset)
        }
        Err(error) => {
            panic!(format!("{:?}", error))
        }
    }
}

#[test]
fn return_200_with_offset_non_empty_file() {
    // create a temporary file
    let file_name = "non_empty_temp_file";
    common::create_temp_file(file_name, 100);

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    let result = client.head("http://localhost:4000/non_empty_temp_file")
        .headers(headers)
        .send();

    common::remove_temp_file(file_name);

    match result {
        Ok(response) => {
            let upload_offset = response.headers.get::<UploadOffset>().unwrap();
            assert_eq!(response.status, StatusCode::Ok);
            assert_eq!(upload_offset.offset, UploadOffset::new(100).offset)
        }
        Err(error) => {
            panic!(format!("{:?}", error))
        }
    }
}
