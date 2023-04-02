# clone-file

A simple way to use your filesystem's [reflink] features.

[reflink]: https://en.wikipedia.org/wiki/Data_deduplication#reflink

```rust
// Clone a file using a reflink, or error if it can not be done.
clone_file("src.bin", "dest.bin");

// Try to clone a file, falling back to a regular copy.
clone_or_copy_file("src.bin", "dest.bin");

// Clone a sub-range of a file using a reflink, or error if it can not be done.
clone_file_range(
  "src.bin",
  /* offset: */ 4 << 10,
  /* length: */ 2 << 20,
  "dest.bin",
  /* dest offset: */ 0
);

// Try to clone a sub-range of a file, falling back to a naive copy.
clone_or_copy_file_range(
  "src.bin",
  /* offset: */ 4 << 10,
  /* length: */ 2 << 20,
  "dest.bin",
  /* dest offset: */ 0
);
```

## Implementation details

### Linux

On Linux, `FICLONE` and `FICLONERANGE` ioctls are used. Please refer to [`man 2 ioctl_ficlonerange`] for details and limitations.

Tested with the `btrfs` filesystem.

[`man 2 ioctl_ficlonerange`]: https://man7.org/linux/man-pages/man2/ioctl_ficlonerange.2.html

### Others

The `clone_file` and `clone_file_range` functions are currently not implemented for other platforms.

However, the fallback functions `clone_or_copy_file` and `clone_or_copy_file_range` will work, falling back to naive copies.

## Running tests

To test the cloning, we need a filesystem that supports reflinks. This requires a bit of a setup, which is implemented in the `test.sh` script.

It expects a Linux system, with a `btrfs-progs` packages installed. It creates a 200MiB loopback device, formats it into `btrfs`, then creates the neccessary test data and runs `cargo tests`. It then cleans up the loopback and the mount.

The tests are intentionally set up to run only through `test.sh`
