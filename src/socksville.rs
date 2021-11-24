use std::io::prelude::*;
use std::net::{ToSocketAddrs};
use std::sync::{Arc, RwLock};

use anyhow::Error;
use wasi_common::WasiFile;

pub struct Socksville {
    stm: Arc<RwLock<std::net::TcpStream>>,  // TODO: cf. cap_std::net::TcpStream
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
