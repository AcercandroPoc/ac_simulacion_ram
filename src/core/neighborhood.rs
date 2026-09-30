use super::cell::Cell;
use super::grid::Grid2D;

pub const CHAMFER_ORTHOGONAL_WEIGHT: usize = 7;
pub const CHAMFER_DIAGONAL_WEIGHT: usize = 5;
pub const CHAMFER_MAX_DENSITY: usize = 48; // (4 * 7) + (4 * 5)
pub const CROWDING_THRESHOLD: usize = 33; // ~70% saturación

#[inline(always)]
pub fn compute_density_chamfer(grid: &Grid2D<Cell>, x: usize, y: usize) -> usize {
    let w = grid.width;
    let h = grid.height;

    let has_n = y > 0;
    let has_s = y + 1 < h;
    let has_w = x > 0;
    let has_e = x + 1 < w;

    let mut density = 0;

    // Ortogonales (peso 7)
    if has_n && grid.get(x, y - 1).is_alive() {
        density += CHAMFER_ORTHOGONAL_WEIGHT;
    }
    if has_s && grid.get(x, y + 1).is_alive() {
        density += CHAMFER_ORTHOGONAL_WEIGHT;
    }
    if has_w && grid.get(x - 1, y).is_alive() {
        density += CHAMFER_ORTHOGONAL_WEIGHT;
    }
    if has_e && grid.get(x + 1, y).is_alive() {
        density += CHAMFER_ORTHOGONAL_WEIGHT;
    }

    // Diagonales (peso 5)
    if has_n && has_w && grid.get(x - 1, y - 1).is_alive() {
        density += CHAMFER_DIAGONAL_WEIGHT;
    }
    if has_n && has_e && grid.get(x + 1, y - 1).is_alive() {
        density += CHAMFER_DIAGONAL_WEIGHT;
    }
    if has_s && has_w && grid.get(x - 1, y + 1).is_alive() {
        density += CHAMFER_DIAGONAL_WEIGHT;
    }
    if has_s && has_e && grid.get(x + 1, y + 1).is_alive() {
        density += CHAMFER_DIAGONAL_WEIGHT;
    }

    density
}

#[inline(always)]
pub fn get_best_neighbor_genotype(grid: &Grid2D<Cell>, x: usize, y: usize) -> Option<u8> {
    let w = grid.width;
    let h = grid.height;

    let neighbors = [
        if y > 0 {
            grid.get(x, y - 1)
        } else {
            Cell::EMPTY
        },
        if y + 1 < h {
            grid.get(x, y + 1)
        } else {
            Cell::EMPTY
        },
        if x > 0 {
            grid.get(x - 1, y)
        } else {
            Cell::EMPTY
        },
        if x + 1 < w {
            grid.get(x + 1, y)
        } else {
            Cell::EMPTY
        },
    ];

    let mut best: Option<u8> = None;
    for n in neighbors {
        if n.is_alive() {
            let g = n.genotype();
            match best {
                None => best = Some(g),
                Some(cur) if g > cur => best = Some(g),
                _ => {}
            }
        }
    }
    best
}
