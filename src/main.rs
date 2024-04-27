/// Generates all possible combinations of inserting spaces into a string.
fn main() {
    // Input string to work with
    let input = "Rust is installed and managed by the rustup tool.";

    // Count the number of spaces in the input string
    let amount_of_spaces = count_spaces(input);

    // Calculate the total possible outputs based on the number of spaces
    let possible_outputs = calculate_possible_outputs(amount_of_spaces);

    // Print the number of spaces and possible outputs
    println!("Amount of Spaces: {}, Possible Outputs: {}", amount_of_spaces, possible_outputs);

    // Start the backtracking process
    backtrack(&input, 0, String::new());
}

/// Recursively generates all possible combinations of inserting spaces.
///
/// # Arguments
///
/// * `input` - The input string to insert spaces into.
/// * `last_modified` - The index of the last character modified in the output string.
/// * `output` - The current output string with spaces inserted.
fn backtrack(input: &str, last_modified: usize, output: String) {
    // If all characters in the input string are processed, print the output
    if last_modified == input.len() {
        println!("{}", output);
        return;
    }
  
    // If the current character is a space, explore two branches: inserting a space or not
    if input.chars().nth(last_modified).unwrap() == ' ' {
        let mut modified_output_with_space = output.clone();
        modified_output_with_space.push_str("  "); // Append two spaces
        backtrack(input, last_modified + 1, modified_output_with_space);
    }
    
    // Always explore the branch of not inserting a space
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
fn count_spaces(input: &str) -> usize {
    input.chars().filter(|&c| c == ' ').count()
}

/// Calculates the total possible combinations based on the number of spaces.
///
/// # Arguments
///
/// * `amount_of_spaces` - The number of spaces in the input string.
///
/// # Returns
///
/// The total possible combinations of inserting spaces.
fn calculate_possible_outputs(amount_of_spaces: usize) -> u32 {
    u32::pow(2, amount_of_spaces as u32)
}
