use ustar::{RamDisk, TarFileSystem, metadata::Metadata};

static TAR: &[u8] = include_bytes!("../test.tar");

fn main() {
    let ramdisk = Box::new(RamDisk::from(TAR));

    let filesys = TarFileSystem::new((*ramdisk).data.len(), ramdisk);
    let mut buffer = [0; 6];
    filesys.load("./test/test_1.txt", &mut buffer);



    println!("Data: {:?}", buffer);
    println!("Contents: {}", String::from_utf8(buffer.to_vec()).expect("Parsing Failed"));
}
