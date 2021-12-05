mod attribute;
mod classfile;
mod constantpool;
mod fieldinfo;
mod methodinfo;

use std::io::Read;

use crate::classfile::ClassFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = std::env::args().skip(1).next().unwrap();

    let mut file = std::fs::File::open(file_name)?;
    let mut buf = Vec::with_capacity(1 << 16);
    file.read_to_end(&mut buf)?;

    let (buf, class_file) = ClassFile::parse_class_file(&buf).unwrap();
    dbg!(&class_file.methods);

    Ok(())
}
