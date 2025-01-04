
use camera::Camera;
use hittable_list::HittableList;
use log::info;
use sphere::Sphere;
use vec3::Point3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
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

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    camera.render(&world);
    info!("Done");
}
