pub const COLOR_OSD_BG: u32     = 0x0002_0617;
pub const COLOR_TISSUE_BG: u32  = 0x00E2_E8F0;
pub const COLOR_AGAR_BG: u32    = 0x00FE_F3C7;
pub const COLOR_GRAPH_BG: u32   = 0x000F_172A;
pub const COLOR_TEXT: u32       = 0x00F8_FAFC;
pub const COLOR_TEXT_INFO: u32  = 0x0038_BDF8;
pub const COLOR_DRUG_CURVE: u32 = 0x00EC_4899;

/// Colores estandarizados por genotipo g in [0..4]
pub const GENOTYPE_COLORS: [u32; 5] = [
    0x0033_4155, // g=0: Cepa salvaje sensible
    0x0002_84C7, // g=1: Mutante 1 - Porinas
    0x000D_9488, // g=2: Mutante 2 - Bombas de eflujo
    0x00D9_7706, // g=3: Mutante 3 - BLEE basal
    0x0093_33EA, // g=4: Hiperresistente / BLEE pleno
];

#[inline(always)]
pub fn get_genotype_color(genotype: u8) -> u32 {
    let g = (genotype as usize).min(4);
    GENOTYPE_COLORS[g]
}