use super::algorithm::Algorithm;
use rand::Rng;

pub fn init_random_list(length: usize, min: isize, max: isize) -> Vec<isize> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
        vec.push(rng.gen_range(min..max))
    }
    vec
}

pub fn is_sorted(list: &[isize]) -> bool {
    if list.len() == 0 {
        return true;
    }

    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn find_smallest_in_list(list: &[isize]) -> Option<(isize, usize)> {
    if list.is_empty() {
        return None;
    }

    let mut min_idx = 0;
    let mut min = list[min_idx];

    for i in min_idx..list.len() {
        if list[i] < min {
            min_idx = i;
            min = list[min_idx];
        }
    }

    Some((min, min_idx))
}

pub struct SelectionSort;
impl Algorithm<Vec<isize>, Vec<isize>> for SelectionSort {
    fn run(&self, list: Vec<isize>) -> Vec<isize> {
        if list.is_empty() {
            return list;
        }

        let mut sort_boundary = 0;
        let mut list = list;
        for _ in 0..list.len() - 1 {
            let (_, min_idx) = find_smallest_in_list(&list[sort_boundary..]).unwrap();

            list.swap(sort_boundary, sort_boundary + min_idx);
            sort_boundary += 1;
        }
        assert!(is_sorted(&list));
        list
    }
}

pub struct BubbleSort;
impl Algorithm<Vec<isize>, Vec<isize>> for BubbleSort {
    fn run(&self, list: Vec<isize>) -> Vec<isize> {
        let mut list = list;
        for i in 0..list.len() {
            for j in 0..list.len() - i - 1 {
                if list[j] > list[j + 1] {
                    list.swap(j, j + 1);
                }
            }
        }
        assert!(is_sorted(&list));
        list
    }
}

pub struct IterativeMerge;
impl Algorithm<Vec<Vec<isize>>, Vec<isize>> for IterativeMerge {
    fn run(&self, list: Vec<Vec<isize>>) -> Vec<isize> {
        let mut sorted = list[0].clone();
        for l in &list[1..] {
            sorted = self.merge(sorted, l);
        }
        sorted
    }
}

impl IterativeMerge {
    fn merge(&self, mut sorted: Vec<isize>, new: &Vec<isize>) -> Vec<isize> {
        let mut s = 0;
        let mut n = 0;

        while n < new.len() {
            if sorted[s] >= new[n] {
                sorted.insert(s, new[n]);
                n += 1;
            } else {
                s += 1;
            }
        }
        sorted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_random_list_length() {
        let min = 0;
        let max = 100;
        let list = init_random_list(10, min, max);
        assert_eq!(list.len(), 10);

        for &num in &list {
            assert!(num >= min && num < max);
        }
    }

    #[test]
    fn test_find_smallest_in_list() {
        let list = vec![5, 2, 8, 3, 1, 6, 4];
        let (value, index) = find_smallest_in_list(&list).unwrap();

        assert_eq!(value, 1);
        assert_eq!(index, 4);
    }

    #[test]
    fn test_find_smallest_in_list_empty() {
        let list = Vec::new();
        assert_eq!(find_smallest_in_list(&list), None);
    }

    #[test]
    fn test_is_sorted() {
        let tests_cases = [
            (vec![1, 2, 3, 4, 5], true),  // Already sorted
            (vec![5, 2, 3, 1, 4], false), // Not sorted
            (vec![1], true),              // Single element
            (Vec::new(), true),           // Empty list
        ];

        for (list, expected) in tests_cases {
            let result = is_sorted(&list);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_iterative_merge() {
        let list = vec![vec![3, 6, 7], vec![1], vec![2, 4]];

        assert!(is_sorted(&IterativeMerge.run(list)))
    }

    use rstest::rstest;

    macro_rules! sort_test_cases {
        () => {
            [
                (vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]), // Already sorted
                (vec![5, 2, 3, 1, 4], vec![1, 2, 3, 4, 5]), // Not sorted
                (vec![1], vec![1]),                         // Single element
                (Vec::new(), Vec::new()),                   // Empty list
            ]
        };
    }

    #[rstest]
    #[case(SelectionSort, sort_test_cases!())]
    #[case(BubbleSort, sort_test_cases!())]
    fn test_sort_algorithm<A: Algorithm<Vec<isize>, Vec<isize>>>(
        #[case] sort_algorithm: A,
        #[case] test_cases: [(Vec<isize>, Vec<isize>); 4],
    ) {
        for (target, expected) in test_cases {
            let result = sort_algorithm.run(target);
            assert_eq!(result, expected);
        }
    }
}
