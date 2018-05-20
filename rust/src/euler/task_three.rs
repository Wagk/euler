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

    //DEBUG: remove this when done
    for p in primes {
        println!("{}", p);
    }

    // going down the list of primes:
    // - Keep dividing by that prime, tracking how many times the division
    // happened
    // - Take the remainder, move to next prime, and repeat former step
    // - There should only be a remainder of 1 since all integers have a unique
    // prime factorisation
    //
    // assert that the factors multiply to that number

    let mut number = 600851475143;

    let mut factors: Vec<usize>;

    for p in primes {
        loop {
            let div = number % p;
            match div {
                0 => { // keep dividing the the term
                    factors.push(p);
                    number = div;
                },
                number => { // can't divide anymore, move on
                    break;
                },
                _ => {

                }
            }
        }
    }

    let number = 600851475143;
}
