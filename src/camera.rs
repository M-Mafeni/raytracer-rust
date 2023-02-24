use crate::{vector::{Point3, Vec3, vec3, point3, utility::{unit_vector, cross_product}}, ray::{Ray, ray}, utility::math::degrees_to_radians};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

pub fn cam(
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    vfov: f64, // vertical field-of-view in degrees
    aspect_ratio: f64
) -> Camera {
    let theta = degrees_to_radians(vfov);
    let h = (theta/2.0).tan();
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    // forward vector
    let w = unit_vector(look_from - look_at);
    // right vector
    let u = unit_vector(cross_product(vup, w));
    // up vector
    let v = cross_product(w, u);
    
    let origin = look_from;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical
    }
}

pub fn initialise_camera() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3(0.0, 0.0, 0.0);
    let horizontal = point3(viewport_width, 0.0, 0.0);
    let vertical = point3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);
    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        ray(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
