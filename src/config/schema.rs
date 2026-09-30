use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct OrganismConfig {
    pub name: String,
    pub base_division_prob: f64,
    pub fitness_cost_per_mutation: f64,
    pub maintenance_cost: u8,
    pub division_cost: u8,
    pub min_res_for_division: u8,
    pub sos_max_mutation_prob: f64,
    pub sos_lethal_fraction: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PharmacokineticsConfig {
    pub absorption_rate_ka: f64,
    pub elimination_rate_ke: f64,
    pub dose_interval_tau: usize,
    pub peak_concentration: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AntibioticConfig {
    pub name: String,
    pub code: String,
    pub family: String,
    pub disk_potency: f64,
    pub mics: Vec<f64>,
    pub mpcs: Vec<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SimulationConfig {
    pub organism: OrganismConfig,
    pub pharmacokinetics: PharmacokineticsConfig,
    pub antibiotics: Vec<AntibioticConfig>,
}

impl SimulationConfig {
    /// Carga y parsea el archivo de configuración TOML desde disco
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Error al leer el archivo de configuración: {}", e))?;

        let config: SimulationConfig = toml::from_str(&content)
            .map_err(|e| format!("Error de parseo sintáctico en TOML: {}", e))?;

        config.validate()?;
        Ok(config)
    }

    /// Validación biofísica de rangos numéricos
    pub fn validate(&self) -> Result<(), String> {
        if self.organism.base_division_prob <= 0.0 || self.organism.base_division_prob > 1.0 {
            return Err("base_division_prob debe estar en el rango (0.0, 1.0]".to_string());
        }
        if self.organism.fitness_cost_per_mutation < 0.0
            || self.organism.fitness_cost_per_mutation >= 1.0
        {
            return Err("fitness_cost_per_mutation debe estar en el rango [0.0, 1.0)".to_string());
        }
        if self.pharmacokinetics.elimination_rate_ke <= 0.0 {
            return Err("elimination_rate_ke debe ser mayor que cero".to_string());
        }
        if self.antibiotics.is_empty() {
            return Err("Debe configurarse al menos un antibiótico".to_string());
        }
        for ab in &self.antibiotics {
            if ab.mics.len() < 5 || ab.mpcs.len() < 5 {
                return Err(format!(
                    "El antibiótico '{}' debe definir al menos 5 niveles de CMI y MPC (g=0..4)",
                    ab.name
                ));
            }
        }
        Ok(())
    }
}
