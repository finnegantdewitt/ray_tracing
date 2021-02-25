use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub struct HittableList {
    pub objects: Vec<Hittable>,
}

impl HittableList {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    fn from(objects: Vec<Hittable>) -> Self {
        Self { objects }
    }
    fn clear(&self) {
        self.objects = Vec::new();
    }
    fn add(&self, object: &Hittable) {
        objects.push(*object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool {
        let mut temp_rec = HitRecord::void();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects {
            if (object.hit(r, t_min, closest_so_far, &mut temp_rec)) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        hit_anything
    }
}
