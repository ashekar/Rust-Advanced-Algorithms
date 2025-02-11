fn main() {
    let mut numbers: [i32; 6] = [-7, 3, 2, 9, -9, 27];

    bubble_sort(&mut numbers);

    println!("{:?}", numbers);


    merge_sort(&mut numbers);

    println!("{:?}", numbers);


    selection_sort(&mut numbers);

    println!("{:?}", numbers);
}

// Merge sort function
fn merge_sort(arr: &mut [i32]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        let mut left = arr[..mid].to_vec();
        let mut right = arr[mid..].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        merge(&mut left, &mut right, arr);
    }
}

fn bubble_sort(numbers: &mut [i32; 6]) {
    let max_val: usize = numbers.len() - 1;
    for _ in 0..numbers.len() {
        for i in 0..max_val {
            let iterator = numbers.len() - i - 1;
            let check_next = iterator - 1;
            if numbers[iterator] < numbers[check_next] {
                swap_indices(numbers, iterator, check_next);
            }
        }
    }
}

fn selection_sort(numbers: &mut [i32; 6]) {
    for i in 0..numbers.len() {
        let mut min_index: usize = i;
        for j in i + 1..numbers.len() {
            if numbers[j] < numbers[min_index] {
                min_index = j;
            }
        }
        swap_indices(numbers, i, min_index);
    }
}

fn swap_indices(numbers: &mut [i32; 6], j: usize, min_index: usize) {
    numbers.swap(j, min_index);
}

fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>, arr: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}