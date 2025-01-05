use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use log::info;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    colog::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let samples_per_pixel = 10;

    let camera = Camera::new(image_width, aspect_ratio, samples_per_pixel);

    //World
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.00 / 1.33);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::<Lambertian>::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));

    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));

    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.), 0.5, material_right));

    camera.render(&world);
    info!("Done");
}
