
## Maker Dump
**Executable:** `makerdump`

This program attempts to create a hexdump of exif makernotes, and outputs it to the specified file.
<!-- This program finds the exif subifd at 0x8769 and then finds the makernotes at 0x927, and then hexdumps the data into the specified file -->

**Usage:**
```sh
makerdump <input_file> --dump <output_file>
```
If `--dump` is ommitted then the output will be printed to the terminal

## Xmp
**Executable:** `xmpdump`

This program attempt to find xmp in a file and dumps it into the specified file
**Usage:**
```sh
xmpdump <image_file> <output_file>
```

## Mata Audio
**Executable:** `mataaudio`

This program tests metadata parsers for audio formats: MP3, FLAC, WAV, OGG, Opus, AAC, AIFF, APE, MPC, WavPack, DSF, and Audible.

**Usage:**
```sh
mataaudio <input> <output_file>
```

**Input options:**
- File path: A single audio file
- Directory path: All audio files in the directory
- `MUSIC`: Special value that points to the default music directory

