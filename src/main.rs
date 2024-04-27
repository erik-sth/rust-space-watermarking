fn main() {
    // Define the input string
    let input = "Rust is installed and managed by the rustup tool.";

    // Count the number of spaces in the input string
    let amount_of_spaces = count_spaces(input);

    // Calculate the total possible outputs based on the number of spaces
    let amount_outputs = calculate_amount_outputs(amount_of_spaces);

    // Print the number of spaces and possible outputs
    println!("Amount of Spaces: {}, Possible Outputs: {}", amount_of_spaces, amount_outputs);

    // Perform backtracking and calculate the identifying number
    let mut outputs = Vec::new(); // Initialize an empty vector to store outputs
    backtrack_and_collect(&input, 0, String::new(), &mut outputs);
    calculate_identifying_number(&outputs[4], amount_of_spaces);
}

// Backtracking function to generate all possible outputs
fn backtrack_and_collect(input: &str, index: usize, output: String, results: &mut Vec<String>) {
    // If all characters in the input string are processed, push the output into the results vector
    if index == input.len() {
        results.push(output);
        return;
    }
  
    // If the current character is a space, explore two branches: inserting a space or not
    if input.chars().nth(index).unwrap() == ' ' {
        let mut modified_output_with_space = output.clone();
        modified_output_with_space.push_str("  "); // Append two spaces
        backtrack_and_collect(input, index + 1, modified_output_with_space, results);
    }
    
    // Always explore the branch of not inserting a space
    let mut modified_output_without_space = output.clone();
    modified_output_without_space.push(input.chars().nth(index).unwrap());
    backtrack_and_collect(input, index + 1, modified_output_without_space, results);
}

// Calculate the identifying number based on the pattern of double spaces
fn calculate_identifying_number(input: &str, amount_of_spaces: usize) -> u32 {
    let mut double_spaces = Vec::new();  // Renamed from double_space_indices

    // Find the positions of consecutive double spaces
    let mut chars = input.chars().peekable();
    while let Some(character) = chars.next() {
        if character == ' ' && chars.peek() == Some(&' ') {
            double_spaces.push(true);
            chars.next(); // Skip the next space
        } else {
            double_spaces.push(false);
        }
    }

    // Calculate the identifying number based on the pattern of double spaces
    let mut identifier = 0;    
    for (index, is_double_space) in double_spaces.iter().enumerate() {
        if *is_double_space {    
            identifier += calculate_left_side_end_nodes(index as u32, amount_of_spaces as u32);
        }
    }
    identifier
}

// Calculate the number of spaces in the input string
fn count_spaces(input: &str) -> usize {
    input.chars().filter(|&c| c == ' ').count()
}

// Calculate the total possible outputs based on the number of spaces
fn calculate_amount_outputs(amount_of_spaces: usize) -> u32 {
    u32::pow(2, amount_of_spaces as u32)
}

// Calculate the number of end nodes on the left side of the tree at a given depth
fn calculate_left_side_end_nodes(index: u32, depth: u32) -> u32 {
    u32::pow(2, depth - index - 1) // -1 to go one tree node down
}
