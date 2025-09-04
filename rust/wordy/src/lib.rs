#[derive(Debug)]
enum Operation {
    Plus,
    Minus,
    Multiplied,
    Divided,
}
#[derive(Debug)]
enum Element {
    Number(i32),
    Oper(Operation),
}

pub fn answer(command: &str) -> Option<i32> {
    // todo!("Return the result of the command '{command}' or None, if the command is invalid.");
    //
    let mut stack: Vec<Element> = Vec::new();

    for ss in command.split(" ") {
        let ss: &str = &ss
            .chars()
            .filter(|ch| ch.is_alphanumeric() || *ch == '-')
            .collect::<String>();

        match ss.parse::<i32>() {
            Ok(operand) => {
                if stack.is_empty() {
                    stack.push(Element::Number(operand));
                } else {
                    match (stack.pop(), stack.pop()) {
                        (Some(Element::Oper(o)), Some(Element::Number(n))) => {
                            let result = match o {
                                Operation::Plus => operand + n,
                                Operation::Minus => n - operand,
                                Operation::Multiplied => operand * n,
                                Operation::Divided => n / operand,
                            };

                            stack.push(Element::Number(result));
                        }
                        _ => return None,
                    }
                }
            }
            Err(_) => match ss {
                "plus" => stack.push(Element::Oper(Operation::Plus)),
                "minus" => stack.push(Element::Oper(Operation::Minus)),
                "multiplied" => stack.push(Element::Oper(Operation::Multiplied)),
                "divided" => stack.push(Element::Oper(Operation::Divided)),
                "cubed" => return None,
                _ => {}
            },
        }
    }

    match stack.pop() {
        Some(Element::Number(n)) => Some(n),
        _ => None,
    }
}
