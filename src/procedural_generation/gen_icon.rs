use bevy::prelude::*;

use image::{ImageBuffer, Rgba};
use noise::{NoiseFn, Perlin};
use palette::{rgb::Rgb, Srgb};
use rand::{rng, RngCore};

use crate::stellar_core::solar_system::Planet;

fn normalize(value: f64, min: f64, max: f64) -> f64 {
    (value - min).max(0.0).min(max - min) / (max - min)
}

fn palette(c: usize) -> Srgb<u8> {
    let p: Vec<Srgb<u8>> = vec![
        Rgb::new(0, 0, 0),       // 0 black
        Rgb::new(0, 30, 70),     // 1 gas
        Rgb::new(0, 120, 200),   // 2 water
        Rgb::new(100, 100, 100), // 3 rocky
        Rgb::new(180, 60, 20),   // 4 lava
        Rgb::new(255, 255, 255), // 5 ice
        Rgb::new(255, 200, 0),   // 6 hot rocky
        Rgb::new(255, 100, 0),   // 7 molten
        Rgb::new(0, 100, 0),     // 8 bio
        Rgb::new(60, 200, 60),   // 9 veg
        Rgb::new(150, 255, 150), //10 gas envelope
        Rgb::new(150, 0, 150),   //11 metallic
        Rgb::new(80, 70, 70),    //12 sub‑ocean
        Rgb::new(200, 0, 0),     //13 rings
        Rgb::new(0, 200, 200),   //14 clouds
        Rgb::new(170, 170, 170), //15 neutral
    ];

    p[c]
}

pub fn render_and_write_icon(planet: &Planet, size: u32, path: &std::path::Path) {
    write_icon(&render_icon(planet, size), path); 
}

pub fn render_icon(planet: &Planet, size: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let perlin = Perlin::new(rng().next_u32());

    //normalize values
    let _mass_n     = normalize(planet.mass, 0.00, 10.0);
    let dens_n     = normalize(planet.density, 2000.0, 8000.0);
    let temp_n     = normalize(planet.surface_temperature, 50.0, 400.0);
    let atm_n      = normalize(planet.atmos_pressure, 0.0, 10.0);
    let mag_n      = normalize(planet.magnetic_field_strength, 0.0, 1.0);
    let hab_n      = planet.habitability.clamp(0.0, 1.0);

    //determine color
    let base_index: usize; //default rocky
    if dens_n < 0.3 {
        base_index = if temp_n < 0.2 { 5 } else { 1 }; //gas or ice
    } else if temp_n > 0.7 {
        base_index = if dens_n > 0.6 { 4 } else { 6 }; //lava or hot rocky
    } else if temp_n < 0.2 {
        base_index = 5; //ice
    } else {
        base_index = 3; //normal rock
    }

    let has_rings = mag_n > 0.5 && dens_n < 0.5;

    let cloud_opacity = (atm_n * 255.0) as u8; // 0–255

    let mut imgbuf = 
        ImageBuffer::from_pixel(size, size, Rgba([0, 0, 0, 0]));

    //center of planet in pixel coords
    let center_x = size as f64 * 0.5;
    let center_y = size as f64 * 0.5;
    let sphere_radius = (size as f64) * 0.3; //70 % of the image width

    for y in 0..size {
        for x in 0..size {

            let dx = (x as f64 - center_x) / sphere_radius;
            let dy = (y as f64 - center_y) / sphere_radius;

            //does the point lie on the projected sphere?
            let dist2 = dx * dx + dy * dy;
            if dist2 > 1.0 {
                //outside the planet so keep transparent
                continue;
            }

            //calculate depth of point on the sphere
            let z = (1.0 - dist2).sqrt();

            let mut color = palette(base_index);

            //terrain variation (perlin)
            let n = perlin.get([x as f64 * 0.1, y as f64 * 0.1, 1.0]) as f64;
            let terrain_shade = (n + 2.0) / 0.5; // 0.5 .. 1.0
            color.red   = ((color.red   as f64) * terrain_shade).min(255.0) as u8;
            color.green = ((color.green as f64) * terrain_shade).min(255.0) as u8;
            color.blue  = ((color.blue  as f64) * terrain_shade).min(255.0) as u8;

            if n > 0.2 {
                //make highlands a little lighter
                color.red = ((color.red as f32) * 1.1).min(255.0) as u8;
                color.green = ((color.green as f32) * 1.1).min(255.0) as u8;
                color.blue = ((color.blue as f32) * 1.1).min(255.0) as u8;
            }

            //add clouds on top
            if n > 0.5 {
                let cloud = palette(0);
                let blend = cloud_opacity as f32 / 255.0;
                color.red = ((1.0 - blend) * color.red as f32 + blend * cloud.red as f32) as u8;
                color.green = ((1.0 - blend) * color.green as f32 + blend * cloud.green as f32) as u8;
                color.blue = ((1.0 - blend) * color.blue as f32 + blend * cloud.blue as f32) as u8;
            }

            //perspective/illumination shading (simple Lambertian)
            let shade = (z + 1.0) / 2.0; // 0.5 .. 1.0
            color.red   = ((color.red   as f64) * shade).min(255.0) as u8;
            color.green = ((color.green as f64) * shade).min(255.0) as u8;
            color.blue  = ((color.blue  as f64) * shade).min(255.0) as u8;

            //write pixel
            imgbuf.put_pixel(x, y, Rgba([color.red, color.green, color.blue, 255]));
        }
    }

    //draw rings
    if has_rings {
        let ring_radius = sphere_radius * 1.35;
        let ring_thickness = sphere_radius * 0.08;
        for y in 0..size {
            for x in 0..size {
                let dx = x as f64 - center_x;
                let dy = y as f64 - center_y;
                let dist = (dx * dx + dy * dy).sqrt();
                if (ring_radius - ring_thickness / 2.0) <= dist && dist <= (ring_radius + ring_thickness / 2.0) {
                    let ring_col = palette(14); // red
                    imgbuf.put_pixel(x, y, Rgba([ring_col.red, ring_col.green, ring_col.blue, 255]));
                }
            }
        }
    }

    if false {
        //hab icon
        let dot_radius: i32 = 4;
        let hab_x = dot_radius as i32;
        let hab_y = size as i32 - hab_x - 1;

        let hab_i = (hab_n * 10.0) as i32;

        let hc: Srgb<u8> = match hab_i {
            0..6 => Rgb::new(200, 25, 0),
            6..9 => Rgb::new(200, 200, 0),
            9..11 => Rgb::new(25, 200, 0),
            _ => palette(0)
        };

        for y in 0..size {
            for x in 0..size {
                let dist = (x as i32 - hab_x).pow(2) + (y as i32 - hab_y).pow(2);
                if dist <= dot_radius.pow(2)  {
                    imgbuf.put_pixel(x, y, Rgba([hc.red, hc.green, hc.blue, 255]));
                }
            }
        }
    }

    imgbuf
}

pub fn write_icon(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, path: &std::path::Path) {
    img.save(path).unwrap();
}

pub fn circle_texture(
    width: u32, 
    height: u32, 
    images: &mut ResMut<Assets<Image>>, 
    r: u8, g: u8, b: u8, a: u8
) -> Handle<Image> {

    let pix_width = match width {
        x if x < 1 => 1,
        x => x
    };
    let pix_height = match height {
        x if x < 1 => 1,
        x => x
    };

    //create an image buffer
    let mut imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = 
        image::ImageBuffer::new(pix_width, pix_height);

    //calculate center and radius of circle
    let radius = pix_width as f32 / 2.0;
    let c_x = pix_width as f32 / 2.0;
    let c_y = pix_height as f32 / 2.0;

    //iterate thru all pixels
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        //get squared distance from the center
        let distance_squared = (x as f32 - c_x).powi(2) + (y as f32 - c_y).powi(2);

        //if it falls in this small range, then color it whatever color
        if distance_squared >= (radius - 1.0).powi(2) && distance_squared <= radius.powi(2) {
            *pixel = image::Rgba([r, g, b, a]);
        }
    }

    //convert the image::Image into bevy_image::image::Image via image::DynamicImage
    let img = Image::from_dynamic(
        image::DynamicImage::ImageRgba8(imgbuf),
        false,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD
    );
    //lastly add it to the thing 
    let img_handle = images.add(img);

    //to get the thing
    return img_handle;
}

pub fn image_to_handle(
    imgbuf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    images: &mut ResMut<Assets<Image>>,
) -> Handle<Image> {

    let img = Image::from_dynamic(
        image::DynamicImage::ImageRgba8(imgbuf),
        false,
        bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD
    );
    
    images.add(img)
}