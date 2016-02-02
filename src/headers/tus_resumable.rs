use std::fmt;
use hyper::header::{parsing, Header, HeaderFormat};
use hyper::error;

#[derive(Clone, Debug)]
pub struct TusResumable {
    pub version: String,
}

impl TusResumable {
    pub fn new() -> Self {
        TusResumable { version: String::from("1.0.0") }
    }
}

impl Header for TusResumable {
    fn header_name() -> &'static str {
        "Tus-Resumable"
    }

    fn parse_header(raw: &[Vec<u8>]) -> error::Result<Self> {
        let value = try!(parsing::from_one_raw_str(raw));
        Ok(TusResumable { version: value })
    }
}

impl HeaderFormat for TusResumable {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.version)
    }
}
