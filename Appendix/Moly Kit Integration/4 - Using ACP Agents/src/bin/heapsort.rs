/// Heap sort (in-place, not stable).
///
/// - Time: O(n log n)
/// - Space: O(1) extra
/// - Requirements: `T: Ord`
fn heap_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    // Build a max-heap in-place.
    let last_parent = (len - 2) / 2;
    for start in (0..=last_parent).rev() {
        sift_down(arr, start, len - 1);
    }

    // Pop max into the end, shrink heap, and restore heap property.
    for end in (1..len).rev() {
        arr.swap(0, end);
        sift_down(arr, 0, end - 1);
    }
}

fn sift_down<T: Ord>(arr: &mut [T], mut root: usize, end: usize) {
    loop {
        let left_child = root * 2 + 1;
        if left_child > end {
            break;
        }

        let right_child = left_child + 1;
        let mut max_child = left_child;
        if right_child <= end && arr[right_child] > arr[left_child] {
            max_child = right_child;
        }

        if arr[max_child] > arr[root] {
            arr.swap(root, max_child);
            root = max_child;
        } else {
            break;
        }
    }
}

fn main() {
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90, 88, 45, 50];
    println!("Original array: {:?}", numbers);
    heap_sort(&mut numbers);
    println!("Sorted array:   {:?}", numbers);

    let mut words = vec!["banana", "apple", "cherry", "date", "elderberry"];
    println!("\nOriginal words: {:?}", words);
    heap_sort(&mut words);
    println!("Sorted words:   {:?}", words);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted<T: Ord>(arr: &[T]) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![42];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn random_integers() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    fn with_duplicates() {
        let mut arr = vec![3, 1, 2, 3, 2, 1, 3];
        heap_sort(&mut arr);
        assert!(is_sorted(&arr));
        assert_eq!(arr, vec![1, 1, 2, 2, 3, 3, 3]);
    }

    #[test]
    fn strings() {
        let mut arr = vec!["banana", "apple", "cherry", "date", "elderberry"];
        heap_sort(&mut arr);
        assert_eq!(arr, vec!["apple", "banana", "cherry", "date", "elderberry"]);
    }
}

