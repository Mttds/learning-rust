use std::io;
use std::process;

const TEMP_UNITS: [char;3] = ['K','C','F'];

fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 9.0/5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0/9.0
}

fn to_kelvin(t: f32) -> f32 {
    t + 273.15
}

fn print_result(input_temp: f32, input_unit: char, result: f32, result_unit: char) {
    println!("{0}{1} = {2}{3}", input_temp, input_unit, result, result_unit);
}

fn main() {
    println!("=== Temperature Converter ===");

    let input_temp = loop {
        println!("Please input the temperature: ");
        let mut input_temp: String = String::new();
        io::stdin().read_line(&mut input_temp).expect("Failed to read input temperature.");

        let input_temp: f32 = match input_temp.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Wrong input. Please try again"); continue }
        };

        break input_temp; // returns this when the loop ends
    };

    let input_unit = 'input_unit_loop: loop {
        println!("Please input the unit (C [Celsius], F [Fahrenheit], K [Kelvin]): ");
        let mut input_unit: String = String::new();
        io::stdin().read_line(&mut input_unit).expect("Failed to read input temperature unit.");

        let input_unit: char = match input_unit.trim().parse() {
            Ok(chr) => chr,
            Err(_) => { println!("Wrong input. Please try again"); continue }
        };

        for i in 0..3 {
            if input_unit == TEMP_UNITS[i] {
                break 'input_unit_loop input_unit; // break the name loop, otherwise we would break the innermost loop/for. Returns input_unit when the loop ends
            }
        }
    };

    let conversion_unit = 'conversion_unit_loop: loop {
        println!("Please input the conversion unit (C [Celsius], F [Fahrenheit], K [Kelvin]): ");
        let mut conversion_unit: String = String::new();
        io::stdin().read_line(&mut conversion_unit).expect("Failed to read input temperature unit.");

        let conversion_unit: char = match conversion_unit.trim().parse() {
            Ok(chr) => chr,
            Err(_) => { println!("Wrong input. Please try again"); continue }
        };

        for i in 0..3 {
            if conversion_unit == TEMP_UNITS[i] {
                break 'conversion_unit_loop conversion_unit;
            }
        }
    };

    println!("Converting {0}{1} to {2}", input_temp, input_unit, conversion_unit);
    let mut result: f32 = 0.0;

    if input_unit == conversion_unit {
        println!("The input unit cannot be equal to the conversion unit!");
        process::exit(1);
    }
    else if input_unit == 'F' && conversion_unit == 'C' {
        result = fahrenheit_to_celsius(input_temp);
    }
    else if input_unit == 'C' && conversion_unit == 'F' {
        result = celsius_to_fahrenheit(input_temp);
    }
    else if input_unit == 'C' && conversion_unit == 'K' {
        result = to_kelvin(input_temp);
    }
    else if input_unit == 'F' && conversion_unit == 'K' {
        result = to_kelvin(fahrenheit_to_celsius(input_temp));
    }
    else {
        println!("Not yet implemented!");
        process::exit(0);
    }

    print_result(input_temp, input_unit, result, conversion_unit);
}