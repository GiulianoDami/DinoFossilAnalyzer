use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Fossil {
    pub id: String,
    pub species: String,
    pub location: String,
    pub date_found: String,
    pub anomaly_score: f64,
}

pub fn load_fossil_from_file(filename: &str) -> Result<Fossil, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let fossil: Fossil = serde_json::from_reader(file)?;
    Ok(fossil)
}

pub fn save_report_to_file(filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn print_report(fossil: &Fossil) {
    println!("Fossil Report:");
    println!("  ID: {}", fossil.id);
    println!("  Species: {}", fossil.species);
    println!("  Location: {}", fossil.location);
    println!("  Date Found: {}", fossil.date_found);
    println!("  Anomaly Score: {:.2}", fossil.anomaly_score);
}