use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
    os::unix::prelude::FileExt
};

use log::error;

use super::IOManager;
use crate::errors::{Result, Errors};
use parking_lot::RwLock;

pub struct FileIO {
    fd: Arc<RwLock<File>>,
}

impl FileIO {
    pub fn new(file_name: PathBuf) -> Result<Self> {
        match OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(file_name)
        {
            Ok(file) => {
                return Ok(FileIO {
                    fd: Arc::new(RwLock::new(file)),
                });
            }
            Err(e) => {
                error!("failed to open data file: {}", e);
                return Err(Errors::FailedToOpenDataFile);
            }
        }
    }
}

impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let read_guard = self.fd.read();
        match read_guard.read_at(buf, offset) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("read form data file err: {}", e);
                return Err(Errors::FailedReadFromDatFile);
            }
        };
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut write_guard = self.fd.write();
        match write_guard.write(buf) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("write to data file err : {}", e);
                return Err(Errors::FailedWriteToDataFile);
            }
        }
    }

    fn sync(&self) -> Result<()> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("failed to sync data file: {}", e);
            return Err(Errors::FailedSyncDataFile);
        }
        Ok(())
    }
}

#[cfg(test)]

mod test {
    use std::{fs, path::PathBuf};

    use super::*;

    #[test]
    fn test_file_io_write() {
        let path = PathBuf::from("/tmp/a.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());

        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("key1".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(4, res1.ok().unwrap());

        let res2 = fio.write("key2".as_bytes());
        assert!(res2.is_ok());
        assert_eq!(4, res2.ok().unwrap());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }

    #[test]
    fn test_io_read() {
        let path = PathBuf::from("/tmp/a.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());

        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("key1".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(4, res1.ok().unwrap());

        let res2 = fio.write("key2".as_bytes());
        assert!(res2.is_ok());
        assert_eq!(4, res2.ok().unwrap());

        let mut buf1 = [0u8; 5];
        let read_res1 = fio.read(&mut buf1, 0);
        assert!(read_res1.is_ok());
        assert_eq!(5, read_res1.ok().unwrap());

        let mut buf2 = [0u8; 5];
        let read_res2 = fio.read(&mut buf2, 0);
        assert!(read_res2.is_ok());
        assert_eq!(5, read_res2.ok().unwrap());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }

    #[test]
    fn test_file_io_sync() {
        let path = PathBuf::from("/tmp/a.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());

        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("key1".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(4, res1.ok().unwrap());

        let res2 = fio.write("key2".as_bytes());
        assert!(res2.is_ok());
        assert_eq!(4, res2.ok().unwrap());

        let sync_res = fio.sync();
        assert!(sync_res.is_ok());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }
}
