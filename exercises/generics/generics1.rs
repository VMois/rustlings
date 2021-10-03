// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// I AM NOT DONE

fn main() {
    // let mut shopping_list: Vec<String> = Vec::new();
    // shopping_list.push("milk".to_string());

    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
