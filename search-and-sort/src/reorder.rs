use super::algorithm::Algorithm;

pub struct ShiftLeft;
impl Algorithm<Vec<isize>, Vec<isize>> for ShiftLeft {
    fn run(&self, mut list: Vec<isize>) -> Vec<isize> {
        if list.is_empty() {
            return list;
        }

        let n = list.len();
        let key = list[0];
        for i in 1..n {
            list[i - 1] = list[i];
        }
        list[n - 1] = key;
        list
    }
}

pub struct ShiftRight;
impl Algorithm<Vec<isize>, Vec<isize>> for ShiftRight {
    fn run(&self, mut list: Vec<isize>) -> Vec<isize> {
        if list.is_empty() {
            return list;
        }

        let n = list.len();
        let key = list[n - 1];
        for i in 1..n {
            list[n - i] = list[n - i - 1];
        }
        list[0] = key;
        list
    }
}

pub struct InvertListRecursive;
impl Algorithm<Vec<isize>, Vec<isize>> for InvertListRecursive {
    fn run(&self, list: Vec<isize>) -> Vec<isize> {
        if list.is_empty() {
            return list;
        }

        self.invert_recursive(0, list.len() - 1, list)
    }
}
impl InvertListRecursive {
    fn invert_recursive(&self, l: usize, r: usize, mut list: Vec<isize>) -> Vec<isize> {
        if r >= list.len() || l >= r {
            return list;
        }

        list.swap(l, r);
        self.invert_recursive(l + 1, r - 1, list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    macro_rules! shift_left_cases {
        () => {
            [
                (vec![1, 2, 3, 4, 5], vec![2, 3, 4, 5, 1]), // Sorted
                (vec![5, 2, 3, 1, 4], vec![2, 3, 1, 4, 5]), // Not sorted
                (vec![1], vec![1]),                         // Single element
                (Vec::new(), Vec::new()),                   // Empty list
            ]
        };
    }

    macro_rules! shift_right_cases {
        () => {
            [
                (vec![1, 2, 3, 4, 5], vec![5, 1, 2, 3, 4]), // Sorted
                (vec![5, 2, 3, 1, 4], vec![4, 5, 2, 3, 1]), // Not sorted
                (vec![1], vec![1]),                         // Single element
                (Vec::new(), Vec::new()),                   // Empty list
            ]
        };
    }

    macro_rules! invert_list_cases {
        () => {
            [
                (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]), // Sorted
                (vec![5, 2, 3, 1, 4], vec![4, 1, 3, 2, 5]), // Not sorted
                (vec![1], vec![1]),                         // Single element
                (Vec::new(), Vec::new()),                   // Empty list
            ]
        };
    }

    #[rstest]
    #[case(ShiftLeft, shift_left_cases!())]
    #[case(ShiftRight, shift_right_cases!())]
    #[case(InvertListRecursive, invert_list_cases!())]
    fn test_reorder_algorithm<A: Algorithm<Vec<isize>, Vec<isize>>>(
        #[case] algorithm: A,
        #[case] test_cases: [(Vec<isize>, Vec<isize>); 4],
    ) {
        for (target, expected) in test_cases {
            let result = algorithm.run(target);
            assert_eq!(result, expected);
        }
    }
}
