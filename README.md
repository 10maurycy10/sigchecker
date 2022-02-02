# sigchecker

A simple rust program to detect mislabled files.

## Example usege, with usb drive mounted at ``/mnt/usb``

``sudo find /mnt/usb | cargo run --release | tee log``

- replace ``cargo run --release`` with ``sudo ./target/release/sigchecker`` if it hangs.

## Requirements

Requires libmagic, It likley wont work on windows without cygwin or wsl.

## Fixing Flase positives

If your terminal is spammed with false positives, add the reported file type and extention ("" if no extention) to db.json. Then rerun the program
