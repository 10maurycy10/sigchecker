# sigchecker

A simple rust program to detect files where the file extension and contents don't match.

For example if you rename a mpv file ".png" it will be detected.

## Example usage, with usb drive mounted at /mnt/usb``

``sudo find /mnt/usb | cargo run --release | tee log``

- replace ``cargo run --release`` with ``sudo ./target/release/sigchecker`` if it hangs.

## Requirements

Requires libmagic, It likely wont work on windows without CygWin or WSL.

## Fixing False positives

If your terminal is spammed with false positives, add the reported file type and extension ("" if no extension) to db.json. Then rerun the program.

Contributions welcome.
