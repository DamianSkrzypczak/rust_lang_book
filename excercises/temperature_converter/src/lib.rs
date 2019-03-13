pub fn to_fahrenheit(celsius_temperature: f32) -> f32 {
    32.0 + (9.0 / 5.0) * celsius_temperature
}


pub fn to_celsius(fahrenheit_temperature: f32) -> f32 {
    (5.0 / 9.0) * (fahrenheit_temperature - 32.0)
}

pub fn convert(temperature: f32, mode: &str) {
    if mode == "fahrenheit" {
        println!("{:.2}°F => {:.2}°C", temperature, to_celsius(temperature))
    } else if mode == "celsius" {
        println!("{:.2}°C => {:.2}°F", temperature, to_fahrenheit(temperature))
    } else {
        println!("lol")
    }
}
