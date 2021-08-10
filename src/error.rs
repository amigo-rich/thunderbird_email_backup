#[derive(Debug)]
pub enum Error {
    ArchiveFileName(std::path::StripPrefixError, std::path::PathBuf),
    CreateArchive(std::io::Error, Option<String>),
    OpenFile(std::io::Error, String),
    PathNotADir(String),
    StdIOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArchiveFileName(error, pathbuf) => write!(
                f,
                "While creating an archive file name: '{:?}' a strip prefix error occured: '{}'",
                pathbuf, error
            ),
            Self::CreateArchive(error, maybe_path) => write!(
                f,
                "While trying to create the archive: '{:?}' an IO error occurred: '{}'",
                maybe_path, error
            ),
            Error::OpenFile(error, path) => write!(
                f,
                "While attempting to open the file: '{:?}' for reading, an IO error occurred: '{}'",
                path, error
            ),
            Error::PathNotADir(path) => {
                write!(f, "The supplied path: '{}' is not a directory", path)
            }
            Error::StdIOError(error) => write!(f, "An IO error occured: '{}'", error),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::StdIOError(e)
    }
}
