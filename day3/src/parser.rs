use crate::types::{Direction, WireVector};

pub fn parse_vectors(input: &str) -> Result<(&str, Vec<WireVector>), &str> {
    let mut vectors = Vec::new();
    let mut next = input;
    while !next.is_empty() {
        let (rest, _) = eat_separators(next)?;
        let (rest, vec) = parse_vector(rest)?;
        next = rest;
        vectors.push(vec);
    }
    Ok((next, vectors))
}

pub fn eat_separators(input: &str) -> Result<(&str, ()), &str> {
    let skipped: usize = input
        .chars()
        .take_while(|c| c.is_ascii_whitespace() || *c == ',')
        .count();
    Ok((&input[skipped ..], ()))
}

pub fn parse_vector(input: &str) -> Result<(&str, WireVector), &str> {
    let (next, direction) = parse_direction(input)?;
    let (next, magnitude) = parse_magnitude(next)?;
    let cmd = WireVector {
        direction,
        magnitude,
    };
    Ok((next, cmd))
}

pub fn parse_direction(input: &str) -> Result<(&str, Direction), &str> {
    let direction = match input.chars().nth(0) {
        Some('U') => Direction::Up,
        Some('D') => Direction::Down,
        Some('R') => Direction::Right,
        Some('L') => Direction::Left,
        _ => return Err(input),
    };

    Ok((&input[1 ..], direction))
}

pub fn parse_magnitude(input: &str) -> Result<(&str, i64), &str> {
    let input_digits: String =
        input.chars().take_while(char::is_ascii_digit).collect();
    if input_digits.is_empty() {
        return Err(input);
    }

    if let Ok(number) = input_digits.parse() {
        let used_chars = input_digits.len();
        return Ok((&input[used_chars ..], number));
    }

    Err(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let expected = WireVector {
            direction: Direction::Right,
            magnitude: 58,
        };
        assert_eq!(parse_vector("R58"), Ok(("", expected)));

        let expected = WireVector {
            direction: Direction::Left,
            magnitude: 16,
        };
        assert_eq!(parse_vector("L16"), Ok(("", expected)));

        let expected = WireVector {
            direction: Direction::Up,
            magnitude: 12000,
        };
        assert_eq!(parse_vector("U12000"), Ok(("", expected)));

        let expected = WireVector {
            direction: Direction::Down,
            magnitude: 0,
        };
        assert_eq!(parse_vector("D0"), Ok(("", expected)));
    }

    #[test]
    fn vsmoke() {
        let e1 = WireVector {
            direction: Direction::Right,
            magnitude: 58,
        };
        let e2 = WireVector {
            direction: Direction::Left,
            magnitude: 16,
        };
        let e3 = WireVector {
            direction: Direction::Up,
            magnitude: 12000,
        };
        let e4 = WireVector {
            direction: Direction::Down,
            magnitude: 0,
        };
        let expected = vec![e1, e2, e3, e4];
        assert_eq!(parse_vectors("R58, L16, U12000, D0"), Ok(("", expected)));
    }
}
