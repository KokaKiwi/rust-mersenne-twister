//! Mersenne Twister 64-bits implementation.
//!
//! See [Mersenne Twister 64-bits page](http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/emt64.html)
//! for more informations.
use std::default::Default;
use rand::{Rng, SeedableRng};

const N: usize = 312;
const M: usize = 156;
const DIFF: isize = M as isize - N as isize;

const MAGIC: [u64; 2] = [ 0, 0xB5026F5AA96619E9 ];

const MAGIC_VALUE1: u64 = 6364136223846793005;
const MAGIC_VALUE2: u64 = 0x5555555555555555;
const MAGIC_VALUE3: u64 = 0x71D67FFFEDA60000;
const MAGIC_VALUE4: u64 = 0xFFF7EEE000000000;

const MAGIC_FACTOR1: u64 = 3935559000370003845;
const MAGIC_FACTOR2: u64 = 2862933555777941757;

const MAGIC_SEED: u64 = 19650218;
const DEFAULT_SEED: u64 = 5489;

const UPPER_MASK: u64 = 0xFFFFFFFF80000000;
const LOWER_MASK: u64 = !UPPER_MASK;

pub struct MTRng64 {
    state: [u64; N],
    index: usize,
}

impl MTRng64 {
    pub fn new(seed: u64) -> MTRng64 {
        let mut rng = MTRng64 {
            state: [0; N],
            // index = NN + 1 means state is not initialized.
            index: N + 1,
        };
        rng.reset(seed);
        rng
    }

    pub fn new_array(seed: &[u64]) -> MTRng64 {
        let mut rng = MTRng64 {
            state: [0; N],
            // index = N + 1 means state is not initialized.
            index: N + 1,
        };
        rng.reset_array(seed);
        rng
    }

    fn reset(&mut self, seed: u64) {
        self.state[0] = seed;
        for index in (1..N) {
            let prec = self.state[index - 1];
            self.state[index] = MAGIC_VALUE1.wrapping_mul(prec ^ (prec >> 62)) + index as u64;
        }
        self.index = N;
    }

    fn reset_array(&mut self, seed: &[u64]) {
        use std::cmp::max;

        self.reset(MAGIC_SEED);

        let (mut i, mut j) = (1, 0);

        let size = max(N, seed.len());
        for _ in (0..size) {
            let prec = self.state[i - 1];
            self.state[i] = (self.state[i] ^ ((prec ^ (prec >> 62)).wrapping_mul(MAGIC_FACTOR1))) + seed[j] + j as u64;

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
            self.state[i] = (self.state[i] ^ ((prec ^ (prec >> 62)).wrapping_mul(MAGIC_FACTOR2))) - i as u64;
            i += 1;

            if i >= N {
                self.state[0] = self.state[N - 1];
                i = 1;
            }
        }

        // MSB is 1; assuring non-zero initial array
        self.state[0] = 1 << 63;
    }

    fn next(&mut self) -> u64 {
        if self.index >= N {
            self.generate_words();
        }

        let mut y = self.state[self.index];
        self.index += 1;

        y ^= (y >> 29) & MAGIC_VALUE2;
        y ^= (y << 17) & MAGIC_VALUE3;
        y ^= (y << 37) & MAGIC_VALUE4;
        y ^= y >> 43;

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

impl Copy for MTRng64 {}

impl Rng for MTRng64 {
    fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next()
    }
}

impl SeedableRng<u64> for MTRng64 {
    fn reseed(&mut self, seed: u64) {
        self.reset(seed);
    }

    fn from_seed(seed: u64) -> MTRng64 {
        MTRng64::new(seed)
    }
}

impl<'a> SeedableRng<&'a [u64]> for MTRng64 {
    fn reseed(&mut self, seed: &'a [u64]) {
        self.reset_array(seed);
    }

    fn from_seed(seed: &'a [u64]) -> MTRng64 {
        MTRng64::new_array(seed)
    }
}

impl Default for MTRng64 {
    fn default() -> MTRng64 {
        MTRng64::new(DEFAULT_SEED)
    }
}

#[cfg(test)]
mod test {
    use std::default::Default;
    use rand::{Rng, SeedableRng};
    use test::Bencher;
    use super::MTRng64;

    const TEST_VECTOR: [u64; 90] = [
 7266447313870364031,  4946485549665804864, 16945909448695747420, 16394063075524226720,  4873882236456199058,
14877448043947020171,  6740343660852211943, 13857871200353263164,  5249110015610582907, 10205081126064480383,
 1235879089597390050, 17320312680810499042, 16489141110565194782,  8942268601720066061, 13520575722002588570,
14226945236717732373,  9383926873555417063, 15690281668532552105, 11510704754157191257, 15864264574919463609,
 6489677788245343319,  5112602299894754389, 10828930062652518694, 15942305434158995996, 15445717675088218264,
 4764500002345775851, 14673753115101942098,   236502320419669032, 13670483975188204088, 14931360615268175698,
 8904234204977263924, 12836915408046564963, 12120302420213647524, 15755110976537356441,  5405758943702519480,
10951858968426898805, 17251681303478610375,  4144140664012008120, 18286145806977825275, 13075804672185204371,
10831805955733617705,  6172975950399619139, 12837097014497293886, 12903857913610213846,   560691676108914154,
 1074659097419704618, 14266121283820281686, 11696403736022963346, 13383246710985227247,  7132746073714321322,
10608108217231874211,  9027884570906061560, 12893913769120703138, 15675160838921962454,  2511068401785704737,
14483183001716371453,  3774730664208216065,  5083371700846102796,  9583498264570933637, 17119870085051257224,
 5217910858257235075, 10612176809475689857,  1924700483125896976,  7171619684536160599, 10949279256701751503,
15596196964072664893, 14097948002655599357,   615821766635933047,  5636498760852923045, 17618792803942051220,
  580805356741162327,   425267967796817241,  8381470634608387938, 13212228678420887626, 16993060308636741960,
  957923366004347591,  6210242862396777185,  1012818702180800310, 15299383925974515757, 17501832009465945633,
17453794942891241229, 15807805462076484491,  8407189590930420827,   974125122787311712,  1861591264068118966,
  997568339582634050, 18046771844467391493, 17981867688435687790,  3809841506498447207,  9460108917638135678,
    ];

    #[test]
    fn test_vector() {
        let mut rng: MTRng64 = SeedableRng::from_seed([0x12345, 0x23456, 0x34567, 0x45678].as_slice());
        let values: Vec<_> = rng.gen_iter().take(TEST_VECTOR.len()).collect();

        assert_eq!(values.as_slice(), TEST_VECTOR.as_slice());
    }

    #[bench]
    fn bench_64k(b: &mut Bencher) {
        let mut rng: MTRng64 = Default::default();
        let mut buf: [u8; 64 * 1024] = unsafe { ::std::mem::uninitialized() };
        b.iter(|| {
            rng.fill_bytes(&mut buf);
        });
        b.bytes = buf.len() as u64;
    }
}
