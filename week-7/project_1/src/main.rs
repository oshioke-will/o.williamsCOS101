use std::f64::consts::PI;

fn main() {
    println!("Shape Calculator");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    
    let mut choice = String::new();
    println!("Enter your choice (1-5):");
    std::io::stdin().read_line(&mut choice).expect("Failed to read input");
    
    match choice.trim().parse::<u32>() {
        Ok(1) => calculate_trapezium_area(),
        Ok(2) => calculate_rhombus_area(),
        Ok(3) => calculate_parallelogram_area(),
        Ok(4) => calculate_cube_area(),
        Ok(5) => calculate_cylinder_volume(),
        _ => println!("Invalid choice!"),
    }
}

fn calculate_trapezium_area() {
    let height = get_input("Enter height:");
    let base1 = get_input("Enter base1:");
    let base2 = get_input("Enter base2:");
    let area = height / 2.0 * (base1 + base2);
    println!("Area of Trapezium: {}", area);
}

fn calculate_rhombus_area() {
    let diagonal1 = get_input("Enter diagonal1:");
    let diagonal2 = get_input("Enter diagonal2:");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of Rhombus: {}", area);
}

fn calculate_parallelogram_area() {
    let base = get_input("Enter base:");
    let altitude = get_input("Enter altitude:");
    let area = base * altitude;
    println!("Area of Parallelogram: {}", area);
}

fn calculate_cube_area() {
    let side = get_input("Enter side length:");
    let area = 6.0 * side * side;
    println!("Surface Area of Cube: {}", area);
}

fn calculate_cylinder_volume() {
    let radius = get_input("Enter radius:");
    let height = get_input("Enter height:");
    let volume = PI * radius * radius * height;
    println!("Volume of Cylinder: {}", volume);
}

fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid number")
}