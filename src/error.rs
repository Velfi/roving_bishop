use thiserror::Error;

#[derive(Error, Debug)]
pub enum BishopArtError {
    #[error("Invalid cell index, no cell at index {index}")]
    InvalidCellIndex { index: isize },
}
