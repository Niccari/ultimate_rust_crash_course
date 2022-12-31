use image::DynamicImage;

pub struct ImageRepository;

pub trait ImageRepositoryTrait {
    fn load(infile: String) -> DynamicImage;
    fn save(img: DynamicImage, outfile: String);
}


impl ImageRepositoryTrait for ImageRepository {
    fn load(infile: String) -> DynamicImage{
       return match image::open(infile) {
            Ok(image) => image,
            Err(error) => panic!("Problem opening the image: {:?}", error)
        };
    }

    fn save(img: DynamicImage, outfile: String) {
        match img.save(outfile) {
            Ok(image) => image,
            Err(error) => panic!("Problem saving the image: {:?}", error)
        };
    }
}