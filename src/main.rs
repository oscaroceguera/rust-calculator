use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // El primer argumento es la ubicacion del binario compilado, así que saltar
    let first: String = args.nth(1).unwrap();
    // Después de acceder al segundo argumento, el elemento iterator siguiente se convierte en el primero
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let total = operate(operator, first_number, second_number);


    println!("{}", result(first_number, operator, second_number, total));
}

fn result(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator"),
    }
}