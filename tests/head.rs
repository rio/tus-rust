extern crate hyper;
extern crate tus;

use hyper::Client;
use hyper::status::StatusCode;
use hyper::header::Headers;

use std::fs;
use std::env;

use tus::headers::UploadOffset;

fn create_temp_file(name: &str, size: u64) {
    let mut temp_file_path = env::temp_dir();
    temp_file_path.push(name);

    let temp_file = fs::File::create(temp_file_path).unwrap();
    temp_file.set_len(size).unwrap();
}

fn remove_temp_file(name: &str) {
    let mut temp_file_path = env::temp_dir();
    temp_file_path.push(name);

    fs::remove_file(temp_file_path).unwrap()
}

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
    create_temp_file(file_name, 0);

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    match client.head("http://localhost:4000/empty_temp_file")
                         .headers(headers)
                         .send() {
        Ok(response) => {
            remove_temp_file(file_name);
            let upload_offset = response.headers.get::<UploadOffset>().unwrap();
            assert_eq!(response.status, StatusCode::Ok);
            assert_eq!(upload_offset.offset, UploadOffset::new(0).offset)
        }
        Err(error) => {
            remove_temp_file(file_name);
            panic!(format!("{:?}", error))
        }
                         }
}

#[test]
fn return_200_with_offset_non_empty_file() {
    // create a temporary file
    let file_name = "non_empty_temp_file";
    create_temp_file(file_name, 100);

    let client = Client::new();
    let mut headers = Headers::new();
    headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

    match client.head("http://localhost:4000/non_empty_temp_file")
                .headers(headers)
                .send() {
        Ok(response) => {
            remove_temp_file(file_name);
            let upload_offset = response.headers.get::<UploadOffset>().unwrap();
            assert_eq!(response.status, StatusCode::Ok);
            assert_eq!(upload_offset.offset, UploadOffset::new(100).offset)
        }
        Err(error) => {
            remove_temp_file(file_name);
            panic!(format!("{:?}", error))
        }
    }
}
