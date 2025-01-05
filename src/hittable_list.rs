use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add<'a, T: Hittable + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: crate::ray::Ray, ray_t: Interval) -> Option<HitRecord> {
        // let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        let mut ret = None;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                ret = Some(rec);
            }
        }

        ret
    }
}
