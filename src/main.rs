fn main() {
    let sample = String::from("Hello, world");

    let c = c_repeat(c_char('H'));

    let result = c(&sample);
    match result {
        Ok((remaining, matched)) => println!(
            "OK:\nRemaining input: {}\nMatched chars: {:?}",
            remaining, matched
        ),
        Err(e) => println!("ERR {}", e),
    }
}

fn c_char(c: char) -> impl Fn(&str) -> Result<(&str, Vec<char>), &str> {
    move |input: &str| -> Result<(&str, Vec<char>), &str> {
        if input.starts_with(c) {
            Ok((&input[1..], vec![c]))
        } else {
            Err(input)
        }
    }
}

fn c_sequence<T>(parsers: Vec<T>) -> impl Fn(&str) -> Result<(&str, Vec<char>), &str>
where
    T: Fn(&str) -> Result<(&str, Vec<char>), &str>,
{
    move |input: &str| -> Result<(&str, Vec<char>), &str> {
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
    }
}

fn c_choice<T>(parsers: Vec<T>) -> impl Fn(&str) -> Result<(&str, Vec<char>), &str>
where
    T: Fn(&str) -> Result<(&str, Vec<char>), &str>
{
    move |input: &str| -> Result<(&str, Vec<char>), &str> {
        for parser in &parsers {
            if let Ok(x) = parser(input) {
                return Ok(x);
            }
        }
        Err(input)
    }
}

fn c_string(target: &str) -> impl Fn(&str) -> Result<(&str, Vec<char>), &str> {
    let mut parsers = vec![];
    for letter in target.chars() {
        parsers.push(c_char(letter));
    }
    c_sequence(parsers)
}

fn c_repeat<T>(parser: T) -> impl Fn(&str) -> Result<(&str, Vec<char>), &str> 
where
    T: Fn(&str) -> Result<(&str, Vec<char>), &str>
{
    move |input: &str| -> Result<(&str, Vec<char>), &str> {
        let mut remaining = &input[..];
        let mut matched = vec![];
        while let Ok((rem, mut matched_char)) = parser(remaining) {
            remaining = rem;
            matched.append(&mut matched_char)
        }
        Ok((remaining, matched))
    }    
}

fn c_whitespace() -> impl Fn(&str) -> Result<(&str, Vec<char>), &str> {
    c_choice(vec![
        c_char(' '),
        c_char('\n'),
        c_char('\t')
    ])
}
