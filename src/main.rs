fn main() {
    let sample = String::from("Hello, world");

    let hello = c_string("Hello,");
    let world = c_string(" world");
    let c = c_sequence(vec![hello, world]);





    let result = c(&sample);
    match result {
        Ok((remaining, matched)) => println!("OK:\nRemaining input: {},\nmatched chars: {:#?}", remaining, matched),
        Err(e) => println!("ERR {}", e)
    }
}

fn c_char(c: char) -> impl Fn(&str) -> Result<(&str, char), &str> {
    move |input: &str| -> Result<(&str, char), &str> {
        if input.starts_with(c) {
            Ok((&input[1..], c))
        } else {
            Err(input)
        }
    }
}

fn c_sequence<T>(parsers: Vec<T>) -> impl Fn(&str) -> Result<(&str, Vec<()>), &str>
where
    T: Fn(&str) -> Result<(&str, ()), &str>,
{
    move |input: &str| -> Result<(&str, Vec<()>), &str> {
        let mut remaining = &input[..];
        let mut matched = vec![];
        for parser in &parsers {
            if let Ok((rem, matched_char)) = parser(remaining) {
                remaining = rem;
                matched.push(matched_char);
            } else {
                return Err(input);
            }
        }
        Ok((remaining, matched))
    }
}

fn c_choice<T>(parsers: Vec<T>) -> impl Fn(&str) -> Result<(&str, ()), &str>
where
    T: Fn(&str) -> Result<(&str, ()), &str>
{
    move |input: &str| -> Result<(&str, ()), &str> {
        for parser in &parsers {
            if let Ok(x) = parser(input) {
                return Ok(x);
            }
        }
        Err(input)
    }
}

fn c_string(target: &str) -> impl Fn(&str) -> Result<(&str, Vec<()>), &str> {
    let mut parsers = vec![];
    for letter in target.chars() {
        parsers.push(c_char(letter));
    }
    c_sequence(parsers)
}

// fn c_whitespace() -> impl Fn(&str) -> Result<(&str, ()), &str> {
//     c_choice(vec![
//         c_char(' '),
//         c_char('\n'),
//         c_char('\t')
//     ])
// }