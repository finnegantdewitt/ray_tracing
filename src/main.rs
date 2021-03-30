#![allow(dead_code)]
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;
mod vec3;
use vec3::*;
mod ray;
use ray::*;
mod hittable;
use hittable::*;
mod sphere;
use sphere::*;
mod hittable_list;
use hittable_list::*;
mod utils;
use utils::*;
mod camera;
use camera::*;
mod color;
use color::*;
mod material;
use material::*;

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

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let rec = &mut HitRecord::void();

    if depth <= 0 {
        return Color::from(0., 0., 0.);
    }

    if world.hit(r, 0.001, f64::INFINITY, rec) {
        //let target: Point3 = rec.p + rec.normal + Point3::random_unit_vector(); // og diffuse formula (ch8.2)
        let target: Point3 = rec.p + Point3::random_in_hemisphere(&rec.normal); // new diffuse formula (ch8.25)

        return 0.5 * ray_color(&Ray::from(&rec.p, &(target - rec.p)), world, depth - 1);
    }

    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let cam: Camera = Camera::new();

    let path = Path::new("image.ppm");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create!: {}", why),
        Ok(file) => file,
    };

    match file.write(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes()) {
        Err(why) => panic!("coun't write to: {}", why),
        Ok(_) => {}
    }
    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color: Color = Color::from(0., 0., 0.);

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double(0., 1.)) / (image_width - 1) as f64;
                let v = (j as f64 + random_double(0., 1.)) / (image_height - 1) as f64;
                let r: Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&mut file, pixel_color, samples_per_pixel);
        }
    }
}
