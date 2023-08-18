import Foundation

print("Ingrese el primer número:")
guard let num1Input = readLine(), let num1 = Double(num1Input) else {
    fatalError("Entrada inválida. Por favor, ingrese un número.")
}

print("Ingrese el operador (+, -, *, /):")
guard let op = readLine() else {
    fatalError("Entrada inválida. Por favor, ingrese un operador.")
}

print("Ingrese el segundo número:")
guard let num2Input = readLine(), let num2 = Double(num2Input) else {
    fatalError("Entrada inválida. Por favor, ingrese un número.")
}

switch op {
case "+":
    print("Resultado: \(num1 + num2)")
case "-":
    print("Resultado: \(num1 - num2)")
case "*":
    print("Resultado: \(num1 * num2)")
case "/":
    if num2 != 0.0 {
        print("Resultado: \(num1 / num2)")
    } else {
        print("No es posible dividir por cero.")
    }
default:
    print("Operador no válido.")
}
