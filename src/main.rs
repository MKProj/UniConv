use mkproj_lib::uniconv::*;
use structopt::StructOpt;
#[derive(StructOpt)]
#[structopt(name = "UniConv - A Command Line Unit Converter")]

struct Cli {
    //#[structopt(subcommand)]
    //flag: Option<Flag>,
    mode: String,
    unit: f32,
    conversion: String,
}

fn main() {
    let mut conv_unit: f32 = 0.0;
    let args = Cli::from_args();
    let mode = std::env::args().nth(1).unwrap();
    let unit = std::env::args().nth(2).unwrap();
    let conv = std::env::args().nth(3).unwrap();

    // First Temperature
    if mode == String::from("temp") {
        let u1: f32 = unit.trim().parse().expect("Expected valid float unit");
        if conv == String::from("c2f") {
            conv_unit = Temperature::cels_to_fahr(u1, conv_unit);
            println!("{} degrees Celcius is {} degrees Fahrenheit", u1, conv_unit);
        } else if conv == String::from("c2k") {
            conv_unit = Temperature::cels_to_kelv(u1, conv_unit);
            println!("{} degrees Celcius is {} degrees Kelvin", u1, conv_unit);
        } else if conv == String::from("f2c") {
            conv_unit = Temperature::fahr_to_cels(u1, conv_unit);
            println!(
                " {} degrees Fahrenheit is {} degrees Celsius ",
                u1, conv_unit
            );
        } else if conv == String::from("f2k") {
            conv_unit = Temperature::fahr_to_kelv(u1, conv_unit);
            println!("{} degrees Fahrenheit is {} degrees Kelvin", u1, conv_unit);
        } else if conv == String::from("k2c") {
            conv_unit = Temperature::kelv_to_cels(u1, conv_unit);
            println!("{} degrees Kelvin is {} degrees Celsius ", u1, conv_unit);
        } else if conv == String::from("k2f") {
            conv_unit = Temperature::kelv_to_fahr(u1, conv_unit);
            println!("{} degrees Kelvin is {} degrees Fahrenheit ", u1, conv_unit);
        }
    }
    // Temperature ends
}
