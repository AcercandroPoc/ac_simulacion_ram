use crate::config::PharmacokineticsConfig;

/// Calcula la concentración plasmática en el minuto `t` aplicando la ecuación
/// analítica mono-compartimental oral con acumulación periódica de dosis:
/// C(t) = C_pico * (e^(-ke * dt) - e^(-ka * dt))
#[inline(always)]
pub fn calculate_bateman_pk(t: usize, max_doses: usize, pk: &PharmacokineticsConfig) -> f64 {
    let mut total_conc = 0.0;

    for d in 0..max_doses {
        let dose_time = d * pk.dose_interval_tau;
        if t >= dose_time {
            let dt = (t - dose_time) as f64;
            let curve = (-pk.elimination_rate_ke * dt).exp() - (-pk.absorption_rate_ka * dt).exp();
            total_conc += pk.peak_concentration * curve.max(0.0) * 1.16;
        }
    }

    total_conc
}
