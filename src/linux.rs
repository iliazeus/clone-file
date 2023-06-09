use std::fs::File;
use std::io;
use std::os::fd::AsRawFd;
use std::path::Path;

/// Clone a file using the FICLONE syscall
///
/// This is mainly tested on the `btrfs` filesystem.
///
/// For the details on the limitations of this method, see [`man 2 ioctl_ficlonerange`].
///
/// [`man 2 ioctl_ficlonerange`]: https://man7.org/linux/man-pages/man2/ioctl_ficlonerange.2.html
pub fn clone_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dest: Q) -> io::Result<()> {
    let src_file = File::open(src)?;
    let dest_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dest)?;

    match unsafe { ioctl::ficlone(dest_file.as_raw_fd(), src_file.as_raw_fd() as u64) } {
        Ok(_) => Ok(()),
        Err(_) => Err(io::Error::last_os_error()),
    }
}

/// Clone a range from a file using the FICLONERANGE syscall
///
/// This is mainly tested on the `btrfs` filesystem.
///
/// For the details on the limitations of this method, see [`man 2 ioctl_ficlonerange`].
///
/// One of the more common limitations is that `src_offset`, `src_length` and `dest_offset`
/// must be multiples of the filesystem block size. Expect this function to fail if that is not the case.
///
/// [`man 2 ioctl_ficlonerange`]: https://man7.org/linux/man-pages/man2/ioctl_ficlonerange.2.html
pub fn clone_file_range<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    src_offset: u64,
    src_length: u64,
    dest: Q,
    dest_offset: u64,
) -> io::Result<()> {
    let src_file = File::open(src)?;
    let dest_file = File::options().write(true).create(true).open(dest)?;

    let args = ioctl::FileCloneRange {
        src_fd: src_file.as_raw_fd() as i64,
        src_offset,
        src_length,
        dest_offset,
    };

    match unsafe { ioctl::ficlonerange(dest_file.as_raw_fd(), &args) } {
        Ok(_) => Ok(()),
        Err(_) => Err(io::Error::last_os_error()),
    }
}

mod ioctl {
    pub struct FileCloneRange {
        pub src_fd: i64,
        pub src_offset: u64,
        pub src_length: u64,
        pub dest_offset: u64,
    }

    nix::ioctl_write_int!(ficlone, 0x94, 9);
    nix::ioctl_write_ptr!(ficlonerange, 0x94, 13, FileCloneRange);
}
