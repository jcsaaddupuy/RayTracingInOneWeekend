use color::Color;
use indicatif::ProgressBar;
use log::info;
use ray::Ray;
use vec3::{Point3, Vec3};

mod color;
mod ray;
mod vec3;

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> f64 {
    let oc = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - f64::sqrt(discriminant)) / (2.0 * a);
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);

    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);

    ((1.0 - a) * Color::new(1.0, 1.0, 1.0)) + (a * Color::new(0.5, 0.7, 1.0))
}

fn main() {
    colog::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 256;

    // Calculate the image height, and ensure that it's at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

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

            let pixel_color = ray_color(r);
            color::write_color(pixel_color);

            bar.inc(1);
        }
    }
    bar.finish();
    info!("Done");
}
