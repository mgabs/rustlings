// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    (2..).take_while(|&i| i <= num).product()
}

pub fn rec_factorial(num: u64) -> u64 {
    if num == 1 {
        return 1;
    }
    num * factorial(num - 1)
}

pub fn lp_factorial(num: u64) -> u64 {
    let mut result: u64 = 1;
    for i in 1..num + 1 {
        result *= i;
        print!("{}", result);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }

    #[test]
    fn factorial_of_1_2() {
        assert_eq!(1, rec_factorial(1));
    }
    #[test]
    fn factorial_of_2_2() {
        assert_eq!(2, rec_factorial(2));
    }

    #[test]
    fn factorial_of_4_2() {
        assert_eq!(24, lp_factorial(4));
    }

    #[test]
    fn factorial_of_1_3() {
        assert_eq!(1, lp_factorial(1));
    }
    #[test]
    fn factorial_of_2_3() {
        assert_eq!(2, lp_factorial(2));
    }

    #[test]
    fn factorial_of_4_3() {
        assert_eq!(24, lp_factorial(4));
    }
}
