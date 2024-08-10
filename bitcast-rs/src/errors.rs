use std::result;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("failed to read from file")]
    FailedReadFromDatFile,

    #[error("failed to write to data file")]
    FailedWriteToDataFile,


    #[error("failed to sync data file")]
    FailedSyncDataFile,

    #[error("failed to open data file")]
    FailedToOpenDataFile,
}


pub type Result<T> = result::Result<T, Errors>;