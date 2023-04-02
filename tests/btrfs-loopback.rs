use clone_file::*;
use std::fs;

const BASE_DIR: &'static str = env!("CARGO_MANIFEST_DIR");

fn ensure_test_sh() -> anyhow::Result<()> {
    use anyhow::Context;
    fs::metadata(format!("{BASE_DIR}/.btrfs-mnt")).context("please run tests with test.sh")?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn clone_file_works() -> anyhow::Result<()> {
    ensure_test_sh()?;

    clone_file(
        format!("{BASE_DIR}/.btrfs-mnt/src.bin"),
        format!("{BASE_DIR}/.btrfs-mnt/dest.bin"),
    )?;

    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn clone_file_range_works() -> anyhow::Result<()> {
    ensure_test_sh()?;

    clone_file_range(
        format!("{BASE_DIR}/.btrfs-mnt/src.bin"),
        // chopping 16K from start and 16K from end
        16 << 10,
        (50 << 20) - (16 << 10) * 2,
        format!("{BASE_DIR}/.btrfs-mnt/dest-part.bin"),
        0,
    )?;

    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn clone_range_consistent_with_copy_range() -> anyhow::Result<()> {
    ensure_test_sh()?;

    clone_file_range(
        format!("{BASE_DIR}/.btrfs-mnt/src.bin"),
        0,
        16 << 10,
        format!("{BASE_DIR}/.btrfs-mnt/clone-range-small.bin"),
        0,
    )?;

    copy_file_range(
        format!("{BASE_DIR}/.btrfs-mnt/src.bin"),
        0,
        16 << 10,
        format!("{BASE_DIR}/.btrfs-mnt/copy-range-small.bin"),
        0,
    )?;

    Ok(())
}
