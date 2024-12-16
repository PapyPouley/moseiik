#[cfg(target_arch = "aarch64")]
use crate::main::*;
#[cfg(target_arch = "aarch64")]
use image::RgbImage;

#[cfg(test)]
#[cfg(target_arch = "aarch64")]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn unit_test_l1_neon_identical_images() {
        // Crée deux images identiques (10x10 pixels)
        let mut im1 = RgbImage::new(10, 10);
        for pixel in im1.pixels_mut() {
            *pixel = image::Rgb([100, 150, 200]); // Remplit avec une valeur fixe
        }
        let im2 = im1.clone();

        // L1 norm entre des images identiques devrait être 0
        let result = unsafe { l1_neon(&im1, &im2) };
        assert_eq!(result, 0);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn unit_test_l1_neon_different_images() {
        // Crée deux images différentes (10x10 pixels)
        let mut im1 = RgbImage::new(10, 10);
        let mut im2 = RgbImage::new(10, 10);

        let im1_c: [u8; 3] = [100, 150, 200];
        let im2_c: [u8; 3] = [50, 100, 150];

        for pixel in im1.pixels_mut() {
            *pixel = image::Rgb(im1_c);
        }
        for pixel in im2.pixels_mut() {
            *pixel = image::Rgb(im2_c);
        }

        // calcule de la différence
        let expected_diff =
            ((im1_c[0] as i32 - im2_c[0] as i32).abs() * im1.width() as i32 * im1.height() as i32)
                + ((im1_c[1] as i32 - im2_c[1] as i32).abs()
                    * im1.width() as i32
                    * im1.height() as i32)
                + ((im1_c[2] as i32 - im2_c[2] as i32).abs()
                    * im1.width() as i32
                    * im1.height() as i32);

        let result = unsafe { l1_neon(&im1, &im2) };
        assert_eq!(result, expected_diff);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn unit_test_l1_neon_partial_difference() {
        // Crée deux images similaires mais avec quelques différences
        let im1 = RgbImage::new(10, 10);
        let mut im2 = im1.clone();

        // Change quelques pixels dans im2
        im2.put_pixel(0, 0, image::Rgb([255, 255, 255]));
        im2.put_pixel(1, 1, image::Rgb([0, 0, 0]));

        // Calcul manuel des différences pour ces deux pixels
        let expected_diff = (im2.get_pixel(0, 0)[0] as i32 - im1.get_pixel(0, 0)[0] as i32).abs()
            + (im2.get_pixel(0, 0)[1] as i32 - im1.get_pixel(0, 0)[1] as i32).abs()
            + (im2.get_pixel(0, 0)[2] as i32 - im1.get_pixel(0, 0)[2] as i32).abs();

        let result = unsafe { l1_neon(&im1, &im2) };
        assert_eq!(result, expected_diff);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn unit_test_l1_neon_empty_images() {
        // Crée deux images vides
        let im1 = RgbImage::new(10, 10);
        let im2 = RgbImage::new(10, 10);

        // L1 norm entre des images vides devrait être 0
        let result = unsafe { l1_neon(&im1, &im2) };
        assert_eq!(result, 0);
    }
}