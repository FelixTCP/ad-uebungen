use super::algorithm::Algorithm;

pub fn contains(list: &Vec<isize>, entry: isize) -> bool {
    BinarySearch.run((list.clone(), entry)).is_some()
}

pub struct BinarySearch;
impl Algorithm<(Vec<isize>, isize), Option<usize>> for BinarySearch {
    fn run(&self, args: (Vec<isize>, isize)) -> Option<usize> {
        let (list, target) = args;

        if list.is_empty() {
            return None;
        }

        let mut search_idx = list.len() / 2;

        while search_idx < list.len() {
            if list[search_idx] == target {
                return Some(search_idx);
            }

            if search_idx == 0 || search_idx == list.len() - 1 {
                break;
            }

            if list[search_idx] < target {
                search_idx = (search_idx + list.len()) / 2;
            } else {
                search_idx = search_idx / 2;
            }
        }
        None
    }
}

pub struct LinearSearch;
impl Algorithm<(Vec<isize>, isize), Option<usize>> for LinearSearch {
    fn run(&self, args: (Vec<isize>, isize)) -> Option<usize> {
        let (list, target) = args;
        for (i, &item) in list.iter().enumerate() {
            if item == target {
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contians() {
        let test_cases = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1, true), // First element
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, true), // Early element
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7, true), // Late element
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, true), // Last element
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11, false), // Not found
            (vec![1], 1, true),                             // Single element
            (Vec::new(), 1, false),                         // Empty list
            (Vec::new(), 10, false),                        // Empty list
        ];

        for (list, target, expected) in test_cases {
            let result = contains(&list, target);
            assert_eq!(result, expected);
        }
    }

    use rstest::rstest;

    macro_rules! search_test_cases {
        () => {
            [
                (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1, Some(0)), // First element
                (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, Some(2)), // Early element
                (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 7, Some(6)), // Late element
                (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, Some(9)), // Last element
                (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 11, None),   // Not found
                (vec![1], 1, Some(0)),                             // Single element
                (Vec::new(), 1, None),                             // Empty list
                (Vec::new(), 10, None),                            // Empty list
            ]
        };
    }

    // Parameterized test for multiple search algorithms
    #[rstest]
    #[case(BinarySearch, search_test_cases!())]
    #[case(LinearSearch, search_test_cases!())]
    fn test_search_algorithm<A: Algorithm<(Vec<isize>, isize), Option<usize>>>(
        #[case] search_algorithm: A,
        #[case] test_cases: [(Vec<isize>, isize, Option<usize>); 8],
    ) {
        for (list, target, expected) in test_cases {
            let result = search_algorithm.run((list, target));
            assert_eq!(result, expected);
        }
    }
}
