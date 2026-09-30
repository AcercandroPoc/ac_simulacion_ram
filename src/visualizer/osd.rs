use super::font::draw_string;
use super::palette::{COLOR_DRUG_CURVE, COLOR_GRAPH_BG, COLOR_OSD_BG, COLOR_TEXT, COLOR_TEXT_INFO, GENOTYPE_COLORS};

/// Renderiza la barra superior de información OSD
pub fn render_osd_bar(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    header_line: &str,
    info_line: &str,
) {
    buffer[..height * width].fill(COLOR_OSD_BG);
    draw_string(buffer, width, height, 12, 10, header_line, COLOR_TEXT);
    draw_string(buffer, width, height, 12, 28, info_line, COLOR_TEXT_INFO);
}

/// Dibuja la curva farmacocinética y las demografías clonales en la franja inferior
pub fn render_telemetry_graph(
    buffer: &mut [u32],
    width: usize,
    total_height: usize,
    graph_y_start: usize,
    graph_height: usize,
    history_counts: &[[usize; 5]],
    history_drug: Option<&[f64]>,
    max_cells_scale: f32,
    max_drug_scale: f32,
) {
    let graph_start_idx = graph_y_start * width;
    if graph_start_idx < buffer.len() {
        buffer[graph_start_idx..].fill(COLOR_GRAPH_BG);
    }

    let n_points = history_counts.len();
    if n_points <= 1 {
        return;
    }

    let h_plot = graph_height as f32 - 20.0;

    for col in 0..width {
        let p_idx = (col * n_points) / width;
        if p_idx < n_points {
            // 1. Curvas demográficas bacterianas
            for g in 0..5 {
                let count = history_counts[p_idx][g] as f32;
                let y_norm = (h_plot - (count / max_cells_scale) * h_plot).clamp(0.0, h_plot) as usize;
                let py = graph_y_start + y_norm;
                if py < total_height {
                    buffer[py * width + col] = GENOTYPE_COLORS[g];
                }
            }

            // 2. Curva PK de fármaco si está presente
            if let Some(drug_data) = history_drug {
                if p_idx < drug_data.len() {
                    let conc = drug_data[p_idx] as f32;
                    let y_pk = (h_plot - (conc / max_drug_scale) * h_plot).clamp(0.0, h_plot) as usize;
                    let py = graph_y_start + y_pk;
                    if py < total_height {
                        buffer[py * width + col] = COLOR_DRUG_CURVE;
                    }
                }
            }
        }
    }
}