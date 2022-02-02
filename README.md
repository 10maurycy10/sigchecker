# sigchecker

A simple rust program to detect mislabled files.

## Example usege, with usb drive mounted at ``/mnt/usb``

``find /mnt/usb | cargo run | tee log``

## Requirements

Requires libmagic, It likley wont work on windows without cygwin or wsl.

## Fixing Flase positives

If your terminal is spammed with false positives, add the reported file type and extention ("" if no extention) to db.json. Then rerun the program
