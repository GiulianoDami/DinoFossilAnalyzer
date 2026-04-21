use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fossil {
    pub id: String,
    pub species: Option<String>,
    pub morphological_features: Vec<String>,
    pub preservation_quality: f64, // 0.0 to 1.0
    pub anomaly_score: Option<f64>,
}

pub fn detect_anomalies(fossil: &Fossil) -> bool {
    if let Some(score) = fossil.anomaly_score {
        score > 0.8
    } else {
        score_anomaly(fossil) > 0.8
    }
}

pub fn classify_species(fossil: &Fossil) -> Option<String> {
    if detect_anomalies(fossil) {
        Some("Potential New Species".to_string())
    } else if let Some(species) = &fossil.species {
        Some(species.clone())
    } else {
        None
    }
}

pub fn score_anomaly(fossil: &Fossil) -> f64 {
    let mut score = 0.0;
    
    // Check for unusual features
    let unusual_features = ["crushed", "deformed", "fragmented", "unusual_size"];
    for feature in &fossil.morphological_features {
        if unusual_features.contains(&feature.as_str()) {
            score += 0.2;
        }
    }
    
    // Consider preservation quality
    score += (1.0 - fossil.preservation_quality) * 0.3;
    
    // Cap at 1.0
    score.min(1.0)
}