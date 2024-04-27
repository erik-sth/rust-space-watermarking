/// Main function to generate all possible combinations of inserting spaces into a string.
fn main() {
    let input = "Rust is installed and managed by the rustup tool.";

    // Count the number of spaces in the input string.
    let amount_of_spaces = amount_of_spaces(input);

    // Calculate the total possible outputs based on the number of spaces.
    let possible_outputs = pow(2, amount_of_spaces);

    // Print the number of spaces and possible outputs.
    println!("Amount of Spaces: {}, Possible Outputs: {}", amount_of_spaces, possible_outputs);

    // Start the backtracking process.
    backtrack(input, 0, String::new());
}

/// Recursive function to backtrack and generate all possible combinations of inserting spaces.
///
/// # Arguments
///
/// * `input` - The input string to insert spaces into.
/// * `last_modified` - The index of the last character modified in the output string.
/// * `output` - The current output string with spaces inserted.
fn backtrack(input: &str, last_modified: usize, output: String) {
    // If all characters in the input string are processed, print the output.
    if last_modified == input.len() {
        println!("{}", output);
        return;
    }
  
    // If the current character is a space, explore two branches: inserting a space or not.
    if input.chars().nth(last_modified).unwrap() == ' ' {
        let mut modified_output_with_space = output.clone();
        modified_output_with_space.push(' ');
        modified_output_with_space.push(' ');
        backtrack(input, last_modified + 1, modified_output_with_space);
    }
    
    // Always explore the branch of not inserting a space.
    let mut modified_output_without_space = output.clone();
    modified_output_without_space.push(input.chars().nth(last_modified).unwrap());
    backtrack(input, last_modified + 1, modified_output_without_space);
}

/// Counts the number of spaces in a given string.
///
/// # Arguments
///
/// * `input` - The input string to count spaces in.
///
/// # Returns
///
/// The number of spaces in the input string.
fn amount_of_spaces(input: &str) -> usize {
    input.chars().filter(|&c| c == ' ').count()
}

/// Calculates the power of a base raised to an exponent.
///
/// # Arguments
///
/// * `base` - The base value.
/// * `exp` - The exponent value.
///
/// # Returns
///
/// The result of raising the base to the exponent.
fn pow(base: u32, exp: usize) -> u32 {
    let mut result = 1;
    for _ in 0..exp {
        result *= base;
    }
    result
}
