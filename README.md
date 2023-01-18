# Combinatrix

Parser combinator library in rust. 

one or more,
zero or more

between

map

apply

# Functions
**fn c_char(c: char) -> Parser**
    - Returns a parser which matches the supplied character

**fn c_digit() -> Parser**
    - Returns a parser which matches any numeric character

**fn c_letter() -> Parser**
    - Returns a parser which matches any alphabetic character

**fn c_sequence(parsers: Vec\<Parser>) -> Parser**
    - Returns a parser which matches a vector of parsers in order

**fn c_choice(parsers: Vec\<Parser>) -> Parser**
    - Returns a parser which matches the first parser that succeeds out of the vector.

**fn c_repeat(parser: Parser) -> Parser**
    - Matches the supplied parser until an error.

**fn c_string(target: &str) -> Parser**
    - Returns a parser which matches a sequence of each character in the target string.

**fn c_whitespace() -> Parser**
    - Matches any amount of whitespace characters