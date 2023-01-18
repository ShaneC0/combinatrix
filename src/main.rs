use combinatrix::*;

fn main() {
    let type_specifier = c_choice(vec![
        c_string("int"),
        c_string("float"),
        c_string("bool"),
        c_string("string"),
    ]);

    let init = c_string("init");
    let halt = c_string("halt");
    let ident = c_repeat(c_choice(vec![c_letter(), c_char('_')]));
    let semicol = c_char(';');

    let sample = "1026";
    let parser = c_repeat(c_digit());
    match parser(&sample) {
        Ok((remaining, matched)) => println!(
            "OK:\nRemaining input: {}\nMatched chars: {:?}",
            remaining, matched
        ),
        Err(remaining) => println!("Err:\nRemaining input: {}", remaining),
    }
}
