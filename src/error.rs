use thiserror::Error;

#[derive(Error, Debug)]
pub enum TriangulationError {
    #[error("No max error in queue.")]
    MaxErrorRetrievalError,
    #[error("Priority queue is empty.")]
    EmptyQueueError,
    #[error("Length of heights data is not equal to width * height.")]
    InvalidDataLengthError,
}

pub type TriangulationResult<T> = std::result::Result<T, TriangulationError>;
