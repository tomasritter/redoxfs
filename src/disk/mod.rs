use syscall::error::Result;

pub use self::cache::DiskCache;
pub use self::file::DiskFile;
pub use self::sparse::DiskSparse;
pub use self::encrypted::DiskEncrypted;

mod cache;
mod file;
mod sparse;
mod encrypted;

/// A disk
pub trait Disk {
    fn read_at(&mut self, block: u64, buffer: &mut [u8]) -> Result<usize>;
    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<usize>;
    fn size(&mut self) -> Result<u64>;
}

