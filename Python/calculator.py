num1 = float(input("Ingrese el primer número: "))
op = input("Ingrese el operador (+, -, *, /): ")
num2 = float(input("Ingrese el segundo número: "))

if op == '+':
    print("Resultado:", num1 + num2)
elif op == '-':
    print("Resultado:", num1 - num2)
elif op == '*':
    print("Resultado:", num1 * num2)
elif op == '/':
    if num2 != 0:
        print("Resultado:", num1 / num2)
    else:
        print("No es posible dividir por cero.")
else:
    print("Operador no válido.")
