#!/usr/bin/env bash

set -E

setup() {
  # volume and file sizes are specifically chosen so that naive copy will not have enough space
  fallocate -l 200M .btrfs.img
  mkfs.btrfs -f .btrfs.img

  mkdir -p .btrfs-mnt
  sudo mount -o loop .btrfs.img .btrfs-mnt
  sudo chmod a+rwx .btrfs-mnt

  cat /dev/random | head -c 50M > .btrfs-mnt/src.bin

  # chopping 16K from start and 16K from end
  cat .btrfs-mnt/src.bin | tail -c +16K | tail -c +2 | head -c -16K > .btrfs-mnt/src-part.bin

}

test() {
  cargo test

  diff -q .btrfs-mnt/src.bin .btrfs-mnt/dest.bin
  diff -q .btrfs-mnt/src-part.bin .btrfs-mnt/dest-part.bin
  diff -q .btrfs-mnt/clone-range-small.bin .btrfs-mnt/copy-range-small.bin
}

teardown() {
  code=$?
  trap - ERR

  sudo umount .btrfs-mnt
  rm -r .btrfs-mnt
  rm .btrfs.img

  exit $code
}

setup
trap teardown ERR
test
teardown
