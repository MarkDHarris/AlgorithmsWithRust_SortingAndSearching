pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut n = vec.len();
    let mut swapped: bool = true;

    while swapped {
        swapped = false;

        for i in 1..n {
            if vec[i - 1] > vec[i] {
                let tmp = vec[i - 1];
                vec[i - 1] = vec[i];
                vec[i] = tmp;
                swapped = true;
            }
        }
        n = n - 1;
    }
}

pub fn cocktail_shaker_bubble_sort(vec: &mut Vec<i32>) {
    let length = vec.len();
    let mut swapped;
    let mut start = 0;
    let mut end = length - 1;

    loop {
        swapped = false;

        // forward pass
        for i in start..end {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }

        end -= 1;

        if !swapped || start > end {
            break;
        }

        swapped = false;

        // backward pass
        for i in (start..end).rev() {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
                swapped = true;
            }
        }
        start += 1;
        if !swapped || start > end {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::*;
    use std::time::Instant;

    #[test]
    fn run_bubble_sort_with_10000_items() {
        let num_items = 10000;
        let max = 1000;

        let mut vec = make_random_vec(num_items, max);

        let start = Instant::now();
        bubble_sort(&mut vec);
        let elapsed = start.elapsed();
        println!("elapsed: {:.2?}", elapsed);

        let is_sorted = check_sorted(&vec);

        assert_eq!(is_sorted, true);
    }

    #[test]
    fn run_cocktailshaker_sort_with_10000_items() {
        let num_items = 10000;
        let max = 1000;

        let mut vec = make_random_vec(num_items, max);

        let start = Instant::now();
        cocktail_shaker_bubble_sort(&mut vec);
        let elapsed = start.elapsed();
        println!("elapsed: {:.2?}", elapsed);

        let is_sorted = check_sorted(&vec);

        assert_eq!(is_sorted, true);
    }
}
