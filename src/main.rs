use std::fs::File;
use std::io::Write;
use std::path::Path;
mod vec3;
use vec3::*;

#[allow(dead_code)]
fn some_vec3_test() {
    let test_vals: [f64; 3] = [45.0, 55.0, 56.0];
    let test_vec = Vec3::from(test_vals[0], test_vals[1], test_vals[2]);

    //x, y, z test
    assert_eq!(test_vals[0], test_vec.x());
    assert_eq!(test_vals[1], test_vec.y());
    assert_eq!(test_vals[2], test_vec.z());

    // Neg Test
    assert_eq!(-test_vals[0], -test_vec.x());
    assert_eq!(-test_vals[1], -test_vec.y());
    assert_eq!(-test_vals[2], -test_vec.z());

    // index test
    assert_eq!(test_vals[0], test_vec[0]);
    assert_eq!(test_vals[1], test_vec[1]);
    assert_eq!(test_vals[2], test_vec[2]);

    // add test
    let new_arr: [f64; 3] = [20.0, 34.0, 12.0];
    let new_vec = Vec3::from(new_arr[0], new_arr[1], new_arr[2]);
    let sum_vec = new_vec + test_vec;
    assert_eq!(new_vec[0] + test_vec[0], sum_vec.x());
    assert_eq!(test_vals[1], test_vec[1]);
    assert_eq!(test_vals[2], test_vec[2]);

    // mut index test
    let mut mut_vec = Vec3::from(new_arr[0], new_arr[1], new_arr[2]);
    mut_vec[2] = 40.0;
    assert_eq!(mut_vec[2], 40.0);

    // add assign test
    mut_vec += test_vec;
    assert_eq!(mut_vec[2], 96.0);

    // mul assign test
    mut_vec *= 56.0;
    assert_eq!(mut_vec[2], (96.0 * 56.0));

    // mul vec test
    let mul_vec = test_vec * new_vec;
    assert_eq!(test_vec[0] * new_vec[0], mul_vec[0])
}

#[allow(dead_code)]
fn first_image_print() {
    let image_width = 256;
    let image_height = 256;
    let path = Path::new("image.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()) {
        Err(why) => panic!("coun't write to {}: {}", display, why),
        Ok(_) => {}
    }

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let ir: i64 = (255.999 * r) as i64;
            let ig: i64 = (255.999 * g) as i64;
            let ib: i64 = (255.999 * b) as i64;
            let line = format!("{} {} {}\n", ir, ig, ib);

            match file.write(line.as_bytes()) {
                Err(why) => panic!("couldn't write to {}: {}", display, why),
                Ok(_) => {}
            }
        }
    }
}

fn write_color(file: &mut File, display: &std::path::Display, pixel_color: color) {
    let ir: i64 = (255.999 * pixel_color.x()) as i64;
    let ig: i64 = (255.999 * pixel_color.y()) as i64;
    let ib: i64 = (255.999 * pixel_color.z()) as i64;
    let line = format!("{} {} {}\n", ir, ig, ib);

    match file.write(line.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {}
    }
}

fn main() {
    let image_width = 256;
    let image_height = 256;
    let path = Path::new("image.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()) {
        Err(why) => panic!("coun't write to {}: {}", display, why),
        Ok(_) => {}
    }
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let pixel_color = color::from(r, g, b);
            write_color(&mut file, &display, pixel_color);
        }
    }
}
