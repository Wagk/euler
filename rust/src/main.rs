// TODO:
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we
// get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {

    let mult3 = |x: i32| (x % 3 == 0) == true;
    let mult5 = |x: i32| (x % 5 == 0) == true;

    let mut sum = 0;

    for x in 0..1000 {
        if mult3(x) || mult5(x) {
            sum += x
        }
    }

    println!("output: {}", sum)

}
