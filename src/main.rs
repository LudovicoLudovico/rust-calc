fn main() {
    loop {
        println!("Insert the mathematical operation: ");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Could not read input...");

        // Parse input
        let input: Vec<&str> = input.split_whitespace().collect();
        let (first_operand, operation, second_operand) = process_input(&input);
        let output = do_math(&first_operand, &operation, &second_operand);

        let output = match output {
            Ok(num) => num,
            Err(MathError::DivideByZero) => panic!("Cannot divide by 0 u dumbass"),
        };

        println!("Result: {}", output);
    }
}

fn process_input(input: &Vec<&str>) -> (f64, char, f64) {
    let first_operand: f64 = match input[0].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    let second_operand: f64 = if input.len() > 2 {
        match input[2].trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Invalid input"),
        }
    } else {
        0.
    };

    let operation = match input[1] {
        "+" => '+',
        "-" => '-',
        "/" => '/',
        "*" => '*',
        "!" => '!',
        _ => panic!("Invalid operand"),
    };

    return (first_operand, operation, second_operand);
}

fn do_math(first_operand: &f64, operation: &char, second_operand: &f64) -> Result<f64, MathError> {
    return match operation {
        '+' => Ok(first_operand + second_operand),
        '-' => Ok(first_operand - second_operand),
        '*' => Ok(first_operand * second_operand),
        '/' => {
            if *second_operand == 0. {
                Err(MathError::DivideByZero)
            } else {
                Ok(first_operand / second_operand)
            }
        }
        '!' => Ok(compute_factorial(&(*first_operand as i64)) as f64),
        _ => panic!("Invalid operation"),
    };
}

fn compute_factorial(num: &i64) -> i64 {
    let mut i = 1;
    let mut result: i64 = 1;
    while i != *num + 1 {
        result *= i;
        println!("{result}");

        i += 1;
    }

    result
}

enum MathError {
    DivideByZero,
}
