
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn at(self, t: f64) -> Point3 {
        // return self.orig + (self.dir * t);
        self.orig + (t * self.dir)
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn origin(self) -> Point3 {
        self.orig
    }
}
