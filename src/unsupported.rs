use std::env;
use std::io;
use std::path::Path;

/// This function is not implemented for this platform
pub fn clone_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q) -> io::Result<()> {
    operation_not_supported()
}

/// This function is not implemented for this platform
pub fn clone_file_range<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    src_offset: u64,
    src_length: u64,
    dest: Q,
    dest_offset: u64,
) -> io::Result<()> {
    operation_not_supported()
}

fn operation_not_supported() -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!(
            "Operation not supported on {}-{}-{}",
            env::consts::ARCH,
            env::consts::OS,
            env::consts::FAMILY
        ),
    ))
}
