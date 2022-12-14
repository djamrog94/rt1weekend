use super::v3::{Color, Point3, V3};
use rand::Rng;

pub struct HitRecord {
    p: Point3,
    normal: V3,
    t: f64,
    front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: V3::default(),
            t: 0.0,
            front_face: true,
        }
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &V3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> Default for HittableList<T> {
    fn default() -> Self {
        Self { objects: vec![] }
    }
}

impl<T: Hittable> HittableList<T> {
    pub fn new(&self, obj: T) -> Self {
        Self { objects: vec![obj] }
    }

    pub fn add(&mut self, obj: T) {
        self.objects.push(obj);
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for item in self.objects.iter() {
            if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        // update rec
        if hit_anything {
            rec.front_face = temp_rec.front_face;
            rec.p = temp_rec.p;
            rec.normal = temp_rec.normal;
            rec.t = temp_rec.t;
        }

        hit_anything
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        x
    }
}

pub fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_float_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub struct Camera {
    origin: Point3,
    horizontal: V3,
    vertical: V3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn new() -> Self {
        let mut temp = Self {
            origin: Point3::new([0.0, 0.0, 0.0]),
            horizontal: V3::new([VIEWPORT_WIDTH, 0.0, 0.0]),
            vertical: V3::new([0.0, VIEWPORT_HEIGHT, 0.0]),
            lower_left_corner: Point3::default(),
        };

        temp.lower_left_corner = &temp.origin
            - &temp.horizontal / 2.0
            - &temp.vertical / 2.0
            - V3::new([0.0, 0.0, FOCAL_LENGTH]);
        temp
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            u * &self.horizontal + &self.lower_left_corner + v * &self.vertical - &self.origin,
        )
    }
}

#[derive(Debug)]
pub struct Ray {
    origin: Point3,
    direction: V3,
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point3::default(),
            radius: 1.0,
        }
    }
}

impl Sphere {
    pub fn new(c: Point3, r: f64) -> Self {
        Self {
            center: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        true
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point3::default(),
            direction: V3::default(),
        }
    }
}

impl Ray {
    pub fn new(or: &Point3, dir: V3) -> Self {
        Self {
            origin: or.clone(),
            direction: dir,
        }
    }

    fn origin(&self) -> &Point3 {
        &self.origin
    }

    fn direction(&self) -> &V3 {
        &self.direction
    }

    fn at(&self, t: f64) -> Point3 {
        let temp = t * &self.direction;
        temp + &self.origin
    }

    pub fn ray_color(&self, world: &dyn Hittable, depth: u32) -> Color {
        let mut hit_record = HitRecord::default();

        if depth <= 0 {
            return Color::default();
        }
        if world.hit(&self, 0.001, f64::INFINITY, &mut hit_record) {
            let target = &hit_record.p + &V3::random_in_hemisphere(&hit_record.normal);
            return Ray::new(&hit_record.p, target - &hit_record.p).ray_color(world, depth - 1)
                * 0.5;
        }
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new([1.0, 1.0, 1.0]) * (1.0 - t) + Color::new([0.5, 0.7, 1.0]) * t
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - center;
        let a = self.direction().length_squared();
        let half_b = oc.dot(self.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return -1.0;
        } else {
            (-half_b - f64::sqrt(discriminant)) / a
        }
    }
}
