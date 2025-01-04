use core::f64;

use color::Color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use indicatif::ProgressBar;
use interval::Interval;
use log::info;
use ray::Ray;
use shpere::Sphere;
use vec3::{Point3, Vec3};

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod shpere;
mod vec3;

fn ray_color(r: Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    colog::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }
    //World

    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let bar = ProgressBar::new((image_width * image_height) as u64);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = ray::Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r, &world);
            color::write_color(pixel_color);

            bar.inc(1);
        }
    }
    bar.finish();
    info!("Done");
}
