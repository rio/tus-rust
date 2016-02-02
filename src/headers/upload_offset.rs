use std::fmt;
use hyper::header::{parsing, Header, HeaderFormat};
use hyper::error;

#[derive(Clone, Debug)]
pub struct UploadOffset {
    offset: u64,
}

impl UploadOffset {
    pub fn new(offset: u64) -> Self {
        UploadOffset { offset: offset }
    }
}

impl Header for UploadOffset {
    fn header_name() -> &'static str {
        "Upload-Offset"
    }

    fn parse_header(raw: &[Vec<u8>]) -> error::Result<Self> {
        let value = try!(parsing::from_one_raw_str(raw));
        Ok(UploadOffset { offset: value })
    }
}

impl HeaderFormat for UploadOffset {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*format!("{}", self.offset))
    }
}
