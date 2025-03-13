pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    inner: Box<ErrorKind>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Box::new(kind),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}
impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::new(ErrorKind::OsynicDownloaderError(e))
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::new(ErrorKind::OsynicDownloaderError(e.to_string()))
    }
}

pub enum ErrorKind {
    OsynicDownloaderError(String),
    VielporkError(vielpork::error::Error),
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicDownloaderError(e) => {
                write!(f, "OsynicDownloaderError: {}", e)
            }
            ErrorKind::VielporkError(e) => {
                write!(f, "VielporkError: {:?}", e)
            }
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicDownloaderError(e) => {
                write!(f, "OsynicDownloaderError: {}", e)
            }
            ErrorKind::VielporkError(e) => {
                write!(f, "VielporkError: {}", e)
            }
        }
    }
}
