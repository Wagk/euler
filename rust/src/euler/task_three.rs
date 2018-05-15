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

    fn sieve(count : usize) -> Vec<usize> {

        let stopcount: usize = (count as f64).sqrt() as usize + 1;

        println!("count {}, stopcount {}", count, stopcount);

        //Initialize a range to what we want
        let mut sieve = (0..count)
            .map(|num| true)
            .collect::<Vec<bool>>();

        for i in 2..stopcount {
            if sieve[i] == true {
                let mut j = i * i;
                while j < count {
                    sieve[j] = false;
                    j += i;
                }
            }
        }

        sieve
            .into_iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i, val)| {if val == true {Some(i)} else {None}})
            .collect::<Vec<usize>>()
    }
}

pub fn solve() {
    // DEBUG: Remove this when done
    let primes = Prime::sieve(300);

    for p in primes {
        println!("{}", p);
    }

    let number = 600851475143;
}
