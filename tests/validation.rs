extern crate yup_hyper_mock;
extern crate hyper;
extern crate tus;

use hyper::server::{Request, Response};
use hyper::net::NetworkStream;
use hyper::header::Headers;
use hyper::status::StatusCode;
use hyper::buffer::BufReader;
use std::net::SocketAddr;
use std::str::FromStr;
use self::yup_hyper_mock::MockStream;

#[test]
fn bad_version_good_method_return_412() {
    let mut mock = MockStream::with_input(b"\
    HEAD / HTTP/1.1\r\n\
    Host: http://example.domain\r\n\
    Tus-Resumable: 1.0.1\r\n\
    \r\n");

    let mock: &mut NetworkStream = &mut mock;
    let mut req_stream = BufReader::new(mock);
    let request = Request::new(&mut req_stream, SocketAddr::from_str("[::1]:8000").unwrap())
                      .unwrap();

    let mut headers = Headers::new();
    let mut res_stream = MockStream::new();
    let response = Response::new(&mut res_stream, &mut headers);

    tus::tus_handler(request, &response);

    assert_eq!(response.status(), StatusCode::PreconditionFailed)
}

#[test]
fn good_version_bad_method_return_405() {
    // TODO
    assert!(false)
}

#[test]
fn bad_version_bad_method_return_412() {
    // TODO
    assert!(false)
}
