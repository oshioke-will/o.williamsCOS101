fn main() {
    println!("=============================");
    println!("   🦀 Rust Restaurant Menu   ");
    println!("=============================");
    println!("Menu\t\t\t\tPrice (₦)");
    println!("P = Poundo Yam & Edinkaiko Soup\t₦3,200");
    println!("F = Fried Rice & Chicken\t₦3,000");
    println!("A = Amala & Ewedu Soup\t\t₦2,500");
    println!("E = Eba & Egusi Soup\t\t₦2,000");
    println!("W = White Rice & Stew\t\t₦2,500");
    println!("=============================");

    // Ask user for food choice
    println!("Enter your meal choice (P, F, A, E, W):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim().to_uppercase();

    // Determine the selected meal and its price
    let (meal, price) = match choice.as_str() {
        "P" => ("Poundo Yam & Edinkaiko Soup", 3200),
        "F" => ("Fried Rice & Chicken", 3000),
        "A" => ("Amala & Ewedu Soup", 2500),
        "E" => ("Eba & Egusi Soup", 2000),
        "W" => ("White Rice & Stew", 2500),
        _ => {
            println!("Invalid choice. Please run the program again.");
            return;
        }
    };

    println!("You selected: {}", meal);

    // Ask how many plates
    println!("Enter number of plates:");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    let qty: i32 = qty_input.trim().parse().unwrap_or(0);

    let total = price * qty;
    println!("=============================");
    println!("Meal: {}", meal);
    println!("Quantity: {}", qty);
    println!("Total Price: ₦{}", total);
    println!("=============================");
    println!("Thank you for your order! 🍽️");
}