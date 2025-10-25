// mod integration;

use crate::Triangulation;
use image::ImageReader;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

fn parse_png() -> Vec<f64> {
    let img = ImageReader::open("./fixtures/14-2625-6369.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let data = img.as_raw();
    let mut heights = Vec::with_capacity(data.len() / 3);
    for chunk in data.chunks(3) {
        let r = chunk[0] as f64;
        let g = chunk[1] as f64;
        let b = chunk[2] as f64;

        let h = ((r * 256.0 * 256.0 + g * 256.0 + b) / 10.0 - 10_000.0) * 10.0;
        let h = h.round() / 10.0;
        heights.push(h);
    }
    heights
}

#[test]
fn main() {
    let heights = parse_png();

    let mut delatin = Triangulation::try_new(&heights, 512, 512).unwrap();
    let (coords, triangles) = delatin.run(0.2).unwrap();

    assert_eq!(coords.len(), 16257);
    assert_eq!(triangles.len(), 32147);

    let flat_coords: Vec<u32> = coords
        .iter()
        .flat_map(|p| vec![p.0 as u32, p.1 as u32])
        .collect();
    let flat_triangles: Vec<u32> = triangles
        .iter()
        .flat_map(|t| vec![t.0 as u32, t.1 as u32, t.2 as u32])
        .collect();

    std::fs::write(
        "./fixtures/tin_coords_from_rust",
        bytemuck::cast_slice(&flat_coords),
    )
    .unwrap();
    std::fs::write(
        "./fixtures/tin_triangles_from_rust",
        bytemuck::cast_slice(&flat_triangles),
    )
    .unwrap();

    // dbg!(flat_points.len());
    // dbg!(flat_triangles.len());

    // let output = TinOutput {
    //     coords: flat_points,
    //     triangles: flat_triangles,
    // };
    // let out = File::create(Path::new("./fixtures/tin_from_rust.json")).unwrap();
    // serde_json::to_writer(out, &output).unwrap();
}

#[derive(Deserialize, Serialize)]
struct TinOutput {
    coords: Vec<usize>,
    triangles: Vec<usize>,
}

fn load_output_from_js() -> TinOutput {
    let file = File::open(Path::new("./fixtures/tin_from_js.json")).unwrap();
    serde_json::from_reader(file).unwrap()
}

#[test]
fn integration_with_js() {
    let file = File::open(Path::new("./fixtures/delatin_rs.json")).unwrap();
    let heights: Vec<f64> = serde_json::from_reader(file).unwrap();

    let mut delatin = Triangulation::try_new(&heights, 512, 512).unwrap();
    let (points, triangles) = delatin.run(0.2).unwrap();

    let js_output = load_output_from_js();

    let flat_points: Vec<usize> = points.iter().flat_map(|p| vec![p.0, p.1]).collect();
    let flat_triangles: Vec<usize> = triangles.iter().flat_map(|t| vec![t.0, t.1, t.2]).collect();

    assert_eq!(flat_points, js_output.coords);
    assert_eq!(flat_triangles, js_output.triangles);
}
