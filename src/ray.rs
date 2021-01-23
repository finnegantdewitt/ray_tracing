use crate::vec3::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn from(origin: &point3, direction: &Vec3) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
        }
    }

    pub fn origin(&self) -> point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> point3 {
        self.orig + t * self.dir
    }
}
