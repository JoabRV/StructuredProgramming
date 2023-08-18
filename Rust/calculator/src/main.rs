use std::io;

fn main() {
    let mut input = String::new();
    let mut num1 = String::new();
    let mut op = String::new();
    let mut num2 = String::new();

    println!("Ingrese el primer número: ");
    io::stdin().read_line(&mut num1).expect("Error al leer la entrada");
    let num1: f64 = num1.trim().parse().expect("Por favor ingrese un número");

    println!("Ingrese el operador (+, -, *, /): ");
    io::stdin().read_line(&mut op).expect("Error al leer la entrada");
    let op = op.trim();

    println!("Ingrese el segundo número: ");
    io::stdin().read_line(&mut num2).expect("Error al leer la entrada");
    let num2: f64 = num2.trim().parse().expect("Por favor ingrese un número");

    match op {
        "+" => println!("Resultado: {}", num1 + num2),
        "-" => println!("Resultado: {}", num1 - num2),
        "*" => println!("Resultado: {}", num1 * num2),
        "/" => {
            if num2 != 0.0 {
                println!("Resultado: {}", num1 / num2);
            } else {
                println!("No es posible dividir por cero.");
            }
        }
        _ => println!("Operador no válido."),
    }
}
