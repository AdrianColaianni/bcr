#![feature(exclusive_range_pattern)]
mod tests;

use rug::{ops::Pow, Float};

const PRECISION: u32 = 20;

fn main() {
    loop {
        (|| {
            let stdin = std::io::stdin();
            let mut buffer = String::new();

            stdin.read_line(&mut buffer).ok()?;
            let chars: Vec<char> = buffer.chars().collect();

            let ops = parse_input(&chars[..]);
            println!("Final: {:?}", ops);

            let res = eval(ops?);
            println!("{} = {}", buffer.trim(), res);
            Some(res)
        })();
    }
}

#[derive(Debug, PartialEq)]
enum Thing {
    BiOp(Box<Thing>, BiOp, Box<Thing>),
    #[allow(dead_code)]
    UnOp(UnOp, Box<Thing>),
    Operand(Float),
}

// Lowest to highest expression precedence
//   || operator, left associative
//   && operator, left associative
//   ! operator, nonassociative
//   Relational operators, left associative
//   Assignment operator, right associative
//   + and - operators, left associative
//   *, / and % operators, left associative
//   ˆ operator, right associative
//   unary - operator, nonassociative
//   ++ and -- operators, nonassociative
#[derive(Debug, PartialEq, PartialOrd)]
enum BiOp {
    // PreInc,
    // PreDec,
    // PostInc,
    // PostDec,
    // Relational Operators
    Eq, // ==
    Ne, // !=
    Gt, // >
    Ge, // >=
    Lt, // <
    Le, // <=
    // Regular Operators
    Sub, // -
    Add, // +
    Mod, // %
    Div, // /
    Mul, // *
    Pow, // ^
}

impl std::fmt::Display for BiOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Eq => "==",
                Self::Ne => "!=",
                Self::Gt => ">",
                Self::Ge => ">=",
                Self::Lt => "<",
                Self::Le => "<=",
                Self::Sub => "-",
                Self::Add => "+",
                Self::Mod => "%",
                Self::Div => "/",
                Self::Mul => "*",
                Self::Pow => "^",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum UnOp {
    Neg, // -
}

fn parse_input(input: &[char]) -> Option<Thing> {
    let mut i = 0;

    let mut root: Option<Thing> = None;

    let mut left: Option<Thing> = None;
    let mut op: Option<BiOp> = None;
    let mut right: Option<Thing> = None;

    loop {
        if op.is_some() && is_op(input[i]) {
            println!(
                "Invalid operator '{}' after '{}'",
                input[i],
                op.as_ref().unwrap()
            );
            return None;
        }
        match input[i] {
            '0'..'9' => {
                let mut num: String = input[i].into();
                if let Some(mut n) = input.get(i + 1) {
                    while *n >= '0' && *n <= '9' || *n == '.' {
                        num.push(*n);
                        i += 1;
                        n = match input.get(i + 1) {
                            Some(n) => n,
                            None => break,
                        };
                    }
                }

                if left.is_none() {
                    left = Some(Thing::Operand(Float::with_val(
                        PRECISION,
                        Float::parse(num).unwrap(),
                    )));
                    // println!("Found {}", left.as_ref().unwrap());
                } else if right.is_none() {
                    right = Some(Thing::Operand(Float::with_val(
                        PRECISION,
                        Float::parse(num).unwrap(),
                    )));
                    // println!("Found {}", right.as_ref().unwrap());
                } else {
                    panic!("Invalid input: {}", num);
                }
            }
            '+' => {
                if input.get(i + 1).is_some_and(|n| *n == '+') {
                    todo!("Pre/Postfix addition");
                    // i += 1;
                } else {
                    op = Some(BiOp::Add);
                }
            }
            '-' => {
                if op.is_some() || (left.is_none() && root.is_none()) {
                    todo!("Negation");
                } else {
                    if input.get(i + 1).is_some_and(|n| *n == '-') {
                        todo!("Pre/Postfix subtraction");
                        // i += 1;
                    } else {
                        op = Some(BiOp::Sub);
                    }
                }
            }
            '/' => op = Some(BiOp::Div),
            '*' => op = Some(BiOp::Mul),
            '%' => op = Some(BiOp::Mod),
            '^' => op = Some(BiOp::Pow),
            '(' => {
                let mut end = i;
                for (pi, c) in input[end..].iter().enumerate() {
                    if *c == ')' {
                        end = pi + i;
                    }
                }

                if end == i {
                    println!("Invalid numer of parenthesis");
                    return None;
                }

                let rec = &input[i + 1..end];

                if left.is_none() {
                    println!("Recusing on {:?}", rec);
                    left = Some(parse_input(rec).unwrap());
                } else if right.is_none() {
                    println!("Recusing on {:?}", rec);
                    right = Some(parse_input(rec).unwrap());
                } else {
                    panic!("Invalid input at {}", i);
                }
                i = end;
            }
            ')' => (),
            '<' => {
                if input.get(i+1).is_some_and(|n| *n == '=') {
                    op = Some(BiOp::Le);
                    i += 1;
                } else {
                    op = Some(BiOp::Lt);
                }
            }
            '>' => {
                if input.get(i+1).is_some_and(|n| *n == '=') {
                    op = Some(BiOp::Ge);
                    i += 1;
                } else {
                    op = Some(BiOp::Gt);
                }
            }
            '=' => {
                if input.get(i+1).is_some_and(|n| *n == '=') {
                    op = Some(BiOp::Eq);
                    i += 1;
                } else {
                    todo!("Variable assignment");
                }
            }
            '!' => {
                if input.get(i+1).is_some_and(|n| *n == '=') {
                    op = Some(BiOp::Ne);
                    i += 1;
                } else {
                    todo!("Boolean not");
                }
            }
            '&' => todo!(),
            '|' => todo!(),
            '\n' => (),
            ';' => (),
            ' ' => (),
            c => todo!("Encountered unknown {}", c),
        }

        println!("{:?} - {:?} - {:?}", left, op, right);

        if left.is_some() && op.is_some() && right.is_some() {
            let left = left.take().unwrap();
            let op = op.take().unwrap();
            let right = right.take().unwrap();

            root = Some(add_ops(root, Some(left), op, right));
            println!("Built new op: {:?}", root);
        } else if root.is_some() && op.is_some() && left.is_some() {
            let left = left.take().unwrap();
            let op = op.take().unwrap();

            root = Some(add_ops(root, None, op, left));
            println!("Built new op: {:?}", root);
        }

        i += 1;
        if i == input.len() {
            println!("Goodby");
            return if root.is_some() {
                root
            } else if left.is_some() {
                left
            } else {
                right
            };
        }
    }
}

fn add_ops(root: Option<Thing>, left: Option<Thing>, op: BiOp, right: Thing) -> Thing {
    // If root and left are both some, there is nowhere to put the new Thing, but if both are none
    // there is not enough to build a new Thing
    assert_ne!(root.is_some(), left.is_some());

    if let Some(root) = root {
        // todo!("Tricky bit of determining operator precedence");
        match root {
            Thing::BiOp(l, o, r) => {
                if o >= op {
                    // root operator has higher precedence than our operator
                    Thing::BiOp(Box::new(Thing::BiOp(l, o, r)), op, Box::new(right))
                } else {
                    // root operator has lower precedence than our operator
                    Thing::BiOp(l, o, Box::new(Thing::BiOp(r, op, Box::new(right))))
                }
            }
            Thing::UnOp(..) => todo!(),
            Thing::Operand(_) => panic!("What the funk"),
        }
    } else if let Some(left) = left {
        Thing::BiOp(Box::new(left), op, Box::new(right))
    } else {
        panic!("Bruv");
    }
}

fn is_op(op: char) -> bool {
    "|&!+-*/%^<>=".contains(op)
}

fn eval(op: Thing) -> Float {
    match op {
        Thing::BiOp(l, o, r) => match o {
            BiOp::Eq => Float::with_val(PRECISION, (eval(*l) == eval(*r)) as usize),
            BiOp::Ne => Float::with_val(PRECISION, (eval(*l) != eval(*r)) as usize),
            BiOp::Gt => Float::with_val(PRECISION, (eval(*l) > eval(*r)) as usize),
            BiOp::Ge => Float::with_val(PRECISION, (eval(*l) >= eval(*r)) as usize),
            BiOp::Lt => Float::with_val(PRECISION, (eval(*l) < eval(*r)) as usize),
            BiOp::Le => Float::with_val(PRECISION, (eval(*l) <= eval(*r)) as usize),
            BiOp::Add => eval(*l) + eval(*r),
            BiOp::Sub => eval(*l) - eval(*r),
            BiOp::Mul => eval(*l) * eval(*r),
            BiOp::Div => eval(*l) / eval(*r),
            BiOp::Mod => {
                let l = eval(*l);
                let r = eval(*r);

                l.clone() - (l / r.clone()) * r
            }
            BiOp::Pow => eval(*l).pow(eval(*r)),
        },
        Thing::UnOp(o, v) => match o {
            UnOp::Neg => -eval(*v),
        },
        Thing::Operand(v) => v,
    }
}
