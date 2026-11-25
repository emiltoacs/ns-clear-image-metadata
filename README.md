## Description

Clean all metadata of the image(picture) file without changing file mtime.

A simple Rust program that once compiled can be copied to the nautilus script directory:
1. It takes as input all the currently selected file in nautilus, use the environment variable NAUTILUS_SCRIPT_SELECTED_FILE_PATHS
2. For each file if its an image in the sens of picture, clear all the metadata of the file
3. Preserve the mtime of each file so store the mtime before changing the metadata and restore it as it was before the metadata clearance

## Dependencies

The program needs `exiftool` to manipulate the metadata.

Debian:
``` sh
sudo apt install libimage-exiftool-perl
```

Arch
``` sh
sudo pacman -S perl-image-exiftool
```

## Install the binary to your nautilus script directory

cp target/release/ns-clear-image-metadata ~/.local/share/nautilus/scripts/
