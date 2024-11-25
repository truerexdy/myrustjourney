use std::io;

fn add() -> i8 {
    let mut uin = String::new();
    println!("\nEnter the first number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let a: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    uin.clear();
    println!("\nEnter the second number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let b: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    a + b
}

fn sub() -> i8 {
    let mut uin = String::new();
    println!("\nEnter the first number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let a: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    uin.clear();
    println!("\nEnter the second number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let b: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    a - b
}

fn div() -> i8 {
    let mut uin = String::new();
    println!("\nEnter the first number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let a: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    uin.clear();
    println!("\nEnter the second number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let b: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    if b == 0 {
        println!("Cannot divide by zero!");
        0
    } else {
        a / b
    }
}

fn mul() -> i8 {
    let mut uin = String::new();
    println!("\nEnter the first number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let a: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    uin.clear();
    println!("\nEnter the second number:");
    io::stdin().read_line(&mut uin).expect("Failed to read input");
    let b: i8 = match uin.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using 0.");
            0
        }
    };

    a * b
}

fn main() {
    loop {
        println!("Choose an option:\n1.Add\n2.Subtract\n3.Multiply\n4.Divide\n5.Exit");
        let mut uin = String::new();
        io::stdin().read_line(&mut uin).expect("Failed to Read");
        let uin_num: i8 = match uin.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };

        match uin_num {
            1 => {
                let result = add();
                println!("Result: {}", result);
            }
            2 => {
                let result = sub();
                println!("Result: {}", result);
            }
            3 => {
                let result = mul();
                println!("Result: {}", result);
            }
            4 => {
                let result = div();
                println!("Result: {}", result);
            }
            5 => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}