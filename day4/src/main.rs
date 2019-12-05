const INPUT_MIN: usize = 245182;
const INPUT_MAX: usize = 790572;

fn main() {
    let mut candidates = 0;
    for i in INPUT_MIN ..= INPUT_MAX {
        if let Ok(_) = is_candidate(i) {
            candidates += 1;
        }
    }
    println!("found {} candidates", candidates);
}

#[derive(Debug, PartialEq)]
enum Rejection {
    WrongNumberOfDigits,
    OutOfRange,
    NoRepetition,
    DigitsDecreased,
}

fn is_candidate(number: usize) -> Result<usize, Rejection> {
    let digits: Vec<char> = format!("{}", number).chars().collect();

    if digits.len() != 6 {
        return Err(Rejection::WrongNumberOfDigits);
    }

    // if number < INPUT_MIN || number > INPUT_MAX {
    //     return Err(Rejection::OutOfRange);
    // }

    let mut found_repetition = false;
    let mut run_start = 0;
    for (i, digit) in digits.iter().enumerate() {
        if i > 0 {
            let prev = digits[i - 1];
            if prev != *digit {
                if i - run_start == 2 {
                    found_repetition = true;
                }
                run_start = i;
            }
            if *digit < prev {
                return Err(Rejection::DigitsDecreased);
            }
        }
    }
    if digits.len() - run_start == 2 {
        found_repetition = true;
    }

    if !found_repetition {
        return Err(Rejection::NoRepetition);
    }

    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn givens() {
        assert_eq!(is_candidate(112233), Ok(112233));
        assert_eq!(is_candidate(123444), Err(Rejection::NoRepetition));
        assert_eq!(is_candidate(111122), Ok(111122));

        assert_eq!(is_candidate(111111), Err(Rejection::NoRepetition));
        assert_eq!(is_candidate(223450), Err(Rejection::DigitsDecreased));
        assert_eq!(is_candidate(123789), Err(Rejection::NoRepetition));
    }
}
