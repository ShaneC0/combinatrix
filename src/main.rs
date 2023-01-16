fn main() {
    let sample = String::from("Hello, world!");

    let hello_parser = c_sequence(vec![
        c_char('H'),
    ]);

    match hello_parser(&sample) {
        Ok((x, _)) => println!("Ok {}", x),
        Err(x) => println!("Err {}", x)
    }
}

fn c_char(c: char) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input: &str| -> Result<(&str, ()), &str> {
        if input.starts_with(c) {
            Ok((&input[1..], ()))
        } else {
            Err(input)
        }
    }
}

fn c_or<T>(parser_a: T, parser_b: T) -> impl Fn(&str) -> Result<(&str, ()), &str>
where
    T: Fn(&str) -> Result<(&str, ()), &str>,
{
    move |input: &str| -> Result<(&str, ()), &str> {
        if let Ok(x) = parser_a(input) {
            return Ok(x);
        }
        if let Ok(x) = parser_b(input) {
            return Ok(x);
        }
        Err(input)
    }
}

fn c_sequence<T>(parsers: Vec<T>) -> impl Fn(&str) -> Result<(&str, ()), &str>
where
    T: Fn(&str) -> Result<(&str, ()), &str>,
{
    move |input: &str| -> Result<(&str, ()), &str> {
        let mut remaining = &input[..];
        for parser in &parsers {
            if let Ok((rem, _)) = parser(remaining) {
                remaining = rem;
            } else {
                return Err(input);
            }
        }
        Ok((remaining, ()))
    }
}
