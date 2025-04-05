#[derive(Clone)]
pub struct SieveOfEratosthenes {
    /// 2 * i + 1
    is_not_prime: Box<[u64]>,
    max: usize,
}

impl SieveOfEratosthenes {
    /// for cache optimization.
    const CHUNK_SIZE: usize = 32 * 1024 / 64;

    // TODO: use isqrt(), next_multiple_of() & div_ceil()
    pub fn new(n: usize) -> Self {
        let mut is_not_prime = Vec::from_iter(
            std::iter::repeat(DIVIDABLE_BY_3_OR_5_OR_7)
                .take(n / (105 * 64) + 1)
                .flatten()
                .take(n / 128 + 1),
        )
        .into_boxed_slice();
        // push 1 and remove 3, 5, and 7
        is_not_prime[0] ^= 0b1111;

        // step 1. find odd prime numbers < sqrt(n)
        let sqrt_b = (is_not_prime.len() as f64).sqrt().ceil() as usize;
        let mut small_primes = Vec::with_capacity(sqrt_b * 64);
        // start from 11
        for i in 5..sqrt_b * 64 {
            // if (2 * i + 1) is odd prime
            if is_not_prime[i / 64] & (1 << (i % 64)) == 0 {
                small_primes.push(2 * i + 1);
                for j in (2 * i * (i + 1)..sqrt_b * 64).step_by(2 * i + 1) {
                    is_not_prime[j / 64] |= 1 << (j % 64)
                }
            }
        }

        // step 2. perform prime test for each chunk
        let mut off_set = sqrt_b * 64;
        for chunk in is_not_prime[sqrt_b..].chunks_mut(Self::CHUNK_SIZE) {
            for &p in &small_primes {
                let next_multiple_of_p = (p * p).max((2 * off_set + 1 + p - 1) / p * p);
                let start = if next_multiple_of_p % 2 == 0 {
                    (next_multiple_of_p + p) / 2 - off_set
                } else {
                    next_multiple_of_p / 2 - off_set
                };
                for j in (start..chunk.len() * 64).step_by(p) {
                    chunk[j / 64] |= 1 << (j % 64)
                }
            }

            off_set += chunk.len() * 64;
        }

        Self {
            is_not_prime,
            max: n,
        }
    }

    pub fn is_prime(&self, i: usize) -> bool {
        i == 2 || (i % 2 == 1 && { self.is_not_prime[i / 2 / 64] & (1 << (i / 2 % 64)) == 0 })
    }

    pub fn into_primes(self) -> Primes {
        let Self {
            mut is_not_prime,
            max,
        } = self;

        is_not_prime[max / 128] |= !0 << ((max + 1) / 2 % 64);
        Primes {
            into_iter: is_not_prime.into_vec().into_iter(),
            is_prime: 0,
            offset: 0,
            state: if max >= 2 {
                State::Entry
            } else {
                State::Finished
            },
        }
    }
}

const DIVIDABLE_BY_3_OR_5_OR_7: [u64; 3 * 5 * 7] = {
    let mut result = [0; 105];
    // 3 = 2 * 1 + 1
    let mut i = 1;
    while i < 105 * 64 {
        result[i / 64] |= 1 << i % 64;
        i += 3
    }
    // 5 = 2 * 2 + 1;
    i = 2;
    while i < 105 * 64 {
        result[i / 64] |= 1 << i % 64;
        i += 5
    }
    // 7 = 2* 3 + 1
    i = 3;
    while i < 105 * 64 {
        result[i / 64] |= 1 << i % 64;
        i += 7
    }

    result
};

enum State {
    Entry,
    OnGoing,
    Finished,
}
pub struct Primes {
    into_iter: <Vec<u64> as IntoIterator>::IntoIter,
    is_prime: u64,
    offset: u32,
    state: State,
}

impl Iterator for Primes {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::OnGoing => (),
            State::Finished => return None,
            State::Entry => {
                if let Some(is_not_prime) = self.into_iter.next() {
                    self.is_prime = !is_not_prime;
                    self.state = State::OnGoing;
                    return Some(2);
                } else {
                    self.state = State::Finished;
                    return None;
                }
            }
        }

        match self.is_prime.trailing_zeros() {
            64 => {
                while let Some(is_not_prime) = self.into_iter.next() {
                    self.offset += 64;
                    self.is_prime = !is_not_prime;

                    match self.is_prime.trailing_zeros() {
                        64 => continue,
                        i @ 0..=63 => {
                            self.is_prime ^= 1 << i;
                            return Some(2 * (self.offset + i) + 1);
                        }
                        _ => continue,
                    }
                }

                self.state = State::Finished;
                None
            }
            i @ 0..=63 => {
                self.is_prime ^= 1 << i;
                Some(2 * (self.offset + i) + 1)
            }
            _ => unreachable!(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, max) = self.into_iter.size_hint();
        (0, max.map(|v| v * 64))
    }
}
