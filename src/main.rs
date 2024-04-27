fn main() {
    let input = "Rust is installed and managed by the rustup tool.";
    let amount_of_spaces = amount_of_spaces(input);
    let possible_outputs = pow(2, amount_of_spaces);
    println!("Amount of Spaces: {}, Possible Outputs: {}", amount_of_spaces, possible_outputs);
    backtrack(input, 0, String::new());
}

fn backtrack(input: &str, last_modified: usize, output: String) {
    if last_modified == input.len() {
        println!("{}", output);
        return;
    }

    for (index, character) in input.chars().enumerate().skip(last_modified) {
        if character == ' ' {
            let mut modified_output = output.clone();
            modified_output.push(' ');
            backtrack(input, index + 1, modified_output);
        }
        let mut modified_output = output.clone();
        modified_output.push(' ');
        backtrack(input, index + 1, modified_output);
    }
}

fn amount_of_spaces(input: &str) -> usize {
    input.chars().filter(|&c| c == ' ').count()
}

fn pow(base: u32, exp: usize) -> u32 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}
