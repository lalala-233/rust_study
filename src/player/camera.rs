use std::ops::Deref;

use crate::Coord;
type Radius = Coord;
#[derive(Debug, PartialEq, Eq)]
pub struct Camera {
    radius: Radius,
}
impl Camera {
    //public
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            radius: Radius::new(x, y),
        }
    }
    pub fn set(&mut self, x: usize, y: usize) {
        self.radius = Radius::new(x, y)
    }
}
impl Deref for Camera {
    type Target = Radius;
    fn deref(&self) -> &Self::Target {
        &self.radius
    }
}
#[cfg(test)]
pub mod public {
    use crate::player::camera::{public::default::default, Camera};
    pub mod default {
        use crate::player::camera::Camera;
        use rand::{thread_rng, Rng};
        pub fn default() -> (Camera, usize, usize) {
            let (x, y) = (
                thread_rng().gen_range(11..114),
                thread_rng().gen_range(19..191),
            );
            let camera = Camera::new(x, y);
            (camera, x, y)
        }
    }
    #[test]
    pub fn deref() {
        let (camera, x, y) = default();
        assert_eq!(*camera, Camera::new(x, y).radius);
    }
    #[test]
    pub fn eq() {
        let (camera, x, y) = default();
        assert_eq!(camera, Camera::new(x, y));
    }
    #[test]
    pub fn new() {
        for _ in 0..10000 {
            let (camera, x, y) = default();
            assert_eq!(camera, Camera::new(x, y));
        }
    }
    #[test]
    pub fn set() {
        let (mut camera, mut x, mut y) = default();
        assert_eq!(Camera::new(x, y), camera);
        x += 1;
        y += 1;
        camera.set(x, y);
        assert_eq!(Camera::new(x, y), camera);
    }
}
