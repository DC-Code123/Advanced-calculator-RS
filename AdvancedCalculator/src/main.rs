mod utils;
use utils::{CalculationHistory, Calculator, InputMode};
use std::io;

/// Main entry point for the Advanced Calculator application
fn main() -> io::Result<()> {
    // Create a CalculationHistory object to manage calculation history
    let mut history = CalculationHistory::new();
    
    // Check if the data directory and file exist, create/load as needed
    if let Err(e) = history.check_datafile() {
        eprintln!("Error checking data file: {}", e);
    }
    
    // Get input mode from user
    let mode = match InputMode::select_mode() {
        Ok(mode) => mode,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Ok(());
        }
    };
    
    // Run the calculator in the selected mode
    if let Err(e) = run_calculator(mode, &mut history) {
        eprintln!("Error: {}", e);
    }
    
    Ok(())
}

/// Runs the calculator in the selected mode
fn run_calculator(mode: InputMode, history: &mut CalculationHistory) -> io::Result<()> {
    match mode {
        InputMode::MenuDriven => run_menu_mode(history),
        InputMode::Expression => run_expression_mode(history),
    }
}

/// Runs the calculator in menu-driven mode
fn run_menu_mode(history: &mut CalculationHistory) -> io::Result<()> {
    loop {
        println!("Enter operation ((a)dd, (s)ub, (m)ulti, (d)iv, (p)ower, (q)uit, (h)istory, (w)rite, (l)oad): ");
        
        let mut operation = String::new();
        io::stdin().read_line(&mut operation)?;
        let operation = operation.trim().to_lowercase();
        
        // Handle special commands
        match operation.as_str() {
            "q" => {
                println!("Exiting calculator. Saving history. Goodbye!");
                history.save()?;
                break;
            }
            "w" => {
                history.save()?;
                continue;
            }
            "l" => {
                history.load()?;
                continue;
            }
            "h" => {
                history.display();
                continue;
            }
            _ => {} // Continue with arithmetic operations
        }
        
        // Validate arithmetic operation
        if !["a", "s", "m", "d", "p"].contains(&operation.as_str()) {
            println!("Invalid operation. Please enter a valid option.");
            continue;
        }
        
        // Get first number
        println!("Enter first number: ");
        let a = read_number()?;
        
        // Get second number
        println!("Enter second number: ");
        let b = read_number()?;
        
        // Perform the selected operation
        match operation.as_str() {
            "a" => {
                let result = Calculator::add(a, b);
                println!("Sum: {}", result);
                history.add_calculation("add", &format!("{} + {}", a, b), &result.to_string());
            }
            "s" => {
                let result = Calculator::sub(a, b);
                println!("Difference: {}", result);
                history.add_calculation("subtract", &format!("{} - {}", a, b), &result.to_string());
            }
            "m" => {
                let result = Calculator::multi(a, b);
                println!("Product: {}", result);
                history.add_calculation("multiply", &format!("{} * {}", a, b), &result.to_string());
            }
            "d" => {
                let result = Calculator::div_print(a, b);
                println!("{}", result);
                history.add_calculation("divide", &format!("{} / {}", a, b), &result);
            }
            "p" => {
                let result = Calculator::pow(a, b);
                println!("Power: {}", result);
                history.add_calculation("power", &format!("{} ^ {}", a, b), &result.to_string());
            }
            _ => {}
        }
        
        // Ask if user wants to continue
        if !ask_to_continue()? {
            history.save()?;
            println!("History saved. Goodbye!");
            break;
        }
    }
    
    Ok(())
}

/// Runs the calculator in expression mode
fn run_expression_mode(history: &mut CalculationHistory) -> io::Result<()> {
    loop {
        println!("Enter a math expression (or type 'q' to quit, 'h' for history, 'w' to save, 'l' to load): ");
        
        let mut expr = String::new();
        io::stdin().read_line(&mut expr)?;
        let expr = expr.trim();
        
        // Handle special commands
        match expr {
            "q" => {
                println!("Exiting calculator. Saving history. Goodbye!");
                history.save()?;
                break;
            }
            "w" => {
                history.save()?;
                continue;
            }
            "l" => {
                history.load()?;
                continue;
            }
            "h" => {
                history.display();
                continue;
            }
            _ => {} // Continue with expression evaluation
        }
        
        if expr.is_empty() {
            println!("Input cannot be empty. Please enter a valid expression or command.");
            continue;
        }
        
        // Try to parse and calculate the expression
        match Calculator::parse_and_calculate(expr, history) {
            Ok(success) => {
                if !success {
                    println!("Invalid expression or calculation error.");
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        
        // Ask if user wants to continue
        if !ask_to_continue()? {
            history.save()?;
            println!("History saved. Goodbye!");
            break;
        }
    }
    
    Ok(())
}

/// Reads a number from standard input
fn read_number() -> io::Result<f64> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().parse() {
            Ok(num) => return Ok(num),
            Err(_) => println!("Invalid input. Please enter a valid number:"),
        }
    }
}

/// Asks the user if they want to continue
fn ask_to_continue() -> io::Result<bool> {
    println!("Do you want to continue? (y/n): ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_lowercase() == "y")
}