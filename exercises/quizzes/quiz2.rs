pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
    let mut result = Vec::new();
    for (s, command) in input {
        match command {
            Command::Uppercase => {
                result.push(s.to_uppercase());
            },
            Command::Trim => {
                result.push(s.trim().to_string());
            },
            Command::Append(n) => {
                let mut new_s = s.clone();
                for _ in 0..n {
                    new_s.push_str("bar");
                }
                result.push(new_s);
            },
        }
    }
    result
}

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

fn main() {}
