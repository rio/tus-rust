use hyper::header::Headers;
use hyper::method::Method;

use headers::TusResumable;

pub fn version_check(headers: &Headers) -> bool {
    match headers.get::<TusResumable>() {
        Some(tus_resumable) => tus_resumable.version == "1.0.0",
        None => false,
    }
}

pub fn validate_method(method: &Method) -> bool {
    match method {
        &Method::Head | &Method::Patch | &Method::Options => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::Headers;

    use super::version_check;

    #[test]
    fn fail_version_check_missing_header() {
        let mut headers = Headers::new();
        assert!(!version_check(&headers))
    }

    #[test]
    fn fail_version_check_wrong_version() {
        let mut headers = Headers::new();
        headers.set_raw("Tus-Resumable", vec![b"0.1.0".to_vec()]);

        assert!(!version_check(&headers))
    }

    #[test]
    fn pass_version_check() {
        let mut headers = Headers::new();
        headers.set_raw("Tus-Resumable", vec![b"1.0.0".to_vec()]);

        assert!(version_check(&headers))
    }
}
