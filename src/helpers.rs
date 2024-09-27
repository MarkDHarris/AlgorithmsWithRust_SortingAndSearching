use core::fmt;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub struct Customer {
    pub id: String,
    pub num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

pub fn customer_make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    let mut prng = Prng::new();

    let mut customers: Vec<Customer> = Vec::with_capacity(num_items as usize);
    for i in 0..num_items {
        let id = format!("C{i}");
        let num_purchases = prng.next_i32(0, max);
        let customer = Customer {
            id: id,
            num_purchases: num_purchases,
        };
        customers.push(customer);
    }
    return customers;
}

pub fn customer_print_vec(vec: &Vec<Customer>, num_items: i32) {
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

pub fn customer_check_sorted(vec: &Vec<Customer>) {
    for i in 1..vec.len() {
        if vec[i - 1].num_purchases > vec[i].num_purchases {
            println!("The customer vector is NOT sorted!");
            return;
        }
    }
    println!("The vector is sorted!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sorted_known_sorted() {
        let mut v1 = Vec::new();

        for n in 1..100 {
            v1.push(n);
        }

        let is_sorted = check_sorted(&v1);

        assert!(is_sorted);
    }

    #[test]
    fn check_sorted_known_unsorted() {
        let mut v1 = Vec::new();

        for n in 1..100 {
            v1.push(n);
        }
        v1.push(1_000_000);
        v1.push(101);

        let is_sorted = check_sorted(&v1);

        assert!(!is_sorted);
    }
}
