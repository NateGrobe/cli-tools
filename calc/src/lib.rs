pub struct Config {
    pub expression: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let expression = match args.next() {
            Some(ex) => ex,
            None => return Err("No expression was given")
        };

        Ok(Config { expression })
    }

    pub fn parse_expression(&mut self) -> Vec<String> {
        let mut chars = self.expression.chars();
        let mut expression: Vec<String> = Vec::new();

        loop {
            let mut token = String::new();
            let mut current_char = match chars.next() {
                Some(c) => c,
                None => break
            };

            while !"+-*/^()".contains(&current_char.to_string()) {
                if current_char != ' '{
                    token.push(current_char);
                }
                current_char = match chars.next() {
                    Some(c) => c,
                    None => break
                };
            }

            if token.len() > 0 {
                expression.push(token);
            }

            token = String::new();

            if token.len() == 0 {
                if "+-*/^".contains(&current_char.to_string()) {
                    token.push(current_char);
                } else {
                    if current_char == ')' {
                        let next_char = match chars.next() {
                            Some(c) => c,
                            None => {
                                expression.push(current_char.to_string());
                                break
                            }
                        };

                        if next_char == '(' {
                            expression.push(current_char.to_string());
                            expression.push('*'.to_string());
                            token.push(next_char);
                        } else {
                            expression.push(current_char.to_string());
                            token.push(next_char);
                        }

                    } else {
                        if current_char == '(' {
                            token.push(current_char);
                        }
                    }
                }
            }

            if token.len() > 0 {
                expression.push(token);
            }
        }

        expression
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    queue: Vec<T>
}

impl<T> Queue<T> 
where T: Clone
{
    pub fn new() -> Queue<T> {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, val: T) {
        self.queue.push(val);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let val = self.queue[0].clone();
        self.queue = self.queue.drain(1..).collect();
        Some(val)
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    stack: Vec<T>
}

impl<T> Stack<T> 
where T: Copy
{
    pub fn new() -> Stack<T> {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<T> {
        match self.stack.len() {
            0 => None,
            _ => Some(self.stack[self.stack.len() -1])
        }       

    }

    pub fn length(&self) -> usize {
        self.stack.len()
    }
}

fn is_operator(token: &str) -> bool {
    "+-*/^".contains(&token.to_string())
}

fn has_precedence(op1: &str, op2: &str) -> bool {
    let w1 = match op1 {
        "^" => 2,
        "*" => 1,
        "/" => 1,
        _ => 0
    };

    let w2 = match op2 {
        "^" => 2,
        "*" => 1,
        "/" => 1,
        _ => 0
    };

    w1 > w2
}

fn has_equal_precedence(op1: &str, op2: &str) -> bool {
    let w1 = match op1 {
        "^" => 2,
        "*" => 1,
        "/" => 1,
        _ => 0
    };

    let w2 = match op2 {
        "^" => 2,
        "*" => 1,
        "/" => 1,
        _ => 0
    };

    w1 == w2
}

fn is_num(val: &str) -> bool {
    match val.parse::<i64>() {
        Ok(_) => true,
        _ => false
    }
}

// TODO: add support for modulus
// This is an implementation of the Shunting-yard algorithm
pub fn rpn(equation: Vec<&str>) -> Vec<&str> {
    let mut output: Queue<&str> = Queue::new();
    let mut operators: Stack<&str> = Stack::new();
    let mut tokens = equation.iter();

    loop {
        let token = match tokens.next() {
            Some(token) => token,
            None => {
                while operators.length() > 0 {
                    output.enqueue(operators.pop().unwrap());
                }
                break
            }
        };

        if is_num(token) {
            output.enqueue(token);
        } else if is_operator(token) {
            while operators.length() > 0
            && (has_precedence(operators.peek().unwrap(), token) 
                || has_equal_precedence(operators.peek().unwrap(), token) 
                && token != &"^")
            && operators.peek().unwrap() != "(" {
                output.enqueue(operators.pop().unwrap());
            }
            operators.push(token);
        } else if token == &"(" {
            operators.push(token);
        } else if token == &")" {
            let mut op_on_stack = operators.peek().unwrap();
            while op_on_stack != "(" {
                output.enqueue(operators.pop().unwrap());
                op_on_stack = operators.peek().unwrap();
            }

            match operators.peek().unwrap() {
                "(" => operators.pop(),
                _ => continue
            };
        }
    }

    output.queue
}

// TODO: add support for modulus
pub fn calculate(ex_in_rpn: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let mut stack: Vec<String> = Vec::new();

    for token in ex_in_rpn.iter() {
        if !is_operator(token) {
            stack.push(token.to_string());
        } else {
            match token {
                &"+" => {
                    let v1 = stack.pop().unwrap().parse::<f64>()?;
                    let v2 = stack.pop().unwrap().parse::<f64>()?;
                    stack.push((v1 + v2).to_string());
                },
                &"-" => {
                    let v1 = stack.pop().unwrap().parse::<f64>()?;
                    let v2 = stack.pop().unwrap().parse::<f64>()?;
                    stack.push((v2 - v1).to_string());
                },
                &"*" => {
                    let v1 = stack.pop().unwrap().parse::<f64>()?;
                    let v2 = stack.pop().unwrap().parse::<f64>()?;
                    stack.push((v1 * v2).to_string());
                },
                &"/" => {
                    let v1 = stack.pop().unwrap().parse::<f64>()?;
                    let v2 = stack.pop().unwrap().parse::<f64>()?;
                    stack.push((v2 / v1).to_string());
                },
                &"^" => {
                    let v1 = stack.pop().unwrap().parse::<f64>()?;
                    let v2 = stack.pop().unwrap().parse::<f64>()?;
                    stack.push((v2.powf(v1)).to_string());
                },
                _ => ()
            }
        }
    }

    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new();
        q.enqueue("3");
        q.enqueue("4");
        q.enqueue("+");

        assert_eq!(q.length(), 3);
        assert_eq!(q.dequeue().unwrap(), "3");
        assert_eq!(q.dequeue().unwrap(), "4");
        assert_eq!(q.dequeue().unwrap(), "+");
        assert_eq!(q.length(), 0);
    }

    #[test]
    fn test_stack() {
        let mut s = Stack::new();
        s.push("3");
        s.push("4");
        s.push("+");

        assert_eq!(s.length(), 3);
        assert_eq!(s.peek().unwrap(), "+");
        assert_eq!(s.pop().unwrap(), "+");
        assert_eq!(s.pop().unwrap(), "4");
        assert_eq!(s.pop().unwrap(), "3");
        assert_eq!(s.length(), 0);
    }

    #[test]
    fn test_operator() {
        assert!(is_operator("+"));
        assert!(is_operator("-"));
        assert!(is_operator("/"));
        assert!(is_operator("*"));
        assert!(is_operator("^"));
        assert_eq!(is_operator("c"), false);
    }

    #[test]
    fn test_rpn() {
        let result1 = vec!["3", "4", "+"];
        assert_eq!(rpn(vec!["3","+", "4"]), result1);

        let result2 = vec!["3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+"];
        assert_eq!(rpn(vec!["3", "+", "4", "*", "2", "/", "(", "1", "-", "5", ")", "^", "2", "^", "3"]), result2);
    }

    #[test]
    fn test_calculate() {
        let result1 = String::from("7");
        assert_eq!(result1, calculate(vec!["3", "4", "+"]).unwrap());

        let result2 = String::from("3.5");
        assert_eq!(result2, calculate(vec!["3", "4", "2", "*", "1", "5", "-", "2", "^", "/", "+"]).unwrap());
    }
}
