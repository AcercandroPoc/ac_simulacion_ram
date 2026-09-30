use crate::config::OrganismConfig;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;

pub const BASAL_MUTATION_RATE: f64 = 0.0001;

/// Evalúa la probabilidad de mutación SOS mediante una sigmoide alostérica continua
#[inline(always)]
pub fn eval_sos_mutation_prob(drug_conc: f64, mic: f64, org: &OrganismConfig) -> f64 {
    if drug_conc <= 0.0 {
        return BASAL_MUTATION_RATE;
    }
    // Pendiente empírica k=4.5 centrada en la CMI
    let k = 4.5;
    let sigmoid = 1.0 / (1.0 + (-k * (drug_conc - mic)).exp());
    BASAL_MUTATION_RATE + (org.sos_max_mutation_prob - BASAL_MUTATION_RATE) * sigmoid
}

/// Resuelve el evento mutacional durante la división:
/// Devuelve (es_viable, nuevo_genotipo)
#[inline(always)]
pub fn resolve_mutation_event(
    parent_g: u8,
    drug_conc: f64,
    mic: f64,
    mpc: f64,
    org: &OrganismConfig,
    rng: &mut Xoshiro256PlusPlus,
) -> (bool, u8) {
    // Si la concentración supera el MPC, no hay adaptación viable
    if drug_conc >= mpc {
        return (false, 0);
    }

    // La hipermutación SOS opera en la ventana sub-inhibitoria
    let in_sos_window = drug_conc >= (mic * 0.40) && drug_conc < mpc;

    if in_sos_window {
        let p_sos = eval_sos_mutation_prob(drug_conc, mic, org);
        if rng.random_bool(p_sos) {
            let roll: f64 = rng.random();
            if roll < org.sos_lethal_fraction {
                // Mutación deletérea / daño letal en ADN
                (false, 0)
            } else if roll < 0.90 {
                // Mutación neutral o sinónima
                (true, parent_g)
            } else {
                // Mutación adaptativa de paso escalonado
                (true, (parent_g + 1).min(4))
            }
        } else {
            (true, parent_g)
        }
    } else if rng.random_bool(BASAL_MUTATION_RATE) {
        // Tasa basal espontánea sin estrés inducido
        (true, (parent_g + 1).min(4))
    } else {
        (true, parent_g)
    }
}
