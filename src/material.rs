use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn from(a: &Color) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction: Vec3 = rec.normal + Vec3::random_unit_vector();
        scattered = Ray::from(rec.p, scatter_direction);
        attenuation = self.albedo;
        true
    }
}
