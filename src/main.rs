use std::*;

enum TempType{
    F,
    C,
}

fn convert_temperature(temp: f32, temp_type: TempType) -> f32{
    match temp_type{
        TempType::F => (temp * 9.0/5.0) + 32.0,
        TempType::C => (temp - 32.0) * 5.0/9.0,
    }
}

fn main() {
    println!("Enter the temperature in number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input!");
    let num = input.trim().parse().expect("Failed to parse!");
    println!("Enter the temperature type to be converted: ");
    let mut temp_type = String::new();
    io::stdin().read_line(&mut temp_type).expect("Failed to read input!");
    let type_input = match temp_type.trim().to_uppercase().as_str() {
        "F" => TempType::F,
        "C" => TempType::C,
        _ => panic!("Invalid input"),
    };
    let to_type = match type_input {
        TempType::F => "Farenheit",
        TempType::C => "Celcius",
    };
    let res = convert_temperature(num, type_input);
    println!("{} in {} is {}", num, to_type, res );
}
