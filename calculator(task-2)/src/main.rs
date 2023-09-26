use std::io::stdin;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut operation_is_valid = false;
    let mut first_number_is_valid = false;
    let mut second_number_is_valid = false;

    let mut a = String::new();
    let mut b = String::new();
    let mut op = String::new();

    loop {
        if !operation_is_valid {
            op = String::new();
            println!("Select the operation you want to perform\n+ for addition\n- for subtraction\n* for multiplication\n/ for division\nq to exit");
            stdin().read_line(&mut op).unwrap();
        }

        op = match op.to_lowercase().trim() {
            "+" | "-" | "*" | "/" => {
                operation_is_valid = true;
                op.trim().to_lowercase()
            }
            "q" => {
                break;
            }
            _ => {
                println!("Wrong operator please try again!");
                operation_is_valid = false;
                continue;
            }
        };

        if !first_number_is_valid {
            a = String::new();
            println!("Enter the first number: ");
            stdin().read_line(&mut a).unwrap();
        }

        let num1 = match a.trim().parse::<f64>() {
            Ok(num) => {
                first_number_is_valid = true;
                num
            }
            Err(_) => {
                println!("Enter a number!");
                first_number_is_valid = false;
                continue;
            }
        };

        if !second_number_is_valid {
            b = String::new();
            println!("Enter the second number: ");
            stdin().read_line(&mut b).unwrap();
        }

        let num2 = match b.trim().parse::<f64>() {
            Ok(num) => {
                if op == "/" && num == 0. {
                    println!("Second number can not be zero");
                    continue;
                } else {
                    second_number_is_valid = true;
                    num
                }
            }
            Err(_) => {
                println!("Enter a number!");
                second_number_is_valid = false;
                continue;
            }
        };

        let operation = match &*op {
            "+" => Operation::Add(num1, num2),
            "-" => Operation::Subtract(num1, num2),
            "*" => Operation::Multiply(num1, num2),
            "/" => Operation::Divide(num1, num2),
            _ => break,
        };
        println!("\n{} {} {} = {}\n", num1, op, num2, calculate(operation));
        operation_is_valid = false;
        first_number_is_valid = false;
        second_number_is_valid = false;
    }
}
