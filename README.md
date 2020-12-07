# fblt
## Introduction
This is the FretBoard Lookup Tool (fblt). Given a set of parameters fbl will
calculate the fret, nut, and bridge locations and output an svg template suitable
for using as a pattern for a fretted stringed instrument. Multiscale designs are
supported. Measurements are currently expected to be in millimeters.
## Building
Just run ```cargo build --release``` to build a release binary in target/release.
## Usage
The first argument is the scale length in millimeters. If the -m, or --multiscale
argument is also given, this becomes the bass side scale length.
#### Flags
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
* -e, --external - open the output file in an external program
## Future Development
There are plans to refine the calculations in regards to the nut and bridge width
for multiscale designs, as the current method is not properly skewed in relation
to the centerline, but instead places the bridge termination equidistant from the
centerline on bass and treble sides rather than being proportional to the two
scale lengths. This is considered a very minor inaccuracy, however.

Currently, when given incorrect input the program panics. There is a need for
proper, more idiomatic error handling. This is higher priority.

A gui in pyqt5 is planned but not yet implemented. An inkscape plugin is possible
but not currently planned.
