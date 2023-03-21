use std::fmt::Display;

#[derive(Default, Clone)]
pub struct Response {
    pub bytes: Vec<u8>,
}

impl Response {
    pub fn new(response: Vec<u8>) -> Self {
        Self { bytes: response }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match String::from_utf8(self.bytes.clone()) {
            Ok(s) => s,
            Err(e) => e.to_string(),
        };

        write!(f, "{}", s)
    }
}

impl From<String> for Response {
    fn from(s: String) -> Self {
        Self { bytes: s.as_bytes().to_vec() }
    }
}
