use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::{self, Write, BufReader, BufWriter};
use std::path::Path;
use chrono::prelude::*;

/// Represents the input mode for the calculator
#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    MenuDriven,
    Expression,
}

impl InputMode {
    /// Prompts the user to select an input mode
    pub fn select_mode() -> io::Result<Self> {
        loop {
            println!("Select input mode: 1) Menu-driven  2) Expression input");
            println!("Enter 1 or 2: ");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            match input.trim() {
                "1" => return Ok(InputMode::MenuDriven),
                "2" => return Ok(InputMode::Expression),
                _ => println!("Invalid choice. Please enter 1 or 2."),
            }
        }
    }
}

/// Represents a single calculation with operation, expression, result, and timestamp
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calculation {
    pub operation: String,    // The type of operation performed
    pub expression: String,   // The mathematical expression or operands used
    pub result: String,       // The result of the calculation
    pub timestamp: String,    // The date and time when the calculation was performed
}

impl Calculation {
    /// Creates a new Calculation instance
    pub fn new(operation: &str, expression: &str, result: &str) -> Self {
        let now = Local::now();
        Calculation {
            operation: operation.to_string(),
            expression: expression.to_string(),
            result: result.to_string(),
            timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/// Manages the history of calculations with JSON storage
#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationHistory {
    calculations: Vec<Calculation>, // Vector to store all calculations
}

impl CalculationHistory {
    /// Creates a new empty CalculationHistory
    pub fn new() -> Self {
        println!("Database initialized.");
        CalculationHistory {
            calculations: Vec::new(),
        }
    }
    
    /// Returns the path to the calculations.json file in the data directory
    fn get_data_path() -> std::path::PathBuf {
        Path::new("data").join("calculations.json")
    }
    
    /// Ensures the data directory exists
    fn ensure_data_directory() -> io::Result<()> {
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
            println!("Created data directory.");
        }
        Ok(())
    }
    
    /// Checks if the data file exists and loads it, or creates a new one
    pub fn check_datafile(&mut self) -> io::Result<()> {
        Self::ensure_data_directory()?;
        
        let data_path = Self::get_data_path();
        if data_path.exists() {
            println!("Data file found.");
            self.load()?;
        } else {
            println!("Data file not found. Creating new data file...");
            self.create_datafile()?;
        }
        
        Ok(())
    }
    
    /// Creates a new empty data file
    fn create_datafile(&self) -> io::Result<()> {
        let data_path = Self::get_data_path();
        let file = File::create(&data_path)?;
        
        // Write empty array to JSON file
        serde_json::to_writer_pretty(file, &Vec::<Calculation>::new())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        println!("New data file created in /data.");
        Ok(())
    }
    
    /// Saves all calculations to the JSON file
    pub fn save(&self) -> io::Result<()> {
        Self::ensure_data_directory()?;
        
        let data_path = Self::get_data_path();
        let file = File::create(&data_path)?;
        let writer = BufWriter::new(file);
        
        serde_json::to_writer_pretty(writer, &self.calculations)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        println!("History saved to file in /data.");
        Ok(())
    }
    
    /// Loads calculations from the JSON file
    pub fn load(&mut self) -> io::Result<()> {
        let data_path = Self::get_data_path();
        
        if !data_path.exists() {
            println!("No saved calculations found.");
            return Ok(());
        }
        
        let file = File::open(&data_path)?;
        let reader = BufReader::new(file);
        
        self.calculations = serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
        println!("History loaded from file in /data.");
        Ok(())
    }
    
    /// Adds a new calculation to the history
    pub fn add_calculation(&mut self, operation: &str, expression: &str, result: &str) {
        let calc = Calculation::new(operation, expression, result);
        self.calculations.push(calc);
    }
    
    /// Clears the calculation history
    pub fn clear(&mut self) {
        self.calculations.clear();
    }
    
    /// Returns true if the history is empty
    pub fn is_empty(&self) -> bool {
        self.calculations.is_empty()
    }
    
    /// Displays all calculations in the history
    pub fn display(&self) {
        if self.calculations.is_empty() {
            println!("No history available.");
            return;
        }
        
        println!("History of calculations:");
        for (i, calc) in self.calculations.iter().enumerate() {
            println!("{}. Operation: {}, Expression: {}, Result: {}, Timestamp: {}", 
                i + 1, calc.operation, calc.expression, calc.result, calc.timestamp);
        }
    }
}

/// Provides calculator operations and expression parsing
pub struct Calculator;

impl Calculator {
    /// Adds two numbers with overflow/underflow check
    pub fn add(x: f64, y: f64) -> f64 {
        let result = x + y;
        // Check for overflow/underflow
        if (x > 0.0 && y > 0.0 && result < 0.0) || (x < 0.0 && y < 0.0 && result > 0.0) {
            eprintln!("Warning: Addition overflow/underflow detected.");
        }
        result
    }
    
    /// Subtracts two numbers with overflow/underflow check
    pub fn sub(x: f64, y: f64) -> f64 {
        let result = x - y;
        // Check for overflow/underflow
        if (x > 0.0 && y < 0.0 && result < 0.0) || (x < 0.0 && y > 0.0 && result > 0.0) {
            eprintln!("Warning: Subtraction overflow/underflow detected.");
        }
        result
    }
    
    /// Divides two numbers with error checking
    pub fn div(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            eprintln!("Error: Division by zero");
            return f64::NAN;
        }
        if !x.is_finite() || !y.is_finite() {
            eprintln!("Error: Non-finite number in division.");
            return f64::NAN;
        }
        x / y
    }
    
    /// Multiplies two numbers with overflow/underflow check
    pub fn multi(x: f64, y: f64) -> f64 {
        let result = x * y;
        // Check for overflow/underflow
        if x != 0.0 && result / x != y {
            eprintln!("Warning: Multiplication overflow/underflow detected.");
        }
        result
    }
    
    /// Calculates modulus with error checking
    pub fn mod_op(x: f64, y: f64) -> f64 {
        if y == 0.0 {
            eprintln!("Error: Modulus by zero");
            return f64::NAN;
        }
        if !x.is_finite() || !y.is_finite() {
            eprintln!("Error: Non-finite number in modulus.");
            return f64::NAN;
        }
        x % y
    }
    
    /// Returns division result and remainder as string
    pub fn div_print(x: f64, y: f64) -> String {
        if y == 0.0 {
            return "Error: Division by zero".to_string();
        }
        let quotient = Self::div(x, y);
        let remainder = Self::mod_op(x, y);
        format!("{} R {}", quotient, remainder)
    }
    
    /// Calculates power with error checking
    pub fn pow(base: f64, exponent: f64) -> f64 {
        if base == 0.0 && exponent <= 0.0 {
            eprintln!("Error: Invalid operation: 0 raised to a non-positive power.");
            return f64::NAN;
        }
        if !base.is_finite() || !exponent.is_finite() {
            eprintln!("Error: Non-finite number in power operation.");
            return f64::NAN;
        }
        base.powf(exponent)
    }
    
    /// Checks if a string is a valid number
    pub fn is_number(s: &str) -> bool {
        if s.is_empty() || s.len() > 100 {
            return false;
        }
        match s.parse::<f64>() {
            Ok(num) => num.is_finite(),
            Err(_) => false,
        }
    }
    
    /// Checks if a string is a supported operator
    pub fn is_operator(token: &str) -> bool {
        matches!(token, "+" | "-" | "*" | "/" | "%")
    }
    
    /// Gets the precedence of an operator
    pub fn get_precedence(op: &str) -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" | "%" => 2,
            _ => 0,
        }
    }
    
    /// Tokenizes a mathematical expression
    pub fn tokenize(expr: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut paren_count = 0;
        
        for c in expr.chars() {
            if c.is_whitespace() {
                continue;
            }
            
            if c.is_ascii_digit() || c == '.' {
                current.push(c);
            } else if c == '(' {
                paren_count += 1;
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push("(".to_string());
            } else if c == ')' {
                paren_count -= 1;
                if paren_count < 0 {
                    eprintln!("Warning: Unmatched closing parenthesis.");
                }
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(")".to_string());
            } else if Self::is_operator(&c.to_string()) {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                // Handle unary minus
                if c == '-' && (tokens.is_empty() || tokens.last() == Some(&"(".to_string()) || Self::is_operator(tokens.last().unwrap())) {
                    current.push(c);
                } else {
                    tokens.push(c.to_string());
                }
            } else {
                eprintln!("Warning: Invalid character '{}' in expression.", c);
            }
        }
        
        if !current.is_empty() {
            tokens.push(current);
        }
        
        if paren_count != 0 {
            eprintln!("Warning: Unbalanced parentheses in expression.");
        }
        
        tokens
    }
    
    /// Converts infix tokens to postfix notation
    pub fn infix_to_postfix(tokens: &[String]) -> Vec<String> {
        let mut postfix = Vec::new();
        let mut op_stack = Vec::new();
        
        for token in tokens {
            if Self::is_number(token) {
                postfix.push(token.clone());
            } else if token == "(" {
                op_stack.push(token.clone());
            } else if token == ")" {
                while let Some(top) = op_stack.pop() {
                    if top == "(" {
                        break;
                    }
                    postfix.push(top);
                }
            } else if Self::is_operator(token) {
                while let Some(top) = op_stack.last() {
                    if top == "(" || Self::get_precedence(top) < Self::get_precedence(token) {
                        break;
                    }
                    postfix.push(op_stack.pop().unwrap());
                }
                op_stack.push(token.clone());
            }
        }
        
        while let Some(op) = op_stack.pop() {
            postfix.push(op);
        }
        
        postfix
    }
    
    /// Evaluates a postfix expression
    pub fn evaluate_postfix(postfix: &[String]) -> Result<f64, String> {
        let mut stack = Vec::new();
        
        for token in postfix {
            if Self::is_number(token) {
                stack.push(token.parse::<f64>().unwrap());
            } else if Self::is_operator(token) {
                if stack.len() < 2 {
                    return Err("Invalid expression: not enough operands".to_string());
                }
                
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                
                if !a.is_finite() || !b.is_finite() {
                    return Err("Non-finite operand in expression".to_string());
                }
                
                let result = match token.as_str() {
                    "+" => Self::add(a, b),
                    "-" => Self::sub(a, b),
                    "*" => Self::multi(a, b),
                    "/" => Self::div(a, b),
                    "%" => Self::mod_op(a, b),
                    _ => return Err(format!("Unknown operator: {}", token)),
                };
                
                if !result.is_finite() {
                    return Err("Result is not a finite number".to_string());
                }
                
                stack.push(result);
            }
        }
        
        if stack.len() != 1 {
            return Err("Invalid expression".to_string());
        }
        
        Ok(stack[0])
    }
    
    /// Evaluates a mathematical expression string
    pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
        let tokens = Self::tokenize(expr);
        let postfix = Self::infix_to_postfix(&tokens);
        Self::evaluate_postfix(&postfix)
    }
    
    /// Parses and calculates an expression, adding it to history
    pub fn parse_and_calculate(expr: &str, history: &mut CalculationHistory) -> Result<bool, String> {
        if expr.is_empty() {
            return Ok(false);
        }
        
        match Self::evaluate_expression(expr) {
            Ok(result) => {
                if result.is_nan() {
                    eprintln!("Calculation error occurred");
                    Ok(false)
                } else {
                    history.add_calculation("expression", expr, &result.to_string());
                    println!("Result: {}", result);
                    Ok(true)
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                Ok(false)
            }
        }
    }
}