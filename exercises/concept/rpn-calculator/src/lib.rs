#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for input in inputs {
        if let CalculatorInput::Value(v) = input {
            stack.push(*v);
        } else {
            let v2 = stack.pop();
            let v1 = stack.pop();
            if v1.is_none() || v2.is_none() {
                return None;
            } else {
                let v2 = v2.unwrap();
                let v1 = v1.unwrap();
                match input {
                    CalculatorInput::Add => {
                        stack.push(v1 + v2);
                    }
                    CalculatorInput::Subtract => {
                        stack.push(v1 - v2);
                    }
                    CalculatorInput::Multiply => {
                        stack.push(v1 * v2);
                    }
                    CalculatorInput::Divide => {
                        stack.push(v1 / v2);
                    }
                    _ => (),
                }
            }
        }
    }
    if stack.len() == 1 { stack.pop() } else { None }
}
