pub mod core;
pub mod scene;
pub mod utils;

// use crate::core::rtengine::RTEngine;
// use utils::geometry::Point;

use glam::Vec3A;
use image;
use ndarray::Array2;

use crate::core::rtengine::Material;
use crate::core::rtengine::RTEngine;
use crate::core::rtengine::Sphere;

pub fn run_lib() {
    let width = 300;
    let height = 300;
    let screen = (-1., 1., 1., -1.);
    let mut pixels = Array2::<Vec3A>::default((width, height));

    let step_height: f32 = (screen.3 - screen.1) / (height as f32);
    let step_width: f32 = (screen.2 - screen.0) / (width as f32);
    for i in 0..height {
        let y: f32 = screen.1 + (i as f32) * step_height;
        for j in 0..width {
            let x: f32 = screen.0 + (j as f32) * step_width;
            pixels[[i, j]] = Vec3A::new(x, y, 0.);
        }
    }

    let red_sphere: Sphere = Sphere {
        center: Vec3A::new(-0.2, 0., -1.),
        radius: 0.7,
    };
    let violet_sphere: Sphere = Sphere {
        center: Vec3A::new(0.1, -0.3, 0.),
        radius: 0.1,
    };
    let green_sphere: Sphere = Sphere {
        center: Vec3A::new(-0.3, 0., 0.),
        radius: 0.15,
    };
    let plane: Sphere = Sphere {
        center: Vec3A::new(0., -9000., 0.),
        radius: 9000. - 0.7,
    };

    let red_material: Material = Material {
        ambiant: Vec3A::new(0.1, 0., 0.),
        diffuse: Vec3A::new(0.7, 0., 0.),
        specular: Vec3A::new(1., 1., 1.),
        shininess: 100.,
        reflection: 0.5,
    };
    let violet_material: Material = Material {
        ambiant: Vec3A::new(0.1, 0., 0.1),
        diffuse: Vec3A::new(0.7, 0., 0.7),
        specular: Vec3A::new(1., 1., 1.),
        shininess: 100.,
        reflection: 0.5,
    };
    let green_material: Material = Material {
        ambiant: Vec3A::new(0., 0.1, 0.),
        diffuse: Vec3A::new(0., 0.6, 0.),
        specular: Vec3A::new(1., 1., 1.),
        shininess: 100.,
        reflection: 0.5,
    };
    let plane_material: Material = Material {
        ambiant: Vec3A::new(0.1, 0.1, 0.1),
        diffuse: Vec3A::new(0.6, 0.6, 0.6),
        specular: Vec3A::new(1., 1., 1.),
        shininess: 100.,
        reflection: 0.5,
    };

    let all_objects: Vec<Sphere> = vec![red_sphere, violet_sphere, green_sphere, plane];
    let materials: Vec<Material> = vec![
        red_material,
        violet_material,
        green_material,
        plane_material,
    ];

    let mut rte = RTEngine {
        pos_camera: Vec3A::new(0., 0., 1.),
        pos_pixels: pixels,
        pos_light: Vec3A::new(5., 5., 5.),
        objects: all_objects,
        material: materials,
    };
    let pixels: Array2<Vec3A> = rte.path_tracing();
    let mut imgbuf = image::ImageBuffer::new(width as u32, height as u32);

    for ((x, y), pixel) in pixels.indexed_iter() {
        let pixel_on_img = imgbuf.get_pixel_mut(x as u32, y as u32);
        let image::Rgb(_data) = *pixel_on_img;
        let rgb = pixel.to_array();
        *pixel_on_img = image::Rgb([
            (rgb[0] * 255.) as u8,
            (rgb[1] * 255.) as u8,
            (rgb[2] * 255.) as u8,
        ]);
    }
    imgbuf.save("test.png").unwrap();
}
