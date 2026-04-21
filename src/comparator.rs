use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Fossil {
    pub id: String,
    pub species: String,
    pub morphological_features: Vec<String>,
    pub preservation_quality: f64,
}

pub fn compare_fossils(fossil1: &Fossil, fossil2: &Fossil) -> bool {
    fossil1.species == fossil2.species
}

pub fn calculate_similarity_score(fossil1: &Fossil, fossil2: &Fossil) -> f64 {
    if fossil1.species != fossil2.species {
        return 0.0;
    }
    
    let features1: std::collections::HashSet<&str> = fossil1.morphological_features.iter().map(|s| s.as_str()).collect();
    let features2: std::collections::HashSet<&str> = fossil2.morphological_features.iter().map(|s| s.as_str()).collect();
    
    let intersection: std::collections::HashSet<_> = features1.intersection(&features2).collect();
    let union: std::collections::HashSet<_> = features1.union(&features2).collect();
    
    if union.is_empty() {
        return 0.0;
    }
    
    let jaccard_similarity = intersection.len() as f64 / union.len() as f64;
    
    jaccard_similarity * fossil1.preservation_quality * fossil2.preservation_quality
}