use mirage::image_processor::ImageProcessor;
use mirage::image_processor::ImageProcessorTrait;
use mirage::image_processor::ImageRotationTheta;
use mirage::image_repository::ImageRepository;
use mirage::image_repository::ImageRepositoryTrait;

enum Command {
    Blur,
    Brighten,
    Crop,
    Rotate,
    Invert,
    Grayscale
}

type CommandParams = Vec<(Command, Vec<String>)>;

fn parse_args (mut args: Vec<String>, commands: &mut CommandParams) {
    let mut arguments: Vec<String> = vec![];
    args.reverse();
    for arg in args {
        let command: Option<Command> = match arg.as_str() {
            "blur" => Some(Command::Blur),
            "brighten" => Some(Command::Brighten),
            "crop" => Some(Command::Crop),
            "rotate" => Some(Command::Rotate),
            "invert" => Some(Command::Invert),
            "grayscale" => Some(Command::Grayscale),
            _ => None
        };
        match command {
            Some(cmd) => {
                arguments.reverse();
                commands.push((cmd, arguments));
                arguments = vec![];
            },
            None => {
                arguments.push(arg);
            }
        };
    }
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args.len() < 3 {
        print_usage_and_exit();
    }
    let infile = args.remove(0);
    let outfile = args.remove(0);
    let mut commands: CommandParams = vec![];
    parse_args(args, &mut commands);

    let mut img = ImageRepository::load(infile);
    for (command, args) in commands {
        match command {
            Command::Blur => {
                if args.len() != 1 {
                    print_usage_and_exit();
                }
                let radius: f32 = args[0].parse().expect("cannot parse: radius");
                img = ImageProcessor::blur(img, radius);
            }
            Command::Brighten => {
                if args.len() != 1 {
                    print_usage_and_exit();
                }
                let brightness: i32 = args[0].parse().expect("cannot parse: brightness");
                img = ImageProcessor::brighten(img, brightness);
            }
            Command::Crop => {
                if args.len() != 4 {
                    print_usage_and_exit();
                }
                let x: u32 = args[0].parse().expect("cannot parse x");
                let y: u32 = args[1].parse().expect("cannot parse y");
                let width: u32 = args[2].parse().expect("cannot parse width");
                let height: u32 = args[3].parse().expect("cannot parse height");
                img = ImageProcessor::crop(img, x, y, width, height);
            }
            Command::Rotate => {
                if args.len() != 1 {
                    print_usage_and_exit();
                }
                let rotate: u32 = args[0].parse().expect("cannot parse angle");
                let rotate = match rotate {
                    90 => ImageRotationTheta::Theta90,
                    180 => ImageRotationTheta::Theta180,
                    270 => ImageRotationTheta::Theta90,
                    _ => panic!("invalid angle, should be either of 90, 180 or 270 - specified: {}", rotate)
                };
                img = ImageProcessor::rotate(img, rotate);
            }
            Command::Invert => {
                ImageProcessor::invert(&mut img);
            }
            Command::Grayscale => {
                img = ImageProcessor::grayscale(img);
            }
        }
    }
    ImageRepository::save(img, outfile);
}

fn print_usage_and_exit() {
    println!("USAGE: cargo run INFILE OUTFILE [command1 command1_args...] [command2 command2_args...] ...");
    println!("** Stackable **");
    println!("- blur radius(f32)");
    println!("- brighten brightness(i32)");
    println!("- crop x(u32) y(u32) width(u32) height(u32)");
    println!("- rotate angle(90 | 180 | 270)");
    println!("- invert");
    println!("- grayscale");

    std::process::exit(-1);
}
