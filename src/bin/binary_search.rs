use sorting::algorithms::*;
use sorting::helpers::*;
use std::time::Instant;

fn main() {
    let num_items = get_i32("Please enter # of values to generate (i32): ");
    let max = get_i32("Please enter the largest individual value allowed (0-max) (i32): ");

    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, 40);
    println!();

    quicksort(&mut vec[..]);
    print_vec(&vec, 40);
    println!();

    loop {
        let target = get_i32("Please enter the target value to search for or -1 to exit (i32): ");

        if target < 0 {
            println!("exiting...");
            break;
        }

        let start_binary_search = Instant::now();
        let (index, number_of_tests) = binary_search(&vec, target);
        let binary_search_elapsed = start_binary_search.elapsed();

        if index < 0 {
            println!("Target {target} not found, {number_of_tests} tests");
        } else {
            let result = vec[index as usize];
            println!("numbers[{index}] = {result}, {number_of_tests} tests");
        }
        println!();
        println!("binary_search elapsed: {:.2?}", binary_search_elapsed);
        println!();
    }
}
