fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);

    quicksort(left);
    quicksort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90, 88, 45, 50];
    println!("Original array: {:?}", numbers);

    quicksort(&mut numbers);
    println!("Sorted array: {:?}", numbers);

    let mut strings = vec!["banana", "apple", "cherry", "date", "elderberry"];
    println!("\nOriginal strings: {:?}", strings);

    quicksort(&mut strings);
    println!("Sorted strings: {:?}", strings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut arr: Vec<i32> = vec![];
        quicksort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut arr = vec![1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn test_sorted_array() {
        let mut arr = vec![1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted_array() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quicksort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_array() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        quicksort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }
}
