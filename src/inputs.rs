use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have the correct number of arguments
    if args.len() != 3 {
        println!("Please provide exactly TWO parameters");
        println!("Usage: {} <param1> <param2>", args[0]);
        return;
    }

    // Get the parameters
    let param1 = &args[1];
    let param2 = &args[2];

    // Print the parameters
    println!("First parameter: {}", param1);
    println!("Second parameter: {}", param2);
} 
