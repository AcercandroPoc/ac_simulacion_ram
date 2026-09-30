use crate::core::Grid2D;

pub const DEFAULT_DIFFUSION_COEFF: f64 = 0.16; // Satisface el límite de Courant-Friedrichs-Lewy (CFL <= 0.25)
pub const DEFAULT_DRUG_DECAY: f64 = 0.0004; // Degradación pasiva del antibiótico en agar

/// Resuelve un paso temporal de la ecuación de difusión de Fick en 2D:
/// dC/dt = D * ∇²C - k_decay * C - sink
#[inline(always)]
pub fn step_diffusion_2d(
    current: &Grid2D<f64>,
    next: &mut Grid2D<f64>,
    diff_coeff: f64,
    decay_rate: f64,
    blee_sinks: Option<&[f64]>,
) {
    let w = current.width;
    let h = current.height;

    for y in 1..h - 1 {
        let row = y * w;
        for x in 1..w - 1 {
            let i = row + x;

            // Stencil discreto laplaciano de 5 puntos en 2D: ∇²C ≈ C_E + C_W + C_N + C_S - 4*C
            let laplacian = current.data[i - 1]
                + current.data[i + 1]
                + current.data[i - w]
                + current.data[i + w]
                - 4.0 * current.data[i];

            let sink = match blee_sinks {
                Some(sinks) => sinks[i],
                None => 0.0,
            };

            let updated =
                current.data[i] + diff_coeff * laplacian - decay_rate * current.data[i] - sink;
            next.data[i] = updated.max(0.0);
        }
    }
}
