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
    block_encrypt : BlockEncrypt
}

impl DiskEncrypted {
    pub fn open(path: &str, cipher : &str) -> Result<DiskEncrypted> {
        println!("Open DiskEncrypted {} ", path);
        let block_encrypt = try_disk!(BlockEncrypt::open(path, cipher));
        Ok(DiskEncrypted {
            block_encrypt
        })
    }

    pub fn create(path: &str, size: u64, cipher : &str) -> Result<DiskEncrypted> {
        println!("Create DiskFile {}", path);
        let block_encrypt = try_disk!(BlockEncrypt::create(path, size, cipher));
        Ok(DiskEncrypted {
            block_encrypt
        })
    }
}

impl Disk for DiskEncrypted {
    fn read_at(&mut self, block: u64, buffer: &mut [u8]) -> Result<usize> {
        let count = try_disk!(self.block_encrypt.read_at(block, buffer));
        Ok(count)
    }

    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<usize> {
        let count = try_disk!(self.block_encrypt.write_at(block, buffer));
        Ok(count)
    }

    fn size(&mut self) -> Result<u64> {
        let size = try_disk!(self.block_encrypt.size());
        Ok(size)
    }
}
