use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

pub fn clone_or_copy_file<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dest: Q,
) -> io::Result<Option<u64>> {
    if let Ok(_) = crate::clone_file(&src, &dest) {
        Ok(None)
    } else {
        fs::copy(&src, &dest).and_then(|x| Ok(Some(x)))
    }
}

pub fn clone_or_copy_file_range<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    src_offset: u64,
    src_length: u64,
    dest: Q,
    dest_offset: u64,
) -> io::Result<Option<u64>> {
    if let Ok(_) = crate::clone_file_range(&src, src_offset, src_length, &dest, dest_offset) {
        Ok(None)
    } else {
        todo!()
    }
}

pub fn copy_file_range<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    src_offset: u64,
    src_length: u64,
    dest: Q,
    dest_offset: u64,
) -> io::Result<u64> {
    let mut src_file = File::open(src)?;
    let mut dest_file = File::options().write(true).create(true).open(dest)?;

    src_file.seek(SeekFrom::Start(src_offset))?;
    let mut src_file = src_file.take(src_length);

    dest_file.seek(SeekFrom::Start(dest_offset))?;

    io::copy(&mut src_file, &mut dest_file)
}
