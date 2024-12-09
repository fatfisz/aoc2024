use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut input = input.as_str();
    let mut result = 0;

    while input.len() > 0 {
        match input.find("mul(") {
            Some(index) => input = &input[index + "mul(".len()..],
            None => break,
        }

        let fancy_input = consume_number(input)
            .and_then(|(first, input)| consume_character(input, ',').map(|input| (first, input)))
            .and_then(|(first, input)| {
                consume_number(input).map(|(second, input)| (first, second, input))
            })
            .and_then(|(first, second, input)| {
                consume_character(input, ')').map(|input| (first, second, input))
            })
            .map(|(first, second, input)| {
                result += first * second;
                input
            });

        input = match fancy_input {
            Ok(i) => i,
            Err(i) => i,
        };
    }

    println!("{result}");
}

fn consume_number(input: &str) -> Result<(i32, &str), &str> {
    let digit_count = input.chars().take_while(|c| c.is_digit(10)).count();
    if digit_count > 0 && digit_count <= 3 {
        Ok((
            (&input[0..digit_count]).parse().unwrap(),
            &input[digit_count..],
        ))
    } else {
        Err(&input[digit_count..])
    }
}

fn consume_character(input: &str, character: char) -> Result<&str, &str> {
    if input.chars().next() == Some(character) {
        Ok(&input[1..])
    } else {
        Err(input)
    }
}
