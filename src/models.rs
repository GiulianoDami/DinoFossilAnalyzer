use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fossil {
    pub id: String,
    pub species: Option<String>,
    pub morphological_features: MorphologicalFeatures,
    pub geological_context: GeologicalContext,
    pub preservation_quality: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MorphologicalFeatures {
    pub skull_shape: Option<String>,
    pub limb_bone_structure: Option<String>,
    pub tooth_pattern: Option<String>,
    pub vertebral_column: Option<String>,
    pub size_indicators: Vec<String>,
    pub anomaly_flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeologicalContext {
    pub formation: String,
    pub period: String,
    pub location: Location,
    pub stratigraphic_layer: i32,
    pub matrix_composition: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub site_name: String,
}