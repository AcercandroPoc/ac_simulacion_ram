pub const LYSIS_RECYCLE_RESOURCE: u8 = 10;
pub const DORMANT_DENSITY_THRESHOLD: usize = 38;

/// Calcula la probabilidad de lisis de una bacteria viva por minuto
#[inline(always)]
pub fn compute_death_probability(
    drug_conc: f64,
    mic: f64,
    mpc: f64,
    density: usize,
) -> f64 {
    // Tolerancia por persistencia en núcleo denso (biofilm/core)
    if density >= DORMANT_DENSITY_THRESHOLD {
        return 0.0002;
    }

    // Acción bactericida tiempo-dependiente (T > CMI)
    if drug_conc >= mpc {
        0.0075 // Zona supracrítica: t1/2 ≈ 92 min
    } else if drug_conc >= mic {
        0.0022 // Ventana MSW: estrés y muerte intermedia
    } else {
        0.0001 // Senescencia basal normal
    }
}

/// Función de mortalidad Hill para modelos continuos analíticos (ODE)
#[inline(always)]
pub fn hill_kill_rate(conc: f64, mic: f64, hill_coeff: f64, max_kill: f64) -> f64 {
    if conc <= 0.0 {
        return 0.0;
    }
    let num = conc.powf(hill_coeff);
    let den = num + mic.powf(hill_coeff);
    max_kill * (num / den)
}