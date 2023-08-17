use std::io;
use std::char;

fn main() {
    //println!("Hello, world!");
    let temperatures: [f64; 9] = 
    [273.15, 100.0, 0.0, 500.0, 1000.0, 99.5, 108.0, 24.0, 15.0];

    let nth_n_s: [u128;18] = [10,123,5,8,2,16,2,0,1,2,3,4,5,6,7,8,9,11];

    let mut prompt: String = String::new();

    println!("1 to Convert Temperatues, 2 for Nth Fibonacci");
    io::stdin()
        .read_line(&mut prompt)
        .expect("Failed to read line");

    let prompt: u32 = match prompt.trim().parse() {
                                Ok(number) => number,
                                Err(_) => 0,
    };

    if prompt == 1 {
        for temp in temperatures {

            println!("{} C is converted to {} K", temp, temperature(temp, 'C', "K"));
            
            println!("-------------------------------------------------------------");

            println!("{} K is converted to {} C", temp, temperature(temp, 'K', "C"));
            
            println!("============================================================");
        }
    }

    if prompt == 2 {
        for n_input in nth_n_s{
            println!("The {}th Fibonacci number is {}", n_input, nth_fibonacci(n_input));
        }
    }

    println!("Done with program!")
}

fn temperature(temperature: f64, unit: char, convert_to: &str) -> f64 {

    let mut converted_temperature: f64 = 0.0;

    if unit == 'F' {
        converted_temperature = match convert_to {
            "K" => ((temperature-32.0)*(5.0/9.0)) +273.15,
            "C" => (temperature-32.0)*(5.0/9.0),
            &_ => todo!(),
        }
        
    } 

    if unit == 'C' {
        converted_temperature = match convert_to {
            "F" => (temperature*(9.0/5.0)) + 32.0,
            "K" => temperature +273.15,
            &_ => todo!()
        }
    }

    if unit == 'K' {
        converted_temperature = match convert_to {
            "F" => (-273.15*(9.0/5.0)) + 32.0 - temperature,
            "C" => temperature -273.15,
            &_ => todo!()

        }
    }

    converted_temperature
}


fn nth_fibonacci(n: u128) -> u128 {

    let mut n_0: u128 = 0;
    let mut n_1: u128 = 1;
    if n == 0 {
        return n_0;
    } 

    if n == 1 {
        return n_1;
    }

    let mut n_n: u128 = n_0 + n_1;
    let mut index: u128 = 2;

    while index<n {

        n_0 = n_1;
        n_1 = n_n;

        n_n = n_0+n_1;

        index += 1;


    }

    n_n



}
