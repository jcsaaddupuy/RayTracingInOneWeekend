use crate::interval::Interval;
use crate::vec3::Vec3;
pub type Color = Vec3;

pub fn write_color(color: Color) {
    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let ir = 255.999 * intensity.clamp(color.x());
    let ig = 255.999 * intensity.clamp(color.y());
    let ib = 255.999 * intensity.clamp(color.z());

    println!("{} {} {}", ir, ig, ib)
}
