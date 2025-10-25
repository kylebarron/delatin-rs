// mod integration;

use crate::Triangulation;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[test]
fn main() {
    let file = File::open(Path::new("./fixtures/delatin_rs.json")).unwrap();
    let heights: Vec<f64> = serde_json::from_reader(file).unwrap();

    let mut delatin = Triangulation::try_new(&heights, 512, 512).unwrap();
    let (points, triangles) = delatin.run(0.2).unwrap();

    assert_eq!(points.len(), 16257);
    assert_eq!(triangles.len(), 32147);

    let flat_points: Vec<usize> = points.iter().flat_map(|p| vec![p.0, p.1]).collect();
    let flat_triangles: Vec<usize> = triangles.iter().flat_map(|t| vec![t.0, t.1, t.2]).collect();

    dbg!(flat_points.len());
    dbg!(flat_triangles.len());

    let output = TinOutput {
        coords: flat_points,
        triangles: flat_triangles,
    };
    let out = File::create(Path::new("./fixtures/tin_from_rust.json")).unwrap();
    serde_json::to_writer(out, &output).unwrap();
}

#[derive(Deserialize, Serialize)]
struct TinOutput {
    coords: Vec<usize>,
    triangles: Vec<usize>,
}

fn load_output_from_js() -> TinOutput {
    let file = File::open(Path::new("./fixtures/delatin_rs_tin.json")).unwrap();
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
