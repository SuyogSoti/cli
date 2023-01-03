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

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(&error.to_string())
    }
}
impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Error::new(&error.to_string())
    }
}
impl From<tmux_interface::Error> for Error {
    fn from(error: tmux_interface::Error) -> Self {
        Error::new(&error.to_string())
    }
}
