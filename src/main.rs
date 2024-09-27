use std::io;
use std::io::Write;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let num_items = get_i32("Please enter # of values to generate (i32): ");
    let max = get_i32("Please enter the largest individual value allowed (0-max) (i32): ");

    let mut vec = make_random_vec(num_items, max);

    println!("INPUT: ");
    print_vec(&vec, 40);
    println!();

    let start_bubble_sort = Instant::now();
    bubble_sort(&mut vec);
    let bubble_sort_elapsed = start_bubble_sort.elapsed();

    println!("OUTPUT: ");
    print_vec(&vec, 50);
    println!();

    println!("VALIDATION: ");
    check_sorted(&vec);
    println!();

    println!("bubble_sort elapsed: {:.2?}", bubble_sort_elapsed);
}

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

pub struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    pub fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32
    }

    pub fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    pub fn next_u64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    pub fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_u64();
        return result as i32;
    }
}

pub fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input..!");

    let trimmed = str_value.trim();

    return trimmed.parse::<i32>().expect("Error parsing integer");
}

pub fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max))
    }
    return vec;
}

pub fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }

    string.push_str("]");

    println!("{string}");
}

pub fn check_sorted(vec: &Vec<i32>) -> bool {
    for i in 1..vec.len() {
        if vec[i - 1] > vec[i] {
            println!("The vector is NOT sorted!");
            return false;
        }
    }
    println!("The vector is sorted!");
    return true;
}
