use ray_trace::types::ray::{random_float, Camera, HittableList, Sphere};
use ray_trace::types::v3::{Color, Point3};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: u64 = 100;

    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new([0.0, 0.0, -1.0]), 0.5));
    world.add(Sphere::new(Point3::new([0.0, -100.5, -1.0]), 100.0));

    let camera = Camera::new();

    println!("P3\n{} {} \n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        eprintln!("Remaining lines : {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_float()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_float()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + r.ray_color(&world);
            }
            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}
