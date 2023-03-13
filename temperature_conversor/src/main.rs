use std::io;

fn main() {
    println!("Temperature Conversor");

    let temperature = capture_temperature();

    convert_temperature(temperature);
}

fn capture_temperature() -> (f64, char) {
    let temperature: f64;

    loop {
        let mut input = String::new();

        println!("Please input a valid temperature without the unit");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to capture number");

        temperature = match input.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break;
    }

    loop {
        let mut unit = String::new();

        println!("Please input a unit. The valid ones are: (C)elsius, (F)ahrenheit or (K)elvin.");

        io::stdin()
            .read_line(&mut unit)
            .ok()
            .expect("Failed to capture number");

        let mut unit = unit.chars().nth(0).unwrap();
        unit = unit.to_lowercase().collect::<Vec<_>>()[0];

        match unit {
            'c' | 'f' | 'k' => return (temperature, unit),
            _ => continue,
        };
    }
}

fn convert_temperature(temperature: (f64, char)) {
    if temperature.1 == 'c' {
        println!("Your temperature in Fahrenreit is {:.2}ºF", {
            (temperature.0 * 1.8) + 32.0
        });
        println!("Your temperature in Kelvin is {:.2}ºK", {
            temperature.0 + 273.15
        })
    }

    if temperature.1 == 'f' {
        println!("Your temperature in Celsius is {:.2}ºC", {
            (temperature.0 - 32.0) * 0.55
        });
        println!("Your temperature in Kelvin is {:.2}ºK", {
            ((temperature.0 - 32.0) * 0.55) + 273.15
        })
    }

    if temperature.1 == 'k' {
        println!("Your temperature in Celsius is {:.2}ºC", {
            temperature.0 - 273.15
        });
        println!("Your temperature in Fahrenheit is {:.2}ºF", {
            ((temperature.0 - 273.15) * 1.8) + 32.0
        })
    }
}
