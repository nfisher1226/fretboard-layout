use clap::{crate_version, App, Arg};
mod draw;
mod fretboard;
mod plot;
use fretboard::Fret;

pub struct Specs {
    scale: f64,
    count: i32,
    multi: bool,
    scale_treble: f64,
    nut: f64,
    bridge: f64,
    pfret: usize,
    output: String,
    border: f64,
}


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
                .takes_value(true),
        )
        .arg(
            Arg::new("PERPENDICULAR")
                .about("Set which fret is perpendicular to the centerline")
                .short('p')
                .long("perpendicular")
                .default_value("8")
                .takes_value(true)
        )
        .arg(
            Arg::new("COUNT")
                .about("Total fret count")
                .short('c')
                .long("count")
                .default_value("24")
                .takes_value(true)
        )
        .arg(
            Arg::new("NUT")
                .about("Nut width")
                .short('n')
                .long("nut")
                .default_value("43")
                .takes_value(true)
        )
        .arg(
            Arg::new("BRIDGE")
                .about("Bridge spacing")
                .short('b')
                .long("bridge")
                .default_value("56")
                .takes_value(true)
        )
        .arg(
            Arg::new("OUTPUT")
                .about("Name of the output file")
                .short('o')
                .long("output")
                .default_value("output.svg")
                .takes_value(true)
        )
        .arg(
            Arg::new("BORDER")
                .about("Image border in mm")
                .short('B')
                .long("border")
                .default_value("10")
                .takes_value(true)
        )
        .get_matches();
    let multi = matches.is_present("MULTI");
    let scale_treble: f64 = if multi {
        matches.value_of_t("MULTI").unwrap()
    } else {
        matches.value_of_t("SCALE").unwrap()
    };
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
    };
    let fretboard: Vec<Fret> = fretboard::Fret::get_fretboard(&specs);
    let factors = plot::Factors::get_factors(&fretboard, &specs);
    draw::create_document(&specs, &factors, &fretboard);
    println!("Output saved as {}.", specs.output);
}
