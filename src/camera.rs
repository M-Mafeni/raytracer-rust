use crate::{vector::{Point3, Vec3, utility::{unit_vector, cross_product}, random::random_in_unit_disk}, ray::{Ray, ray}, utility::math::degrees_to_radians};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    // Vectors describing camera's position
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

pub fn cam(
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    vfov: f64, // vertical field-of-view in degrees
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64
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
    let horizontal = focus_dist * viewport_width * u;
    let vertical = focus_dist * viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
    let lens_radius = aperture / 2.0;
    Camera {
        origin,
        lower_left_corner,
        horizontal,
        vertical,
        u,
        v,
        w,
        lens_radius
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
    
        ray(
            self.origin + offset,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset
        )
    }
}
