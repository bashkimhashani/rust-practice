fn quick_sort(numbers: &mut [i32]) {
    // Zero or one element is already sorted.
    if numbers.len() <= 1 {
        return;
    }

    let pivot_index = partition(numbers);

    // Sort both sides, excluding the pivot.
    quick_sort(&mut numbers[..pivot_index]);
    quick_sort(&mut numbers[pivot_index + 1..]);
}

fn partition(numbers: &mut [i32]) -> usize {
    let last = numbers.len() - 1;
    let pivot = numbers[last];
    let mut boundary = 0;

    for i in 0..last {
        if numbers[i] < pivot {
            // Move smaller numbers to the left.
            numbers.swap(i, boundary);
            boundary += 1;
        }
    }

    // Put the pivot in its final sorted position.
    numbers.swap(boundary, last);

    boundary
}

fn main() {
    let mut numbers = [6, 3, 8, 5, 2, 7, 4];

    quick_sort(&mut numbers);

    println!("{:?}", numbers);
}