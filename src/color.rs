use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(color: Color) {
    let ir = 255.999 * color.x();
    let ig = 255.999 * color.y();
    let ib = 255.999 * color.z();

    println!("{} {} {}", ir, ig, ib)
}
