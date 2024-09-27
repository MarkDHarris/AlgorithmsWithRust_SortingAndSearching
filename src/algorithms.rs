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
