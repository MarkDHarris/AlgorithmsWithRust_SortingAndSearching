use sorting::algorithms::*;
use sorting::helpers::*;
use std::time::Instant;

fn main() {
    let num_items = 8; //get_i32("Please enter # of values to generate (i32): ");
    let max = 5; //get_i32("Please enter the largest individual value allowed (0-max) (i32): ");

    let mut vec = customer_make_random_vec(num_items, max);

    println!("INPUT: ");
    customer_print_vec(&vec, 40);
    println!();

    let start_quicksort = Instant::now();
    let sorted = customer_counting_sort(&mut vec[..]);
    let quicksort_elapsed = start_quicksort.elapsed();

    println!("OUTPUT: ");
    customer_print_vec(&sorted, 50);
    println!();

    println!("VALIDATION: ");
    customer_check_sorted(&sorted);
    println!();

    println!("counting_sort elapsed: {:.2?}", quicksort_elapsed);
}
