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

#[derive(Clone, Debug)]
pub struct TusVersion {
    pub versions: Vec<String>,
}

impl TusVersion {
    pub fn new() -> Self {
        TusVersion {
            versions: vec![
                String::from("1.0.0"),
                //String::from("0.2.2"),
                //String::from("0.2.1"),
            ],
        }
    }
}

impl Header for TusVersion {
    fn header_name() -> &'static str {
        "Tus-Version"
    }

    fn parse_header(raw: &[Vec<u8>]) -> error::Result<Self> {
        let values = try!(parsing::from_comma_delimited(raw));
        Ok(TusVersion { versions: values })
    }
}

impl HeaderFormat for TusVersion {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.versions.join(","))
    }
}

#[derive(Clone, Debug)]
pub struct UploadOffset {
    pub offset: u64,
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
