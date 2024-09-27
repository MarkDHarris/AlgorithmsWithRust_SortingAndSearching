use sorting::algorithms::*;
use sorting::helpers::*;
use std::time::Instant;
fn main() {
    let num_items = 15000;
    let max = 1000;
    let target = 500;

    let input = make_random_vec(num_items, max);
    let mut bubble_sort_input = input.clone();
    let mut cocktail_shaker_bubble_sort_input = input.clone();
    let mut quicksort_input = input.clone();
    let mut counting_sort_input = input.clone();

    let start_bubble_sort = Instant::now();
    bubble_sort(&mut bubble_sort_input);
    let bubble_sort_elapsed = start_bubble_sort.elapsed();
    print_vec(&bubble_sort_input, 50);
    let bubble_sort_is_sorted = check_sorted(&bubble_sort_input);
    println!(
        "bubble_sort [{:?}] elapsed: {:.2?}",
        bubble_sort_is_sorted, bubble_sort_elapsed
    );
    println!();

    let start_cocktail_bubble_sort = Instant::now();
    cocktail_shaker_bubble_sort(&mut cocktail_shaker_bubble_sort_input);
    let cocktail_shaker_bubble_sort_elapsed = start_cocktail_bubble_sort.elapsed();
    print_vec(&cocktail_shaker_bubble_sort_input, 50);
    let cocktail_shaker_bubble_sort_is_sorted = check_sorted(&cocktail_shaker_bubble_sort_input);
    println!(
        "cocktail_shaker_bubble_sort [{:?}] elapsed: {:.2?}",
        cocktail_shaker_bubble_sort_is_sorted, cocktail_shaker_bubble_sort_elapsed
    );
    println!();

    let start_quicksort = Instant::now();
    quicksort(&mut quicksort_input);
    let quicksort_elapsed = start_quicksort.elapsed();
    print_vec(&quicksort_input, 50);
    let quicksort_is_sorted = check_sorted(&quicksort_input);
    println!(
        "quicksort [{:?}] elapsed: {:.2?}",
        quicksort_is_sorted, quicksort_elapsed
    );
    println!();

    let start_counting_sort = Instant::now();
    let counting_sort_sorted = counting_sort(&mut counting_sort_input);
    let counting_sort_elapsed = start_counting_sort.elapsed();
    print_vec(&counting_sort_sorted, 50);
    let counting_sort_is_sorted = check_sorted(&counting_sort_sorted);
    println!(
        "counting_sort [{:?}] elapsed: {:.2?}",
        counting_sort_is_sorted, counting_sort_elapsed
    );
    println!();

    let start_linear_search = Instant::now();
    let (linear_search_index, number_of_tests) = linear_search(&input, target);
    let linear_search_elapsed = start_linear_search.elapsed();
    if linear_search_index < 0 {
        println!("Target {target} not found, {number_of_tests} tests");
    } else {
        let result = input[linear_search_index as usize];
        println!("linear search: input[{linear_search_index}] = {result}, {number_of_tests} tests");
    }
    println!("linear_search elapsed: {:.2?}", linear_search_elapsed);
    println!();

    let start_binary_search = Instant::now();
    let (binary_search_index, number_of_tests) = binary_search(&quicksort_input, target);
    let binary_search_elapsed = start_binary_search.elapsed();
    if binary_search_index < 0 {
        println!("Target {target} not found, {number_of_tests} tests");
    } else {
        let result = input[binary_search_index as usize];
        println!("binary search input[{binary_search_index}] = {result}, {number_of_tests} tests");
    }
    println!("binary_search elapsed: {:.2?}", binary_search_elapsed);
    println!();
}
