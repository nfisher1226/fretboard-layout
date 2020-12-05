use crate::fretboard::Fret;
use crate::plot::{self, Factors, Line};
use crate::Specs;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

fn draw_centerline(specs: &Specs) -> svg::node::element::Path {
    let start_x = specs.border;
    let start_y = (specs.bridge / 2.0) + specs.border;
    let end_x = specs.border + specs.scale;
    let end_y = (specs.bridge / 2.0) + specs.border;
    let data = Data::new()
        .move_to((start_x, start_y))
        .line_to((end_x, end_y))
        .close();
    let centerline = Path::new()
        .set("fill", "none")
        .set("stroke", "blue")
        .set("stroke-dasharray", "4.0,4.0")
        .set("stroke-dashoffset", "0")
        .set("stroke-width", 1)
        .set("id", "Centerline")
        .set("d", data);
    centerline
}

fn draw_fretboard(fretboard: &Vec<Fret>, factors: &Factors, specs: &Specs) -> svg::node::element::Path {
    let nut = plot::Line::get_fret_line(&fretboard, &(0 as usize), &factors, &specs);
    let end = plot::Line::get_fret_line(&fretboard, &(specs.count as usize + 1), &factors, &specs);
    let data = Data::new()
        .move_to((nut.start.0, nut.start.1))
        .line_to((nut.end.0, nut.end.1))
        .line_to((end.end.0, end.end.1))
        .line_to((end.start.0, end.start.1))
        .line_to((nut.start.0, nut.start.1))
        .close();
    let board = Path::new()
        .set("fill", "none")
        .set("stroke", "grey")
        .set("stroke-width", 1)
        .set("id", "Fretboard")
        .set("d", data);
    board
}

fn draw_bridge(specs: &Specs, factors: &Factors) -> svg::node::element::Path {
    let start_x = specs.border;
    let start_y = specs.border;
    let end_x = specs.border + factors.treble_offset;
    let end_y = specs.border + specs.bridge;
    let data = Data::new()
        .move_to((start_x, start_y))
        .line_to((end_x, end_y))
        .close();
    let bridge = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("id", "Bridge")
        .set("d", data);
    bridge
}

fn draw_fret(fret: &i32, line: &Line) -> svg::node::element::Path {
    let id = if *fret == 0 {
        "Nut".to_string()
    } else {
        format!("Fret {}", fret)
    };
    let data = Data::new()
        .move_to((line.start.0, line.start.1))
        .line_to((line.end.0, line.end.1))
        .close();
    let fret = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("id", id)
        .set("d", data);
    fret
}

pub fn create_document(specs: &Specs, factors: &Factors, fretboard: &Vec<Fret>) {
    let width = (specs.border * 2.0) + specs.scale;
    let widthmm = format!("{}mm", width);
    let height = (specs.border * 2.0) + specs.bridge;
    let heightmm = format!("{}mm", height);
    let mut document = Document::new()
        .set("width", widthmm)
        .set("height", heightmm)
        .set("viewBox", (0, 0, width, height));
    document = document.add(draw_centerline(&specs));
    document = document.add(draw_fretboard(&fretboard, &factors, &specs));
    document = document.add(draw_bridge(&specs, &factors));
    for fret in 0..specs.count + 1 {
        let line = plot::Line::get_fret_line(&fretboard, &(fret as usize), &factors, &specs);
        document = document.add(draw_fret(&fret, &line));
    }
    svg::save(&specs.output, &document).unwrap();
}
