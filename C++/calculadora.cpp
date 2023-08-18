#include <iostream>
using namespace std;

int main()
{
    char op;
    double num1, num2;

    cout << "Ingrese el primer número: ";
    cin >> num1;
    cout << "Ingrese el operador (+, -, *, /): ";
    cin >> op;
    cout << "Ingrese el segundo número: ";
    cin >> num2;

    switch (op)
    {
    case '+':
        cout << "Resultado: " << num1 + num2 << endl;
        break;
    case '-':
        cout << "Resultado: " << num1 - num2 << endl;
        break;
    case '*':
        cout << "Resultado: " << num1 * num2 << endl;
        break;
    case '/':
        if (num2 != 0)
        {
            cout << "Resultado: " << num1 / num2 << endl;
        }
        else
        {
            cout << "No es posible dividir por cero." << endl;
        }
        break;
    default:
        cout << "Operador no válido." << endl;
    }

    return 0;
}
