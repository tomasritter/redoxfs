extern crate block_encrypt;

use syscall::error::{Error, Result, EIO};

use disk::Disk;
use self::block_encrypt::BlockEncrypt;

macro_rules! try_disk {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Disk I/O Error: {}", err);
            return Err(Error::new(EIO));
        }
    })
}

pub struct DiskEncrypted {
    pub encr : BlockEncrypt
}

impl DiskEncrypted {
    pub fn open(path: &str) -> Result<DiskEncrypted> {
        println!("Open DiskEncrypted {} ", path);
        let encr = try_disk!(BlockEncrypt::open(path));
        Ok(DiskEncrypted {
            encr
        })
    }

    pub fn create(path: &str, size: u64) -> Result<DiskEncrypted> {
        println!("Create DiskFile {}", path);
        let encr = try_disk!(BlockEncrypt::create(path, size));
        Ok(DiskEncrypted {
            encr
        })
    }
}

impl Disk for DiskEncrypted {
    fn read_at(&mut self, block: u64, buffer: &mut [u8]) -> Result<usize> {
        println!("DiskEncypted read at {}", block);
        let count = try_disk!(self.encr.read_at(block, buffer));
        Ok(count)
    }

    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<usize> {
        println!("DiskEncrypted write at {}", block);
        let count = try_disk!(self.encr.write_at(block, buffer));
        Ok(count)
    }

    fn size(&mut self) -> Result<u64> {
        let size = try_disk!(self.encr.size());
        Ok(size)
    }
}
