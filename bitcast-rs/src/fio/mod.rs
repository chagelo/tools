pub mod file_io;
use crate::errors::Result;

pub trait IOManager: Sync + Send{
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;
    fn write(&self, buf: &[u8]) -> Result<usize>;
    fn sync(&self) -> Result<()>;
}