use crate::Triangulation;
use image::ImageReader;
use std::path::Path;

fn parse_png<P: AsRef<Path>>(path: P) -> Vec<f64> {
    let img = ImageReader::open(path).unwrap().decode().unwrap().to_rgb8();

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

/// Reads a file of raw little-endian u32s into a Vec<u32>.
fn read_u32_file<P: AsRef<Path>>(path: P) -> Vec<u32> {
    let bytes = std::fs::read(path).unwrap();
    assert!(
        bytes.len().is_multiple_of(size_of::<u32>()),
        "File length not multiple of 4"
    );

    let num_elems = bytes.len() / size_of::<u32>();
    let mut out = Vec::with_capacity(num_elems);

    // SAFETY: we read u32s from properly sized byte chunks
    for chunk in bytes.chunks_exact(size_of::<u32>()) {
        out.push(u32::from_le_bytes(chunk.try_into().unwrap()));
    }

    out
}

#[test]
fn main() {
    let heights = parse_png("./fixtures/14-2625-6369.png");

    let mut delatin = Triangulation::try_new(&heights, 512, 512).unwrap();
    delatin.run(0.2).unwrap();

    // TODO: make coords flat
    let coords = delatin.coords();
    let triangles = delatin.triangles();

    assert_eq!(coords.len(), 16257);
    assert_eq!(triangles.len() / 3, 32147);

    let flat_coords: Vec<u32> = coords
        .iter()
        .flat_map(|p| vec![p.0 as u32, p.1 as u32])
        .collect();
    let flat_triangles: Vec<u32> = triangles.iter().map(|v| *v as u32).collect();

    let expected_coords = read_u32_file("./fixtures/coords_from_js.bin");
    let expected_triangles = read_u32_file("./fixtures/triangles_from_js.bin");

    // Assert exactly the same as JS output
    assert_eq!(flat_coords, expected_coords);
    assert_eq!(flat_triangles, expected_triangles);
}
