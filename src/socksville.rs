use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{ToSocketAddrs};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use anyhow::Error;
use wasi_common::{WasiDir, WasiFile};

#[derive(Clone)]
pub struct Socksville {
    stm: Arc<RwLock<std::net::TcpStream>>,  // TODO: cf. cap_std::net::TcpStream
}

pub struct SocksvillePlusPlus {
    name_to_addr: HashMap<String, String>,
}

impl Socksville {
    pub fn new(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {
        let stm = Arc::new(RwLock::new(std::net::TcpStream::connect(addr)?));
        Ok(Self { stm })
    }

    fn borrow(&self) -> std::sync::RwLockWriteGuard<std::net::TcpStream> {
        RwLock::write(&self.stm).unwrap()
    }
}

#[async_trait::async_trait]
impl WasiFile for Socksville {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    async fn datasync(&self) -> Result<(), Error> {
        Ok(())
    }
    async fn sync(&self) -> Result<(), Error> {
        Ok(())
    }
    async fn get_filetype(&self) -> Result<wasi_common::file::FileType, Error> {
        Ok(wasi_common::file::FileType::SocketStream)
    }
    async fn get_fdflags(&self) -> Result<wasi_common::file::FdFlags, Error> {
        Ok(wasi_common::file::FdFlags::APPEND)
    }
    async fn set_fdflags(&mut self, _flags: wasi_common::file::FdFlags) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn get_filestat(&self) -> Result<wasi_common::file::Filestat, Error> {
        Ok(wasi_common::file::Filestat {
            device_id: 0,
            inode: 0,
            filetype: self.get_filetype().await?,
            nlink: 0,
            size: 0, // XXX no way to get a size out of a Write :(
            atim: None,
            mtim: None,
            ctim: None,
        })

    }
    async fn set_filestat_size(&self, _size: u64) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn advise(&self, _offset: u64, _len: u64, _advice: wasi_common::file::Advice) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn allocate(&self, _offset: u64, _len: u64) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn set_times(
        &self,
        _atime: Option<wasi_common::SystemTimeSpec>,
        _mtime: Option<wasi_common::SystemTimeSpec>,
    ) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn read_vectored<'a>(&self, bufs: &mut [std::io::IoSliceMut<'a>]) -> Result<u64, Error> {
        let mut tot = 0;
        for buf in bufs {
            tot += self.borrow().read(buf)?;
        }
        Ok(tot.try_into()?)
    }
    async fn read_vectored_at<'a>(
        &self,
        _bufs: &mut [std::io::IoSliceMut<'a>],
        _offset: u64,
    ) -> Result<u64, Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn write_vectored<'a>(&self, bufs: &[std::io::IoSlice<'a>]) -> Result<u64, Error> {
        let mut tot = 0;
        for buf in bufs {
            tot += self.borrow().write(buf)?;
        }
        Ok(tot.try_into()?)
    }
    async fn write_vectored_at<'a>(
        &self,
        _bufs: &[std::io::IoSlice<'a>],
        _offset: u64,
    ) -> Result<u64, Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn seek(&self, _pos: std::io::SeekFrom) -> Result<u64, Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn peek(&self, _buf: &mut [u8]) -> Result<u64, Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn num_ready_bytes(&self) -> Result<u64, Error> {
        Ok(0)
    }

    async fn readable(&self) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
    async fn writable(&self) -> Result<(), Error> {
        Err(anyhow::anyhow!("no don't"))
    }
}

impl SocksvillePlusPlus {
    pub fn single(name: impl Into<String>, addr: impl Into<String>) -> Self {
        Self {
            name_to_addr: HashMap::from_iter(vec![(name.into(), addr.into())]),
        }
    }
}

#[async_trait::async_trait]
impl WasiDir for SocksvillePlusPlus {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn open_file(
        &self,
        symlink_follow: bool,
        path: &str,
        oflags: wasi_common::file::OFlags,
        read: bool,
        write: bool,
        fdflags: wasi_common::file::FdFlags,
    ) -> Result<Box<dyn WasiFile>, Error> {
        println!("tried to open {} for r:{}/w:{}", path, read, write);
        match self.name_to_addr.get(path) {
            Some(addr) => Ok(Box::new(Socksville::new(addr)?)),
            None => Err(anyhow::anyhow!("File {} does not exist", path)),
        }
        // if path == "socko" {
        //     Ok(Box::new(self.socks[0].clone()))
        // } else {
        //     Err(anyhow::anyhow!("File {} does not exist", path))
        // }
    }

    async fn open_dir(&self, symlink_follow: bool, path: &str) -> Result<Box<dyn WasiDir>, Error> {
        todo!()
    }

    async fn create_dir(&self, path: &str) -> Result<(), Error> {
        todo!()
    }
    async fn readdir(
        &self,
        cursor: wasi_common::dir::ReaddirCursor,
    ) -> Result<Box<dyn Iterator<Item = Result<wasi_common::dir::ReaddirEntity, Error>> + Send>, Error> {
        todo!()
    }

    async fn symlink(&self, old_path: &str, new_path: &str) -> Result<(), Error> {
        todo!()
    }

    async fn remove_dir(&self, path: &str) -> Result<(), Error> {
        todo!()
    }

    async fn unlink_file(&self, path: &str) -> Result<(), Error> {
        todo!()
    }

    async fn read_link(&self, path: &str) -> Result<PathBuf, Error> {
        todo!()
    }

    async fn get_filestat(&self) -> Result<wasi_common::file::Filestat, Error> {
        todo!()
    }

    async fn get_path_filestat(&self, path: &str, follow_symlinks: bool)
        -> Result<wasi_common::file::Filestat, Error> {
        todo!()
    }

    async fn rename(
        &self,
        path: &str,
        dest_dir: &dyn WasiDir,
        dest_path: &str,
    ) -> Result<(), Error> {
        todo!()
    }

    async fn hard_link(
        &self,
        path: &str,
        target_dir: &dyn WasiDir,
        target_path: &str,
    ) -> Result<(), Error> {
        todo!()
    }

    async fn set_times(
        &self,
        path: &str,
        atime: Option<wasi_common::SystemTimeSpec>,
        mtime: Option<wasi_common::SystemTimeSpec>,
        follow_symlinks: bool,
    ) -> Result<(), Error> {
        todo!()
    }
}
