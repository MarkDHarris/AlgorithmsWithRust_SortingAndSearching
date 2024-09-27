use sorting::algorithms::*;
use sorting::helpers::*;
use std::time::Instant;

fn main() {
    let num_items = get_i32("Please enter # of values to generate (i32): ");
    let max = get_i32("Please enter the largest individual value allowed (0-max) (i32): ");

    let mut vec = make_random_vec(num_items, max);

    println!("INPUT: ");
    print_vec(&vec, 40);
    println!();

    let start_quicksort = Instant::now();
    let sorted = counting_sort(&mut vec[..]);
    let quicksort_elapsed = start_quicksort.elapsed();

    println!("OUTPUT: ");
    print_vec(&sorted, 50);
    println!();

    println!("VALIDATION: ");
    check_sorted(&sorted);
    println!();

    println!("counting_sort elapsed: {:.2?}", quicksort_elapsed);
}
