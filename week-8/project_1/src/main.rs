use std::collections::HashMap;

fn main() {
    // Create vectors for different professions and their levels
    let mut aps_levels: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();
    
    // Office Administrator levels
    aps_levels.insert("Office Administrator", vec![
        ("APS 1-2", "Intern"),
        ("APS 3-5", "Administrator"),
        ("APS 5-8", "Senior Administrator"),
        ("EL1 8-10", "Office Manager"),
        ("EL2 10-13", "Director"),
        ("SES", "CEO")
    ]);
    
    // Academic levels
    aps_levels.insert("Academic", vec![
        ("APS 1-2", "-"),
        ("APS 3-5", "Research Assistant"),
        ("APS 5-8", "PhD Candidate"),
        ("EL1 8-10", "Post-Doc Researcher"),
        ("EL2 10-13", "Senior Lecturer"),
        ("SES", "Dean")
    ]);
    
    // Lawyer levels
    aps_levels.insert("Lawyer", vec![
        ("APS 1-2", "Paralegal"),
        ("APS 3-5", "Junior Associate"),
        ("APS 5-8", "Associate"),
        ("EL1 8-10", "Senior Associate 1-2"),
        ("EL2 10-13", "Senior Associate 3-4"),
        ("SES", "Partner")
    ]);
    
    // Teacher levels
    aps_levels.insert("Teacher", vec![
        ("APS 1-2", "Placement"),
        ("APS 3-5", "Classroom Teacher"),
        ("APS 5-8", "Senior Teacher"),
        ("EL1 8-10", "Leading Teacher"),
        ("EL2 10-13", "Deputy Principal"),
        ("SES", "Principal")
    ]);

    println!("=== Public Service APS Level Checker ===");
    
    // Get user input
    let mut profession = String::new();
    println!("Enter profession (Office Administrator, Academic, Lawyer, Teacher):");
    std::io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim();
    
    let mut experience = String::new();
    println!("Enter years of experience:");
    std::io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience: u32 = experience.trim().parse().expect("Please enter a valid number");

    // Validate staff level based on experience
    if let Some(levels) = aps_levels.get(profession) {
        let position = match experience {
            0..=2 => &levels[0],   // APS 1-2
            3..=5 => &levels[1],   // APS 3-5
            6..=8 => &levels[2],   // APS 5-8
            9..=10 => &levels[3],  // EL1 8-10
            11..=13 => &levels[4], // EL2 10-13
            _ => &levels[5]        // SES
        };
        
        println!("\nStaff Validation Result:");
        println!("Profession: {}", profession);
        println!("Years of Experience: {}", experience);
        println!("APS Level: {}", position.0);
        println!("Position: {}", position.1);
    } else {
        println!("Invalid profession entered!");
    }
}