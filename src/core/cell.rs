#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Cell {
    pub state: u8,
    pub resource: u8,
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        state: 0,
        resource: 0,
    };

    #[inline(always)]
    pub fn new_alive(genotype: u8, resource: u8) -> Self {
        let g = (genotype & 0x07) << 4;
        Cell {
            state: 0b1000_0000 | g, // Bit 7 activo (vivo), estrés en 0
            resource,
        }
    }

    #[inline(always)]
    pub fn is_alive(&self) -> bool {
        (self.state & 0b1000_0000) != 0
    }

    #[inline(always)]
    pub fn genotype(&self) -> u8 {
        (self.state >> 4) & 0x07
    }

    #[inline(always)]
    pub fn stress(&self) -> u8 {
        self.state & 0x0F
    }

    #[inline(always)]
    pub fn with_stress_and_genotype(&self, stress: u8, genotype: u8) -> Self {
        let alive_bit = self.state & 0b1000_0000;
        let g = (genotype & 0x07) << 4;
        let s = stress.min(15) & 0x0F;
        Cell {
            state: alive_bit | g | s,
            resource: self.resource,
        }
    }

    #[inline(always)]
    pub fn with_resource(&self, resource: u8) -> Self {
        Cell {
            state: self.state,
            resource,
        }
    }
}
