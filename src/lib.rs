use std::error::Error;

#[derive(Debug)]
pub struct Config{
    pub convert_to: String,
    pub degree: f64,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            panic!("Not enough arguments");
        } else if args.len() >= 4 {
            panic!("Too many arguments. This CLI only accepts 2 arguments.");
        }

        let convert_to = args[1].clone();
        let degree = args[2].parse::<f64>().unwrap();
        Ok(Config{convert_to: convert_to, degree: degree})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let conver_to = config.convert_to.clone();

    if conver_to.to_lowercase() == "celcius" {
        let result = celcius_to_fahrenheit(config.degree);
        println!("{} Celcius same as {:.2} Fahrenheit", config.degree ,result);
    }else {
        let result = fahrenheit_to_celcius(config.degree);
        println!("{} Fahrenheit same as {:.2} Celcius", config.degree ,result);
    }
    Ok(())
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8 + 32.0
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 1.8 + 32.0
}