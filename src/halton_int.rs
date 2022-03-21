// #![feature(unboxed_closures)]

fn vdc(mut k: usize, base: usize, scale: u32) -> usize {
    let mut vdc: usize = 0;
    let mut factor = base.pow(scale);
    while k != 0 {
        factor /= base;
        let remainder = k % base;
        k /= base;
        vdc += remainder * factor;
    }
    vdc
}

pub struct Vdcorput {
    count: usize,
    base: usize,
    scale: u32,
}

impl Vdcorput {
    pub fn new(base: usize, scale: u32) -> Self {
        Vdcorput {
            count: 0,
            base,
            scale,
        }
    }

    pub fn pop(&mut self) -> usize {
        self.count += 1;
        vdc(self.count, self.base, self.scale)
    }

    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.count = seed;
    }
}

// impl FnOnce<()> for Vdcorput {
//     type Output = f64;
//     extern "rust-call" fn call_once(self, _arg: ()) -> Self::Output {
//         self.count += 1;
//         vdc(self.count, self.base)
//     }
// }

/**
 * @brief Halton sequence generator
 *
 */
pub struct Halton {
    vdc0: Vdcorput,
    vdc1: Vdcorput,
}

impl Halton {
    pub fn new(base: &[usize], scale: &[u32]) -> Self {
        Halton {
            vdc0: Vdcorput::new(base[0], scale[0]),
            vdc1: Vdcorput::new(base[1], scale[1]),
        }
    }

    pub fn pop(&mut self) -> [usize; 2] {
        [self.vdc0.pop(), self.vdc1.pop()]
    }

    /**
     * @brief
     *
     * @param seed
     */
    #[allow(dead_code)]
    pub fn reseed(&mut self, seed: usize) {
        self.vdc0.reseed(seed);
        self.vdc1.reseed(seed);
    }
}
