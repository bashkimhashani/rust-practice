fn binary_search(numbers: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = numbers.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if numbers[mid] == target {
            return Some(mid);
        } else if numbers[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

fn main() {
    let numbers = [1, 3, 5, 7, 9, 11];
    let target = 7;

    match binary_search(&numbers, target) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}