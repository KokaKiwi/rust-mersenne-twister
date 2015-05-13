//! Mersenne Twister 32-bits implementation.
//!
//! See [Mersenne Twister Homepage](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/MT2002/emt19937ar.html)
//! for more informations.
use std::default::Default;
use rand::{Rng, SeedableRng};

const N: usize = 624;
const M: usize = 397;
const DIFF: isize = M as isize - N as isize;

const MAGIC: [u32; 2] = [ 0, 0x9908b0df ];

const MAGIC_VALUE1: u32 = 1812433253;
const MAGIC_VALUE2: u32 = 0x9d2c5680;
const MAGIC_VALUE3: u32 = 0xefc60000;

const MAGIC_FACTOR1: u32 = 1664525;
const MAGIC_FACTOR2: u32 = 1566083941;

const MAGIC_SEED: u32 = 19650218;
const DEFAULT_SEED: u32 = 5489;

const UPPER_MASK: u32 = 1 << 31;
const LOWER_MASK: u32 = !UPPER_MASK;

#[derive(Copy)]
pub struct MTRng32 {
    state: [u32; N],
    index: usize,
}

impl MTRng32 {
    pub fn new(seed: u32) -> MTRng32 {
        let mut rng = MTRng32 {
            state: [0; N],
            // index = N + 1 means state is not initialized.
            index: N + 1,
        };
        rng.reset(seed);
        rng
    }

    pub fn new_array(seed: &[u32]) -> MTRng32 {
        let mut rng = MTRng32 {
            state: [0; N],
            // index = N + 1 means state is not initialized.
            index: N + 1,
        };
        rng.reset_array(seed);
        rng
    }

    fn reset(&mut self, seed: u32) {
        self.state[0] = seed;
        for index in (1..N) {
            let prec = self.state[index - 1];

            self.state[index] = MAGIC_VALUE1.wrapping_mul(prec ^ (prec >> 30)) + index as u32;
        }
        self.index = N;
    }

    fn reset_array(&mut self, seed: &[u32]) {
        use std::cmp::max;

        self.reset(MAGIC_SEED);

        let (mut i, mut j) = (1, 0);

        let size = max(N, seed.len());
        for _ in (0..size) {
            let prec = self.state[i - 1];
            self.state[i] = (self.state[i] ^ ((prec ^ (prec >> 30)).wrapping_mul(MAGIC_FACTOR1))) + seed[j] + j as u32;

            i += 1;
            j += 1;

            if i >= N {
                self.state[0] = self.state[N - 1];
                i = 1;
            }

            if j >= seed.len() {
                j = 0;
            }
        }

        for _ in (0..N - 1) {
            let prec = self.state[i - 1];
            self.state[i] = (self.state[i] ^ ((prec ^ (prec >> 30)).wrapping_mul(MAGIC_FACTOR2))) - i as u32;
            i += 1;

            if i >= N {
                self.state[0] = self.state[N - 1];
                i = 1;
            }
        }

        // MSB is 1; assuring non-zero initial array
        self.state[0] = 1 << 31;
    }

    fn next(&mut self) -> u32 {
        if self.index >= N {
            self.generate_words();
        }

        let mut y = self.state[self.index];
        self.index += 1;

        y ^= y >> 11;
        y ^= (y << 7) & MAGIC_VALUE2;
        y ^= (y << 15) & MAGIC_VALUE3;
        y ^= y >> 18;

        y
    }

    fn generate_words(&mut self) {
        for index in 0..(N - M) {
            let y = (self.state[index] & UPPER_MASK) | (self.state[index + 1] & LOWER_MASK);
            let magic_idx = (y & 0x1) as usize;
            self.state[index] = self.state[index + M] ^ (y >> 1) ^ MAGIC[magic_idx];
        }

        for index in (N - M)..(N - 1) {
            let y = (self.state[index] & UPPER_MASK) | (self.state[index + 1] & LOWER_MASK);
            let magic_idx = (y & 0x1) as usize;
            let nindex = index as isize + DIFF;
            self.state[index] = self.state[nindex as usize] ^ (y >> 1) ^ MAGIC[magic_idx];
        }

        {
            let y = (self.state[N-1] & UPPER_MASK) | (self.state[0] & LOWER_MASK);
            let magic_idx = (y & 0x1) as usize;
            self.state[N - 1] = self.state[M - 1] ^ (y >> 1) ^ MAGIC[magic_idx];
        }

        self.index = 0;
    }
}

impl Clone for MTRng32 {
	fn clone(&self) -> Self {
		*self
	}
}

impl Rng for MTRng32 {
    fn next_u32(&mut self) -> u32 {
        self.next()
    }
}

impl SeedableRng<u32> for MTRng32 {
    fn reseed(&mut self, seed: u32) {
        self.reset(seed);
    }

    fn from_seed(seed: u32) -> MTRng32 {
        MTRng32::new(seed)
    }
}

impl<'a> SeedableRng<&'a [u32]> for MTRng32 {
    fn reseed(&mut self, seed: &'a [u32]) {
        self.reset_array(seed);
    }

    fn from_seed(seed: &'a [u32]) -> MTRng32 {
        MTRng32::new_array(seed)
    }
}

impl Default for MTRng32 {
    fn default() -> MTRng32 {
        MTRng32::new(DEFAULT_SEED)
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;
    use rand::{Rng, SeedableRng};
    use test::Bencher;
    use super::MTRng32;

    const TEST_VECTOR: [u32; 80] = [
        1067595299,  955945823,  477289528, 4107218783, 4228976476,
        3344332714, 3355579695,  227628506,  810200273, 2591290167,
        2560260675, 3242736208,  646746669, 1479517882, 4245472273,
        1143372638, 3863670494, 3221021970, 1773610557, 1138697238,
        1421897700, 1269916527, 2859934041, 1764463362, 3874892047,
        3965319921,   72549643, 2383988930, 2600218693, 3237492380,
        2792901476,  725331109,  605841842,  271258942,  715137098,
        3297999536, 1322965544, 4229579109, 1395091102, 3735697720,
        2101727825, 3730287744, 2950434330, 1661921839, 2895579582,
        2370511479, 1004092106, 2247096681, 2111242379, 3237345263,
        4082424759,  219785033, 2454039889, 3709582971,  835606218,
        2411949883, 2735205030,  756421180, 2175209704, 1873865952,
        2762534237, 4161807854, 3351099340,  181129879, 3269891896,
         776029799, 2218161979, 3001745796, 1866825872, 2133627728,
          34862734, 1191934573, 3102311354, 2916517763, 1012402762,
        2184831317, 4257399449, 2899497138, 3818095062, 3030756734,
    ];

    #[test]
    fn test_vector() {
        let mut rng: MTRng32 = SeedableRng::from_seed(&[0x123, 0x234, 0x345, 0x456][..]);
        let values: Vec<u32> = rng.gen_iter().take(TEST_VECTOR.len()).collect();

        assert_eq!(&values[..], &TEST_VECTOR[..]);
    }

    #[bench]
    fn bench_64k(b: &mut Bencher) {
        let mut rng: MTRng32 = Default::default();
        let mut buf: [u8; 64 * 1024] = unsafe { ::std::mem::uninitialized() };
        b.iter(|| {
            rng.fill_bytes(&mut buf);
        });
        b.bytes = buf.len() as u64;
    }
}
