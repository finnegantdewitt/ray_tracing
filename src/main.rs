#![allow(dead_code)]
use std::fs::File;
use std::io::Write;
use std::path::Path;
mod vec3;
use vec3::*;
mod ray;
use ray::*;

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

fn test_ray() {
    let test_ray = Ray::from(&Point3::from(3.0, 4.0, 5.0), &Vec3::from(6.0, 7.0, 8.0));
    println!("{:?}", test_ray);
}

fn write_color(file: &mut File, display: &std::path::Display, pixel_color: Color) {
    let ir: i64 = (255.999 * pixel_color.x()) as i64;
    let ig: i64 = (255.999 * pixel_color.y()) as i64;
    let ib: i64 = (255.999 * pixel_color.z()) as i64;
    let line = format!("{} {} {}\n", ir, ig, ib);

    match file.write(line.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => {}
    }
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::from(0.0, 0.0, 0.0);
    let horizontal = Vec3::from(viewport_width, 0.0, 0.0);
    let vertical = Vec3::from(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::from(0.0, 0.0, focal_length);

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
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::from(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r);
            write_color(&mut file, &display, pixel_color);
        }
    }
}
