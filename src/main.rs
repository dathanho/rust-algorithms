use std::cmp::Ordering;

fn main() {
    let mut array = [-1, 10, 9, 2, -2, 7, -9, -3];
    let target = 9;
    // Unsorted
    println!("The unsorted array is: \n{:?}", array);
    let index = linear_search(&target, &array);
    println!("The number {} is at {} in this array.\n", target, index);
    // Sorted
    bubble_sort(&mut array);
    let index = binary_search(&target, &array);
    println!("The sorted array is: \n{:?}", array);
    println!("The number {} is at {} in this array.", target, index);
}

pub fn linear_search<T: Eq>(target: &T, arr: &[T]) -> isize {
    for (i, data) in arr.iter().enumerate() {
        if target == data {
            return i as isize;
        }
    }
    -1
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn binary_search<T: Ord>(target: &T, arr: &[T]) -> isize {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match target.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Equal => return mid as isize,
            Ordering::Greater => left = mid + 1,
        }
    }
    -1
}
