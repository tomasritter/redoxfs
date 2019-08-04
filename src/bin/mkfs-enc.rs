extern crate redoxfs;
extern crate uuid;

use std::{env, fs, process, time};
use std::io::Read;

use redoxfs::{FileSystem, DiskEncrypted};
use uuid::Uuid;

fn main() {
    let mut args = env::args().skip(1);

    let disk_path = if let Some(path) = args.next() {
        path
    } else {
        println!("redoxfs-mkfs-enc: no disk image provided");
        println!("redoxfs-mkfs-enc DISK [CIPHER] [BOOTLOADER]");
        process::exit(1);
    };

    let cipher = match args.next() {
        Some(c) => c,
        None => {
            println!("redoxfs-mkfs-enc: failed to read cipher");
            process::exit(1);
        }
    };

    let bootloader_path_opt = args.next();

    let disk = match DiskEncrypted::open(&disk_path, &cipher) {
        Ok(disk) => disk,
        Err(err) => {
            println!("redoxfs-mkfs-enc: failed to open image {}: {}", disk_path, err);
            process::exit(1);
        }
    };

    let mut bootloader = vec![];
    if let Some(bootloader_path) = bootloader_path_opt {
        match fs::File::open(&bootloader_path) {
            Ok(mut file) => match file.read_to_end(&mut bootloader) {
                Ok(_) => (),
                Err(err) => {
                    println!("redoxfs-mkfs-enc: failed to read bootloader {}: {}", bootloader_path, err);
                    process::exit(1);
                }
            },
            Err(err) => {
                println!("redoxfs-mkfs-enc: failed to open bootloader {}: {}", bootloader_path, err);
                process::exit(1);
            }
        }
    };

    let ctime = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
    match FileSystem::create_reserved(disk, &bootloader, ctime.as_secs(), ctime.subsec_nanos()) {
        Ok(filesystem) => {
            let uuid = Uuid::from_bytes(&filesystem.header.1.uuid).unwrap();
            println!("redoxfs-mkfs-enc: created filesystem on {}, reserved {} blocks, size {} MB, uuid {}", disk_path, filesystem.block, filesystem.header.1.size/1000/1000, uuid.hyphenated());
        },
        Err(err) => {
            println!("redoxfs-mkfs-enc: failed to create filesystem on {}: {}", disk_path, err);
            process::exit(1);
        }
    }
}
