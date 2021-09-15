use std::cmp::min;

#[derive(PartialEq, Eq, Debug)]
pub struct SieveOfEratosthenes {
    sieve: Vec<bool>,
    index: usize,
}

impl Iterator for SieveOfEratosthenes {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if let Some(&is_prime) = self.sieve.get(self.index) {
            if is_prime {
                for i in (self.index..self.sieve.len()).step_by(self.index) {
                    self.sieve[i] = false;
                }
            }
            self.index += 1;
            Some(is_prime)
        } else {
            None
        }
    }
}

pub fn sieve_of_eratosthenes(len: usize) -> SieveOfEratosthenes {
    let mut ret = SieveOfEratosthenes {
        sieve: vec![true; len],
        index: 0,
    };
    for i in 0..(min(len, 2)) {
        ret.sieve[i] = false;
    }
    ret
}

pub fn primes(n: impl Into<usize>) -> Vec<usize> {
    sieve_of_eratosthenes(n.into())
        .enumerate()
        .filter(|&(_, p)| p)
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
}

pub fn divisors(n: impl Into<usize>) -> Vec<usize> {
    let n = n.into();
    (1..=n)
        .take_while(|i| i * i <= n)
        .filter(|&i| n % i == 0)
        .flat_map(|i| if i * i == n { vec![i] } else { vec![i, n / i] })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::{primes, sieve_of_eratosthenes};

    mod sieve_of_eratosthenes {
        #[test]
        fn test_next() {
            use super::super::sieve_of_eratosthenes;
            macro_rules! test {
                ($n:expr, [$($e:expr),*]) => {
                    let mut d = sieve_of_eratosthenes($n);
					$(
						assert_eq!($e, d.next());
					)*
                };
            }

            test!(0, [None]);
            test!(1, [Some(false), None]);
            test!(2, [Some(false), Some(false), None]);
            test!(3, [Some(false), Some(false), Some(true), None]);
            test!(4, [Some(false), Some(false), Some(true), Some(true), None]);
            test!(
                5,
                [
                    Some(false),
                    Some(false),
                    Some(true),
                    Some(true),
                    Some(false),
                    None
                ]
            );
        }
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        use super::SieveOfEratosthenes;

        macro_rules! test {
            ($e:expr) => {
                assert_eq!(
                    SieveOfEratosthenes {
                        sieve: (0..$e).map(|i| i >= 2).collect::<Vec<_>>(),
                        index: 0,
                    },
                    sieve_of_eratosthenes($e)
                )
            };
        }

        test!(0);
        test!(1);
        test!(2);
        test!(3);
        test!(10);
        test!(100);
    }

    #[test]
    fn test_primes() {
        assert_eq!(vec![2, 3, 5, 7], primes(10usize));
        assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29], primes(30usize));
    }
}
