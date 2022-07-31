use ray_trace::types::ray::{HittableList, Ray, Sphere};
use ray_trace::types::v3::{Point3, V3};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;

    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5));
    world.add(Sphere::new(Point3::new([0.0, -100.5, -1.0]), 100.0));

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = V3::new([viewport_width, 0.0, 0.0]);
    let vertical = V3::new([0.0, viewport_height, 0.0]);
    let lower_left_corner =
        &origin - &horizontal / 2.0 - &vertical / 2.0 - V3::new([0.0, 0.0, focal_length]);

    println!("P3\n{} {} \n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        eprintln!("Remaining lines : {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                &origin,
                u * &horizontal + &lower_left_corner + v * &vertical - &origin,
            );
            let pixel_color = r.ray_color(&world);
            pixel_color.write_color();
        }
    }
    eprintln!("Done");
}
