
use image::{DynamicImage, ImageBuffer, Rgb};


pub struct ImageProcessor;

pub enum ImageRotationTheta {
    Theta90 = 90,
    Theta180 = 180,
    Theta270 = 270
}

pub trait ImageProcessorTrait {
    fn blur(img: DynamicImage, radius: f32) -> DynamicImage;
    fn brighten(img: DynamicImage, brightness: i32) -> DynamicImage;
    fn crop(img: DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage;
    fn rotate(img: DynamicImage, rotation: ImageRotationTheta) -> DynamicImage;
    fn invert(img: &mut DynamicImage);
    fn grayscale(img: DynamicImage) -> DynamicImage;
    fn fractal() -> ImageBuffer<Rgb<u8>, Vec<u8>>;
}

impl ImageProcessorTrait for ImageProcessor {
    fn blur(img: DynamicImage, radius: f32) -> DynamicImage {
        img.blur(radius)
    }

    fn brighten(img: DynamicImage, brightness: i32) -> DynamicImage {
        img.brighten(brightness)
    }

    fn crop(img: DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        if x + width >= img.width() {
            panic!("invalid arguments: x - width > image width: {} > {}", x + width, img.width());
        }
        if y + height >= img.height() {
            panic!("invalid arguments: y - height > image height: {} > {}", y + height, img.height());
        }
        img.crop_imm(x, y, width, height)
    }

    fn rotate(img: DynamicImage, rotation: ImageRotationTheta) -> DynamicImage {
        match rotation {
            ImageRotationTheta::Theta90 => img.rotate90(),
            ImageRotationTheta::Theta180 => img.rotate180(),
            ImageRotationTheta::Theta270 => img.rotate270()
        }
    }

    fn invert(img: &mut DynamicImage){
        img.invert();
    }

    fn grayscale(img: DynamicImage) -> DynamicImage{
        img.grayscale()
    }

    // (unused from cli) This code was adapted from https://github.com/PistonDevelopers/image
    fn fractal() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let width = 800;
        let height = 800;
    
        let mut imgbuf = image::ImageBuffer::new(width, height);
    
        let scale_x = 3.0 / width as f32;
        let scale_y = 3.0 / height as f32;

        let x_to_red = 255.0 / width as f32;
        let y_to_blue = 255.0 / height as f32;
    
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // Use red and blue to be a pretty gradient background
            let red = (x_to_red * x as f32) as u8;
            let blue = (y_to_blue * y as f32) as u8;
    
            // Use green as the fractal foreground (here is the fractal math part)
            let cx = y as f32 * scale_x - 1.5;
            let cy = x as f32 * scale_y - 1.5;
    
            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);
    
            let mut green = 0;
            while green < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                green += 1;
            }
    
            // Actually set the pixel. red, green, and blue are u8 values!
            *pixel = image::Rgb([red, green, blue]);
        }
    
        imgbuf
    }
}