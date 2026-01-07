use image::{DynamicImage, GenericImageView, Pixel, Rgb};

pub fn convert(image: &DynamicImage, target_width: u32) -> String {
    let mut result = String::new();
    let resized_image = resize_image(image, target_width);
    let pixels = resized_image.pixels();

    for (index, pixel) in pixels.into_iter().enumerate() {
        let rgb: Rgb<u8> = pixel.2.to_rgb();
        let luminosity = get_pixel_luminosity(&rgb);
        result.push(map_luminosity_to_char(luminosity));
        if (index + 1) % resized_image.width() as usize == 0 {
            result.push('\n');
        }
    }

    result
}

fn map_luminosity_to_char(luminosity: usize) -> char {
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let index: usize = (luminosity * (chars.len() - 1)) / 255;
    chars[index]
}

fn resize_image(image: &DynamicImage, target_width: u32) -> DynamicImage {
    let aspect_ratio = 0.5;
    let target_height = (target_width as f64 * (image.height() as f64 / image.width() as f64) * aspect_ratio) as u32;
    image.thumbnail(target_width, target_height)
}

fn get_pixel_luminosity(rgb: &Rgb<u8>) -> usize {
    (0.2126 * rgb.0[0] as f64 +
        0.7152 * rgb.0[1] as f64 +
        0.0722 * rgb.0[2] as f64) as usize
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
