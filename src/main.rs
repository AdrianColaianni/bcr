#![feature(exclusive_range_pattern)]
use rug::{ops::Pow, Float};

const PRECISION: u32 = 20;

fn main() {
    println!("{}", BiOp::Mul > BiOp::Add);
    loop {
        (|| {
            let ops = read_input()?;
            println!("Final: {:?}", ops);
            let res = eval(ops);
            println!("Final res: {}", res);
            Some(res)
        })();
    }
}

#[derive(Debug, PartialEq)]
enum Thing {
    BiOp(Box<Thing>, BiOp, Box<Thing>),
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
                BiOp::Sub => '-',
                BiOp::Add => '+',
                BiOp::Mod => '%',
                BiOp::Div => '/',
                BiOp::Mul => '*',
                BiOp::Pow => '^',
            }
        )
    }
}

#[derive(Debug, PartialEq)]
enum UnOp {
    Neg, // -
}

fn read_input() -> Option<Thing> {
    let stdin = std::io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();

    let mut buffer = buffer.chars().into_iter();
    let mut c = buffer.next()?;

    let mut root: Option<Thing> = None;

    let mut left: Option<Float> = None;
    let mut op: Option<BiOp> = None;
    let mut right: Option<Float> = None;

    loop {
        if left.is_some() && op.is_some() && right.is_some() {
            let left = left.take().unwrap();
            let op = op.take().unwrap();
            let right = right.take().unwrap();

            root = Some(add_ops(root, Some(left), op, right));
        } else if root.is_some() && op.is_some() && left.is_some() {
            let left = left.take().unwrap();
            let op = op.take().unwrap();

            root = Some(add_ops(root, None, op, left));
        }

        match c {
            '0'..'9' => {
                let mut thing = String::new();
                while c >= '0' && c <= '9' {
                    thing.push(c);
                    c = buffer.next()?;
                }

                if left.is_none() {
                    left = Some(Float::with_val(PRECISION, Float::parse(thing).unwrap()));
                    // println!("Found {}", left.as_ref().unwrap());
                } else if right.is_none() {
                    right = Some(Float::with_val(PRECISION, Float::parse(thing).unwrap()));
                    // println!("Found {}", right.as_ref().unwrap());
                } else {
                    panic!("Invalid input: {}", thing);
                }
                continue;
            }
            '+' => {
                c = buffer.next()?;
                if c == '+' {
                    todo!("Pre/Postfix addition");
                } else {
                    op = Some(BiOp::Add);
                    continue;
                }
            }
            '-' => {
                if op.is_some() || (left.is_none() && root.is_none()) {
                    todo!("Negation");
                } else {
                    c = buffer.next()?;
                    if c == '-' {
                        todo!("Pre/Postfix subtraction");
                    } else {
                        op = Some(BiOp::Sub);
                        continue;
                    }
                }
            }
            '/' => {
                if let Some(op) = &op {
                    println!("Invalid operand '/' after '{}'", op);
                    return None;
                }
                op = Some(BiOp::Div);
            }
            '*' => {
                if let Some(op) = &op {
                    println!("Invalid operand '*' after '{}'", op);
                    return None;
                }
                op = Some(BiOp::Mul);
            }
            '%' => {
                if let Some(op) = &op {
                    println!("Invalid operand '%' after '{}'", op);
                    return None;
                }
                op = Some(BiOp::Mod);
            }
            '^' => {
                if let Some(op) = &op {
                    println!("Invalid operand '^' after '{}'", op);
                    return None;
                }
                op = Some(BiOp::Pow);
            }
            '(' => todo!(),
            ')' => todo!(),
            '<' => todo!(),
            '>' => todo!(),
            '=' => todo!(),
            '!' => todo!(),
            '&' => todo!(),
            '|' => todo!(),
            '\n' => return root,
            ' ' => (),
            c => todo!("Encountered unknown {}", c),
        }
        c = buffer.next()?;
    }
}

fn add_ops(root: Option<Thing>, left: Option<Float>, op: BiOp, right: Float) -> Thing {
    if let Some(root) = root {
        // todo!("Tricky bit of determining operator precedence");
        match root {
            Thing::BiOp(l, o, r) => {
                if o >= op {
                    // root operator has higher precedence than our operator
                    Thing::BiOp(
                        Box::new(Thing::BiOp(l, o, r)),
                        op,
                        Box::new(Thing::Operand(right)),
                    )
                } else {
                    // root operator has lower precedence than our operator
                    Thing::BiOp(
                        l,
                        o,
                        Box::new(Thing::BiOp(r, op, Box::new(Thing::Operand(right)))),
                    )
                }
            }
            Thing::UnOp(o, v) => todo!(),
            Thing::Operand(v) => panic!("What the funk"),
        }
    } else if let Some(left) = left {
        Thing::BiOp(
            Box::new(Thing::Operand(left)),
            op,
            Box::new(Thing::Operand(right)),
        )
    } else {
        panic!("Bruv");
    }
}

fn eval(op: Thing) -> Float {
    match op {
        Thing::BiOp(l, o, r) => match o {
            BiOp::Add => eval(*l) + eval(*r),
            BiOp::Sub => eval(*l) - eval(*r),
            BiOp::Mul => eval(*l) * eval(*r),
            BiOp::Div => eval(*l) / eval(*r),
            BiOp::Mod => eval(*l) % eval(*r),
            BiOp::Pow => eval(*l).pow(eval(*r)),
        },
        Thing::UnOp(o, v) => match o {
            UnOp::Neg => -eval(*v),
        },
        Thing::Operand(v) => v,
    }
}
