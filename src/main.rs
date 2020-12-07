#![warn(clippy::all, clippy::pedantic)]
use clap::{crate_version, App, Arg};
mod fretboard;
mod run;
use run::Specs;

fn main() {
    let matches = App::new("fblt")
        .version(crate_version!())
        .author("The JeanG3nie <jeang3nie@hitchhiker-linux.org>")
        .about("Generates layout dimensions for a stringed instrument fretboard.")
        .arg(
            Arg::new("SCALE")
                .about("Scale length in mm.")
                .default_value("648")
                .takes_value(true),
        )
        .arg(
            Arg::new("MULTI")
                .about("Creates a multiscale fretboard with <MULTI> as the treble scale.")
                .short('m')
                .long("multi")
                .default_value("610")
                .takes_value(true),
        )
        .arg(
            Arg::new("PERPENDICULAR")
                .about("Set which fret is perpendicular to the centerline")
                .short('p')
                .long("perpendicular")
                .default_value("8")
                .takes_value(true),
        )
        .arg(
            Arg::new("COUNT")
                .about("Total fret count")
                .short('c')
                .long("count")
                .default_value("24")
                .takes_value(true),
        )
        .arg(
            Arg::new("NUT")
                .about("Nut width")
                .short('n')
                .long("nut")
                .default_value("43")
                .takes_value(true),
        )
        .arg(
            Arg::new("BRIDGE")
                .about("Bridge spacing")
                .short('b')
                .long("bridge")
                .default_value("56")
                .takes_value(true),
        )
        .arg(
            Arg::new("OUTPUT")
                .about("Name of the output file")
                .short('o')
                .long("output")
                .default_value("output.svg")
                .takes_value(true),
        )
        .arg(
            Arg::new("BORDER")
                .about("Image border in mm")
                .short('B')
                .long("border")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::new("EXTERN")
                .about("Open output file in external program")
                .short('e')
                .long("external")
                .default_value("inkscape")
                .takes_value(true),
        )
        .get_matches();
    let multi = matches.occurrences_of("MULTI") > 0;
    let scale_treble: f64 = if multi {
        matches.value_of_t("MULTI").unwrap()
    } else {
        matches.value_of_t("SCALE").unwrap()
    };
    let cmd = matches.value_of("EXTERN").unwrap().to_string();
    let bridge: f64 = matches.value_of_t("BRIDGE").unwrap();
    let specs = Specs {
        scale: matches.value_of_t("SCALE").unwrap(),
        count: matches.value_of_t("COUNT").unwrap(),
        multi,
        scale_treble,
        nut: matches.value_of_t("NUT").unwrap(),
        bridge: bridge + 6.0,
        pfret: matches.value_of_t("PERPENDICULAR").unwrap(),
        output: matches.value_of("OUTPUT").unwrap().to_string(),
        border: matches.value_of_t("BORDER").unwrap(),
        external: matches.occurrences_of("EXTERN") > 0,
        cmd,
    };
    specs.run();
}
