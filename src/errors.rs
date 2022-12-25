pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Error {
        return Error {
            msg: msg.to_string(),
        };
    }
    pub fn to_string(&self) -> &str {
        return self.msg.as_str();
    }
}
