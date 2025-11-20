use std::collections::HashMap;

fn main() {
    println!("=== EY Nigeria Developer Experience Finder ===");
    
    // Create a vector to store candidate information using a tuple struct
    let mut candidates: Vec<Candidate> = Vec::new();
    
    // Add some sample candidates
    candidates.push(Candidate::new("John Smith", "Rust", 5));
    candidates.push(Candidate::new("Sarah Johnson", "Python", 8));
    candidates.push(Candidate::new("Mike Brown", "Java", 3));
    candidates.push(Candidate::new("Lisa Davis", "Rust", 7));
    candidates.push(Candidate::new("David Wilson", "JavaScript", 6));
    
    // Display all candidates
    println!("\nAll Candidates:");
    for candidate in &candidates {
        println!("{}", candidate);
    }
    
    // Find candidate with highest experience
    if let Some(highest_exp_candidate) = find_highest_experience(&candidates) {
        println!("\n=== SELECTED CANDIDATE ===");
        println!("Developer with highest programming experience:");
        println!("{}", highest_exp_candidate);
        println!("Years of Experience: {}", highest_exp_candidate.years_experience);
    } else {
        println!("No candidates found!");
    }
    
    // Alternative approach using HashMap
    println!("\n=== Alternative Approach Using HashMap ===");
    let mut candidate_map = HashMap::new();
    candidate_map.insert("Alice Chen", 9);
    candidate_map.insert("Bob Miller", 4);
    candidate_map.insert("Carol White", 11);
    candidate_map.insert("Daniel Lee", 6);
    
    // Find max experience in HashMap
    if let Some((&name, &experience)) = candidate_map.iter()
        .max_by_key(|&(_, &exp)| exp) {
        println!("Highest experience candidate (HashMap approach):");
        println!("Name: {}, Years of Experience: {}", name, experience);
    }
}

// Define a struct to represent a candidate
#[derive(Debug)]
struct Candidate {
    name: String,
    programming_language: String,
    years_experience: u32,
}

impl Candidate {
    fn new(name: &str, language: &str, experience: u32) -> Self {
        Candidate {
            name: name.to_string(),
            programming_language: language.to_string(),
            years_experience: experience,
        }
    }
}

// Implement Display trait for nice printing
impl std::fmt::Display for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {} ({} years)", self.name, self.programming_language, self.years_experience)
    }
}

// Function to find candidate with highest experience
fn find_highest_experience(candidates: &[Candidate]) -> Option<&Candidate> {
    candidates.iter()
        .max_by_key(|candidate| candidate.years_experience)
}