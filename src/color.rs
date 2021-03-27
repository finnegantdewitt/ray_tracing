use crate::vec3::*;
use std::fs::File;
use std::io::Write;

pub fn write_color(file: &mut File, pixel_color: Color, samples_per_pixel: i32) {
    // let ir: i64 = (255.999 * pixel_color.x()) as i64;
    // let ig: i64 = (255.999 * pixel_color.y()) as i64;
    // let ib: i64 = (255.999 * pixel_color.z()) as i64;
    // let line = format!("{} {} {}\n", ir, ig, ib);

    // match file.write(line.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => {}
    // }

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    let ir: i64 = (255.999 * r) as i64;
    let ig: i64 = (255.999 * g) as i64;
    let ib: i64 = (255.999 * b) as i64;
    let line = format!("{} {} {}\n", ir, ig, ib);

    match file.write(line.as_bytes()) {
        Err(why) => panic!("couldn't write!: {}", why),
        Ok(_) => {}
    }
}
