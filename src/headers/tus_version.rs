use std::fmt;
use hyper::header::{parsing, Header, HeaderFormat};
use hyper::error;

#[derive(Clone, Debug)]
pub struct TusVersion {
    versions: Vec<String>,
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
