// main.rs
mod fractions;
mod base_conversion;

use fractions::Fraction;
use std::io::{self, Write};

fn main() {
    loop {
        println!("Choose an operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Modulo (Fraction form)");
        println!("6. Exponentiation");
        println!("7. Base Conversion (Decimal to Binary and vice versa)");
        println!("0. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        if choice == 0 {
            println!("Exiting...");
            break;
        }

        match choice {
            1..=5 => {
                let (frac1, frac2) = read_two_fractions();
                match choice {
                    1 => println!("Result: {}", frac1 + frac2),
                    2 => println!("Result: {}", frac1 - frac2),
                    3 => println!("Result: {}", frac1 * frac2),
                    4 => {
                        if let Some(result) = frac1.checked_div(frac2.clone()) {
                            println!("Result: {}", result);
                        } else {
                            println!("Division by zero is not allowed.");
                        }
                    }
                    5 => println!("Modulo not defined for fractions. Try integer operations."),
                    _ => (),
                }
            }
            6 => {
                let (base, exp) = read_base_and_exponent();
                let result = base.exponentiate(&exp);
                println!("Result: {}", result);
            }
            7 => {
                base_conversion::run_repl();
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn read_two_fractions() -> (Fraction, Fraction) {
    println!("Enter first fraction (e.g., 3/4):");
    let frac1 = read_fraction();

    println!("Enter second fraction (e.g., 5/6):");
    let frac2 = read_fraction();

    (frac1, frac2)
}

fn read_fraction() -> Fraction {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Fraction::from_str(input.trim()).unwrap_or_else(|_| {
        println!("Invalid fraction format. Defaulting to 0.");
        Fraction::new(0, 1)
    })
}

fn read_base_and_exponent() -> (Fraction, Fraction) {
    println!("Enter the base (e.g., 2/3):");
    let base = read_fraction();

    println!("Enter the exponent (e.g., -2):");
    let exp = read_fraction();

    (base, exp)
}
