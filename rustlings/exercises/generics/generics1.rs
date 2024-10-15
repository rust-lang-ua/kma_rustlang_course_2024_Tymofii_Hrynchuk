// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

// I AM DONE

fn add_to_list<T>(item: T, list: &mut Vec<T>) {
    list.push(item);
}

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    add_to_list("milk", &mut shopping_list);
}
