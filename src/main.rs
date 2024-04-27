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
    let mut outputs = Vec::new(); // Initialize an empty vector to store outputs
    backtrack(&input, 0, String::new(), &mut outputs);

    // Print or process the stored outputs as needed
    for output in outputs {
        println!("{}", output);
    }
}

fn backtrack(input: &str, last_modified: usize, output: String, results: &mut Vec<String>) {
    // If all characters in the input string are processed, push the output into the results vector
    if last_modified == input.len() {
        results.push(output);
        return;
    }
  
    // If the current character is a space, explore two branches: inserting a space or not
    if input.chars().nth(last_modified).unwrap() == ' ' {
        let mut modified_output_with_space = output.clone();
        modified_output_with_space.push_str("  "); // Append two spaces
        backtrack(input, last_modified + 1, modified_output_with_space, results);
    }
    
    // Always explore the branch of not inserting a space
    let mut modified_output_without_space = output.clone();
    modified_output_without_space.push(input.chars().nth(last_modified).unwrap());
    backtrack(input, last_modified + 1, modified_output_without_space, results);
}

fn count_spaces(input: &str) -> usize {
    input.chars().filter(|&c| c == ' ').count()
}

fn calculate_possible_outputs(amount_of_spaces: usize) -> u32 {
    u32::pow(2, amount_of_spaces as u32)
}
