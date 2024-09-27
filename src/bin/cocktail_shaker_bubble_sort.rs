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

    let start_bubble_sort = Instant::now();
    cocktail_shaker_bubble_sort(&mut vec);
    let bubble_sort_elapsed = start_bubble_sort.elapsed();

    println!("OUTPUT: ");
    print_vec(&vec, 50);
    println!();

    println!("VALIDATION: ");
    check_sorted(&vec);
    println!();

    println!(
        "cocktail_shaker_bubble_sort elapsed: {:.2?}",
        bubble_sort_elapsed
    );
}
