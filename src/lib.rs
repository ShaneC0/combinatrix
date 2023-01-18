

pub type Parser<'a> = Box<dyn Fn(&'a str) -> Result<(&'a str, Vec<char>), &'a str> + 'a>;

pub fn c_char<'a>(c: char) -> Parser<'a> {
    Box::new(
        move |input: &'a str| -> Result<(&'a str, Vec<char>), &'a str> {
            if input.starts_with(c) {
                Ok((&input[1..], vec![c]))
            } else {
                Err(input)
            }
        },
    )
}

pub fn c_letter<'a>() -> Parser<'a> {
    Box::new(move |input: &'a str| -> Result<(&'a str, Vec<char>), &'a str> {
        if input.chars().next().unwrap().is_alphabetic() {
            Ok((&input[1..], vec![input.chars().next().unwrap()]))
        } else {
            Err(input)
        }
    })
}

pub fn c_digit<'a>() -> Parser<'a> {
    Box::new(move |input: &str| -> Result<(&str, Vec<char>), &str> {
        if input.chars().next().unwrap().is_numeric() {
            Ok((&input[1..], vec![input.chars().next().unwrap()]))
        } else {
            Err(input)
        }
    })
}

pub fn c_sequence<'a>(parsers: Vec<Parser<'a>>) -> Parser<'a> {
    Box::new(move |input: &'a str| -> Result<(&'a str, Vec<char>), &'a str> {
        let mut remaining = &input[..];
        let mut matched = vec![];
        for parser in &parsers {
            if let Ok((rem, mut matched_char)) = parser(remaining) {
                remaining = rem;
                matched.append(&mut matched_char);
            } else {
                return Err(input);
            }
        }
        Ok((remaining, matched))
    })
}

pub fn c_choice<'a>(parsers: Vec<Parser<'a>>) -> Parser<'a> {
    Box::new(move |input: &'a str| -> Result<(&'a str, Vec<char>), &'a str> {
        for parser in &parsers {
            if let Ok(x) = parser(input) {
                return Ok(x);
            }
        }
        Err(input)
    })
}

pub fn c_string<'a>(target: &'a str) -> Parser<'a> {
    let mut parsers = vec![];
    for letter in target.chars() {
        parsers.push(c_char(letter));
    }
    c_sequence(parsers)
}

pub fn c_repeat<'a>(parser: Parser<'a>) -> Parser<'a> {
    Box::new(move |input: &'a str| -> Result<(&'a str, Vec<char>), &'a str> {
        let mut remaining = &input[..];
        let mut matched = vec![];
        while let Ok((rem, mut matched_char)) = parser(remaining) {
            remaining = rem;
            matched.append(&mut matched_char)
        }
        Ok((remaining, matched))
    })
}

pub fn c_whitespace<'a>() -> Parser<'a> {
    c_repeat(c_choice(vec![c_char(' '), c_char('\n'), c_char('\t')]))
}
