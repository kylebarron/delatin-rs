use crate::Triangulation;
use std::{fs::File, path::Path};

#[test]
fn main() {
    let file = File::open(Path::new("./fixtures/delatin_rs.json")).unwrap();
    let heights: Vec<f64> = serde_json::from_reader(file).unwrap();

    let mut delatin = Triangulation::try_new(&heights, 512, 512).unwrap();
    let (points, triangles) = delatin.run(0.2).unwrap();

    assert_eq!(points.len(), 16257);
    assert_eq!(triangles.len(), 32147);
}
