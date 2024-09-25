use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");
    println!("To convert from Fahrenheit to Celsius press f, from Celcius to Fahrenheit press c.");
    
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input = user_input.trim();
    
        match user_input.to_lowercase().as_str() {
            "f" => {
                println!("Enter the temperature in Fahrenheit:");

                let mut fahrenheit = String::new();
                io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
        
                let fahrenheit = fahrenheit.trim();
                match fahrenheit.trim().parse::<f64>() {
                    
                    Ok(fahrenheit) => {
                        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

                        println!("{:.2}F° is equal to {:.2}°C", fahrenheit, celsius);
                        break;
                    }
                    
                    Err(_) => {
                        println!("please input a valid temperature");
                    
                    },
                };

                
            },
            "c" => {
                println!("Enter the temperature in celsius:");

                let mut celsius = String::new();
                io::stdin().read_line(&mut celsius).expect("Failed to read line");
                
                let celsius = celsius.trim();
                match celsius.trim().parse::<f64>() {
                    
                    Ok(celsius) => {
                        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

                        println!("{:.2}°C is equal to {:.2}°F", celsius, fahrenheit);
                        break;
                    }
                    
                    Err(_) => {
                        println!("please input a valid temperature");
                        
                    },
                };
                
            },
            _ => {
                println!("Invalid input, please type 'f' or 'c'.");
            },
        
        }
    }
}
