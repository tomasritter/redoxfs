use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use syscall::error::{Error, Result, EIO};

use BLOCK_SIZE;
use disk::Disk;

macro_rules! try_disk {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Disk I/O Error: {}", err);
            return Err(Error::new(EIO));
        }
    })
}

pub struct DiskFile {
    pub file: File
}

impl DiskFile {
    pub fn open(path: &str) -> Result<DiskFile> {
        println!("Open DiskFile {} ", path);
        let file = try_disk!(OpenOptions::new().read(true).write(true).open(path));
        Ok(DiskFile {
            file: file
        })
    }

    pub fn create(path: &str, size: u64) -> Result<DiskFile> {
        println!("Create DiskFile {}", path);
        let file = try_disk!(OpenOptions::new().read(true).write(true).create(true).open(path));
        try_disk!(file.set_len(size));
        Ok(DiskFile {
            file: file
        })
    }
}

impl Disk for DiskFile {
    fn read_at(&mut self, block: u64, buffer: &mut [u8]) -> Result<usize> {
        //println!("Disk file read at {}", block);
        try_disk!(self.file.seek(SeekFrom::Start(block * BLOCK_SIZE)));
        let count = try_disk!(self.file.read(buffer));
        Ok(count)
    }

    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<usize> {
        //println!("Disk file write at {}", block);
        try_disk!(self.file.seek(SeekFrom::Start(block * BLOCK_SIZE)));
        let count = try_disk!(self.file.write(buffer));
        Ok(count)
    }

    fn size(&mut self) -> Result<u64> {
        let size = try_disk!(self.file.seek(SeekFrom::End(0)));
        Ok(size)
    }
}
