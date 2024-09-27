use sorting::algorithms::*;
use sorting::helpers::*;
use std::time::Instant;

fn main() {
    let num_items = get_i32("Please enter # of values to generate (i32): ");
    let max = get_i32("Please enter the largest individual value allowed (0-max) (i32): ");

    let vec = make_random_vec(num_items, max);
    print_vec(&vec, 40);
    println!();

    loop {
        let target = get_i32("Please enter the target value to search for or -1 to exit (i32): ");

        if target < 0 {
            println!("exiting...");
            break;
        }

        let start_linear_search = Instant::now();
        let (index, number_of_tests) = linear_search(&vec, target);
        let linear_search_elapsed = start_linear_search.elapsed();

        if index < 0 {
            println!("Target {target} not found, {number_of_tests} tests");
        } else {
            let result = vec[index as usize];
            println!("numbers[{index}] = {result}, {number_of_tests} tests");
        }
        println!();
        println!("linear_search elapsed: {:.2?}", linear_search_elapsed);
        println!();
    }
}
