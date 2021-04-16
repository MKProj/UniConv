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

    // First temperature
    if mode == String::from("temp") {
        let u1: f32 = unit.trim().parse().expect("Expected valid float unit");
        if conv == String::from("cels_to_fahr") {
            conv_unit = temperature::cels_to_fahr(u1);
            println!("{} degrees Celcius is {} degrees Fahrenheit", u1, conv_unit);
        } else if conv == String::from("cles_to_kelv") {
            conv_unit = temperature::cels_to_kelv(u1);
            println!("{} degrees Celcius is {} degrees Kelvin", u1, conv_unit);
        } else if conv == String::from("fahr_to_cels") {
            conv_unit = temperature::fahr_to_cels(u1);
            println!(
                " {} degrees Fahrenheit is {} degrees Celsius ",
                u1, conv_unit
            );
        } else if conv == String::from("fahr_to_kelv") {
            conv_unit = temperature::fahr_to_kelv(u1);
            println!("{} degrees Fahrenheit is {} degrees Kelvin", u1, conv_unit);
        } else if conv == String::from("kelv_to_cels") {
            conv_unit = temperature::kelv_to_cels(u1);
            println!("{} degrees Kelvin is {} degrees Celsius ", u1, conv_unit);
        } else if conv == String::from("kelv_to_fahr") {
            conv_unit = temperature::kelv_to_fahr(u1);
            println!("{} degrees Kelvin is {} degrees Fahrenheit ", u1, conv_unit);
        }
    }
    // temperature ends
}
