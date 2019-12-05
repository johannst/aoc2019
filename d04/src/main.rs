fn is_pw_valid(pw: i32) -> bool {
    let mut prev = None;
    let mut pair_seen = false;

    for digit in pw.to_string().chars().rev() {
        if let Some(prev_digit) = prev {
            // constraint 1: digits never decrease
            if digit > prev_digit {
                return false;
            }
            // constraint 2: at least one group with at least two same digits
            if digit == prev_digit {
                pair_seen = true;
            }
        }
        prev = Some(digit)
    }
    pair_seen
}

fn is_pw_valid2(pw: i32) -> bool {
    let mut prev = None;
    let mut pair_seen = false;
    let mut cnt = 1;

    for digit in pw.to_string().chars().rev() {
        if let Some(prev_digit) = prev {
            // constraint 1: digits never decrease
            if digit > prev_digit {
                return false;
            }
            if digit == prev_digit {
                cnt += 1;
            } else {
                // constraint 2: at least one group with exact two same digits
                if cnt == 2 {
                    pair_seen = true;
                }
                cnt = 1;
            }
        }
        prev = Some(digit)
    }
    pair_seen || cnt == 2
}

fn main() {
    let valid_pws = (236491..713787).filter(|&pw| is_pw_valid(pw)).count();
    println!("Part One: number of valid passwords {}", valid_pws);

    let valid_pws = (236491..713787).filter(|&pw| is_pw_valid2(pw)).count();
    println!("Part Two: number of valid passwords {}", valid_pws);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples1() {
        let validate = |pw, valid| assert_eq!(is_pw_valid(pw), valid);

        validate(111111, true);
        validate(223450, false);
        validate(123789, false);
    }

    #[test]
    fn test_examples2() {
        let validate = |pw, valid| assert_eq!(is_pw_valid2(pw), valid);

        validate(112233, true);
        validate(123444, false);
        validate(111122, true);
        validate(133345, false);
        validate(133445, true);
        validate(112345, true);
    }
}
