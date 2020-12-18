# Gfret
## Introduction
This is a fretBoard layout tool. Given a set of parameters gfret will
calculate the fret, nut, and bridge locations and output an svg template suitable
for using as a pattern for a fretted stringed instrument. Multiscale designs are
supported. Measurements are currently expected to be in millimeters.
## Building
You will need a Rust toolchain installed, including cargo. Gtk+3x is also
required. To build the program, run ```cargo build --release``` to build a
release binary in target/release.

Alternatively, you can use the included Makefile to build and install the
program, adjusting the installation path with the PREFIX and DESTDIR variables.

## Usage
By default the Gtk based gui will run. If desired, the program can be run in
standalone cli mode by using the ```cli``` subcommand. The first argument to the
cli is the scale length in millimeters. If the -m, or --multiscale argument is
also given, this becomes the bass side scale length.
#### Cli Flags
* -m, --multiscale <scale> - creates a multiscale design where [scale] is
the treble side scale.
* -c, --count <count> - the total number of frets to plot.
* -p, --perpendicular <fret> - which fret is to be perpendicular to the centerline.
* -b, --bridge <spacing> - string spacing at the bridge. This is usually given as
the actual string spacing, while the nut is given as the physical width of the
nut. Therefore, this number is padded by 6mm to give approximately 3mm overhang
of the fretboard in relation to the two outer strings.
* -n, --nut <width> - the width of the nut. On multiscale designs this will be an
approximation, as the nut is slanted.
* -B, --border <width> - the border to be placed around the completed image.
* -o, --output <file> - the name of the output file.
* -e, --external <program> - open the output file in an external program
## Future Development
Currently, when given incorrect input the program panics. There is a need for
proper, more idiomatic error handling. This is higher priority.

For the gui, it would be nice to save state and allow loading specs from and saving
to templates.
