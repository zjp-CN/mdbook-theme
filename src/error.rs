#[derive(Debug)]
pub enum Error {
    StrNotFound,
    FileNotCreated,
    DirNotCreated,
}

pub type Result<T> = std::result::Result<T, Error>;
