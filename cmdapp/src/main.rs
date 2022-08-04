use vtracer::{convert_image_to_svg, Config};

fn main() {
    let config = Config::from_args();
    let result = convert_image_to_svg(config);
    match result {
        Ok(()) => {
            println!("Conversion successful.");
        }
        Err(msg) => {
            panic!("Conversion failed with error message: {}", msg);
        }
    }
}
