use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut result = 0;
    let mut enabled = true;
    let mut parser = Parser::new(&input);

    while parser.input.len() > 0 {
        match true {
            _ if parser.try_consume("do") => match true {
                _ if parser.try_consume("()") => enabled = true,
                _ if parser.try_consume("n't()") => enabled = false,
                _ => parser.next(),
            },
            _ if parser.try_consume("mul(") => {
                parser.consume_number();
                parser.consume_character(',');
                parser.consume_number();
                parser.consume_character(')');
                parser.and_then(|numbers| {
                    if enabled {
                        result += &numbers.iter().product();
                    }
                });
                parser.reset();
            }
            _ => parser.next(),
        }
    }

    println!("{result}");
}

#[derive(Debug)]
struct Parser<'a> {
    input: &'a str,
    state: Result<Vec<i32>, ()>,
}

impl Parser<'_> {
    fn new(input: &str) -> Parser {
        Parser {
            input,
            state: Ok(vec![]),
        }
    }

    fn try_consume(&mut self, string: &str) -> bool {
        if self.input.starts_with(string) {
            self.input = &self.input[string.len()..];
            true
        } else {
            false
        }
    }

    fn consume_number(&mut self) {
        if let Ok(numbers) = &mut self.state {
            let digit_count = self.input.chars().take_while(|c| c.is_digit(10)).count();
            if digit_count > 0 && digit_count <= 3 {
                numbers.push(self.input[0..digit_count].parse().unwrap());
            } else {
                self.state = Err(());
            }
            self.input = &self.input[digit_count..];
        }
    }

    fn consume_character(&mut self, character: char) {
        if self.state.is_ok() {
            if self.input.chars().next() == Some(character) {
                self.input = &self.input[1..];
            } else {
                self.state = Err(());
            }
        }
    }

    fn reset(&mut self) {
        self.state = Ok(vec![]);
    }

    fn and_then<F: FnOnce(&[i32]) -> ()>(&mut self, op: F) {
        if let Ok(numbers) = &self.state {
            op(numbers)
        }
    }

    fn next(&mut self) {
        self.input = &self.input[1..];
    }
}
