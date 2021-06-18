#[derive(Debug)]
pub enum Error {
    NotFound,
    FilesNotCreated,
}

pub type Result<T> = std::result::Result<T, Error>;
