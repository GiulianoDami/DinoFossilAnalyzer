use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct FossilData {
    id: String,
    species: String,
    morphological_features: HashMap<String, f64>,
    preservation_quality: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnalysisResult {
    fossil_id: String,
    anomaly_score: f64,
    potential_species: Option<String>,
    confidence: f64,
}

fn main() {
    // Main entry point - parse CLI arguments and execute appropriate function
    println!("DinoFossilAnalyzer initialized");
    
    // Example usage of required functions
    let fossil1 = FossilData {
        id: "F001".to_string(),
        species: "Tyrannosaurus rex".to_string(),
        morphological_features: [("skull_length".to_string(), 1.2), ("jaw_width".to_string(), 0.8)].iter().cloned().collect(),
        preservation_quality: 0.7,
    };
    
    let fossil2 = FossilData {
        id: "F002".to_string(),
        species: "Triceratops".to_string(),
        morphological_features: [("skull_length".to_string(), 1.5), ("horn_length".to_string(), 0.9)].iter().cloned().collect(),
        preservation_quality: 0.6,
    };
    
    let result = analyze_fossil(&fossil1);
    println!("Analysis Result: {:?}", result);
    
    let comparison = compare_fossils(&fossil1, &fossil2);
    println!("Comparison Result: {:?}", comparison);
    
    let report = generate_report(&vec![result]);
    println!("Generated Report: {}", report);
}

fn analyze_fossil(fossil: &FossilData) -> AnalysisResult {
    // Simple anomaly detection based on morphological features
    let mut anomaly_score = 0.0;
    let mut confidence = 0.0;
    
    // Calculate anomaly score based on feature values
    for (_, &value) in &fossil.morphological_features {
        anomaly_score += value.abs();
    }
    
    // Normalize anomaly score
    if !fossil.morphological_features.is_empty() {
        anomaly_score /= fossil.morphological_features.len() as f64;
    }
    
    // Confidence calculation (simplified)
    confidence = fossil.preservation_quality * 0.8 + 0.2;
    
    AnalysisResult {
        fossil_id: fossil.id.clone(),
        anomaly_score,
        potential_species: None,
        confidence,
    }
}

fn compare_fossils(fossil1: &FossilData, fossil2: &FossilData) -> HashMap<String, f64> {
    // Compare two fossils and return similarity metrics
    let mut similarities = HashMap::new();
    
    // Compare morphological features
    let mut total_similarity = 0.0;
    let mut common_features = 0;
    
    for (feature, &val1) in &fossil1.morphological_features {
        if let Some(&val2) = fossil2.morphological_features.get(feature) {
            let similarity = 1.0 - (val1 - val2).abs() / (val1 + val2).max(1e-10);
            total_similarity += similarity;
            common_features += 1;
        }
    }
    
    // Calculate average similarity
    if common_features > 0 {
        similarities.insert("morphological_similarity".to_string(), total_similarity / common_features as f64);
    } else {
        similarities.insert("morphological_similarity".to_string(), 0.0);
    }
    
    // Compare preservation quality
    let preservation_diff = (fossil1.preservation_quality - fossil2.preservation_quality).abs();
    similarities.insert("preservation_difference".to_string(), preservation_diff);
    
    similarities
}

fn generate_report(results: &[AnalysisResult]) -> String {
    // Generate a simple text report from analysis results
    let mut report = String::from("DinoFossilAnalyzer Report\n");
    report.push_str("========================\n\n");
    
    for result in results {
        report.push_str(&format!("Fossil ID: {}\n", result.fossil_id));
        report.push_str(&format!("Anomaly Score: {:.2}\n", result.anomaly_score));
        report.push_str(&format!("Confidence: {:.2}%\n", result.confidence * 100.0));
        if let Some(species) = &result.potential_species {
            report.push_str(&format!("Potential Species: {}\n", species));
        }
        report.push_str("\n");
    }
    
    report
}