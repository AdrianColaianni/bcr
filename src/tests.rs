#[cfg(test)]
mod tests {
    use rug::Float;

    use crate::{eval, parse_input, PRECISION};

    #[test]
    fn order_of_ops() {
        assert_eq!(
            7,
            eval(parse_input(&"1+2*3".chars().collect::<Vec<char>>()[..]).unwrap())
        );

        assert_eq!(
            -1,
            eval(parse_input(&"1*2-3".chars().collect::<Vec<char>>()[..]).unwrap())
        );

        assert_eq!(
            9,
            eval(parse_input(&"1+2^3".chars().collect::<Vec<char>>()[..]).unwrap())
        );

        assert_eq!(
            Float::with_val(PRECISION, 0.1),
            eval(parse_input(&"0.1 + 1534 % 10".chars().collect::<Vec<char>>()[..]).unwrap())
        );
    }

    #[test]
    fn parenthensis() {
        assert_eq!(
            2,
            eval(parse_input(&"(1+1)".chars().collect::<Vec<char>>()[..]).unwrap())
        );

        assert_eq!(
            3,
            eval(parse_input(&"1+(1+1)".chars().collect::<Vec<char>>()[..]).unwrap())
        );

        assert_eq!(
            27,
            eval(parse_input(&"(1+2)^3".chars().collect::<Vec<char>>()[..]).unwrap())
        );

        assert_eq!(
            3,
            eval(parse_input(&"1*(2+1)".chars().collect::<Vec<char>>()[..]).unwrap())
        );
    }
}
