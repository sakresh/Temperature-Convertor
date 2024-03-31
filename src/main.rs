enum Type{
    F(String),
    C(String),
}

fn celcius_to_farenheit(celcius: f32) -> f32{
     (celcius * 9.0/5.0) + 32.0
}

fn farenheit_to_celcius(farenheit: f32) -> f32{
    (farenheit - 32.0) * 5.0/9.0
}

fn main() {
    let temp:f32;
}
