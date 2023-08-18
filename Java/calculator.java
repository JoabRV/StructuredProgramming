import java.util.Scanner;

public class Calculator {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        char op;
        double num1, num2;

        System.out.print("Ingrese el primer número: ");
        num1 = scanner.nextDouble();
        System.out.print("Ingrese el operador (+, -, *, /): ");
        op = scanner.next().charAt(0);
        System.out.print("Ingrese el segundo número: ");
        num2 = scanner.nextDouble();

        switch (op) {
            case '+':
                System.out.println("Resultado: " + (num1 + num2));
                break;
            case '-':
                System.out.println("Resultado: " + (num1 - num2));
                break;
            case '*':
                System.out.println("Resultado: " + (num1 * num2));
                break;
            case '/':
                if (num2 != 0) {
                    System.out.println("Resultado: " + (num1 / num2));
                } else {
                    System.out.println("No es posible dividir por cero.");
                }
                break;
            default:
                System.out.println("Operador no válido.");
        }

        scanner.close();
    }
}
