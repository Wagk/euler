// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

// TODO: Generate prime numbers
// divide until undivisible, then go to next prime and try again

use std::vec::Vec;

struct Prime {
    primes : Vec<i64>
}

// TODO: Create a pseudo-generator for thiso
// TODO: https://gist.github.com/glebm/440bbe2fc95e7abee40eb260ec82f85c
// We're probably not thinking functional enough
impl Prime {

    fn sieve(count : i64) -> Vec<i64> {

        let stopcount = (count as f64).sqrt() as i64;

        let mut sieve = {
            let mut sieve = Vec::<bool>::with_capacity(count as usize);
            for i in 0..count {
                if i % 2 != 0 {
                    sieve.push(false);
                } else {
                    sieve.push(true);
                }
            }
            sieve
        };
        let mut primes = vec![2 as i64];

        for i in sieve {

        }

        primes
    }


    fn generate(&self, until: i32) {

    }

    fn next_prime() {

    }

}

pub fn solve() {
    // DEBUG: Remove this when done
    Prime::sieve(300);

    let number = 600851475143;
}
