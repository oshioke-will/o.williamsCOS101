use std::io;

fn main() {
    // Display the menu
    println!("=======================================");
    println!("            FOOD ORDER MENU            ");
    println!("=======================================");
    println!("| Menu                            | Price |");
    println!("|---------------------------------|-------|");
    println!("| P = Poundo Yam/Edinkaiko Soup   | 3,200 |");
    println!("| F = Fried Rice & Chicken        | 3,000 |");
    println!("| A = Amala & Ewedu Soup          | 2,500 |");
    println!("| E = Eba & Egusi Soup            | 2,000 |");
    println!("| W = White Rice & Stew           | 2,500 |");
    println!("=======================================\n");

    // Get food type from user
    println!("Enter food type (P, F, A, E, W):");
    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");
    
    let food_type = food_type.trim().to_uppercase();
    
    // Validate food type
    if !["P", "F", "A", "E", "W"].contains(&food_type.as_str()) {
        println!("Invalid food type selected!");
        return;
    }

    // Get quantity from user
    println!("Enter quantity:");
    let mut quantity = String::new();
    io::stdin()
        .read_line(&mut quantity)
        .expect("Failed to read input");
    
    let quantity: u32 = match quantity.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    // Calculate price based on food type
    let price = match food_type.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => 0, // This should never happen due to validation above
    };

    // Calculate total charges
    let mut total_charges = price * quantity;
    
    // Apply 5% discount if total is greater than 10,000
    if total_charges > 10000 {
        let discount = (total_charges as f32) * 0.05;
        total_charges = total_charges - discount as u32;
        println!("\n✨ Discount Applied: 5% discount for order above N10,000");
    }

    // Display the order summary
    println!("\n=======================================");
    println!("           ORDER SUMMARY");
    println!("=======================================");
    println!("Food Type: {}", get_food_name(&food_type));
    println!("Price per unit: N{}", price);
    println!("Quantity: {}", quantity);
    println!("Total Charges: N{}", total_charges);
    println!("=======================================");
}

// Helper function to get full food name
fn get_food_name(code: &str) -> &str {
    match code {
        "P" => "Poundo Yam/Edinkaiko Soup",
        "F" => "Fried Rice & Chicken",
        "A" => "Amala & Ewedu Soup",
        "E" => "Eba & Egusi Soup",
        "W" => "White Rice & Stew",
        _ => "Unknown",
    }
}