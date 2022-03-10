fn main() {
    let mut numbers: [i32; 6] = [-7, 3, 2, 9, -9, 27];

    selection_sort(&mut numbers);

    println!("{:?}", numbers);

}

fn insertion_sort(numbers: &mut [i32; 6]){
    
}

fn selection_sort(numbers: &mut [i32; 6]){
    for i in 0..numbers.len(){
        let mut min_number: i32 = i32::MAX;
        let mut min_index: usize = usize::MIN;
        for j in i..numbers.len(){
            if numbers[j] <= min_number {
                min_number = numbers[j];
                min_index = j
;            }
        }

        swap_indices(numbers, i, min_index);
    }
}

fn swap_indices(numbers: &mut [i32; 6], j: usize, min_index: usize){
    let temp: i32 = numbers[j];
    numbers[j] = numbers[min_index];
    numbers[min_index] = temp;
}
