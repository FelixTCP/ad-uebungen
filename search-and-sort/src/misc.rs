use super::algorithm::Algorithm;

pub struct AmountOfElementsThatSumUpToK;
impl Algorithm<(Vec<isize>, isize), usize> for AmountOfElementsThatSumUpToK {
    fn run(&self, args: (Vec<isize>, isize)) -> usize {
        let (list, k) = args;
        let mut count = 0;
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                if list[i] + list[j] == k {
                    count += 1;
                }
            }
        }
        count
    }
}

pub struct GetMinElementOrZero;
impl Algorithm<Vec<isize>, isize> for GetMinElementOrZero {
    fn run(&self, mut list: Vec<isize>) -> isize {
        for i in 0..list.len() {
            if list[i] == 0 {
                continue;
            }

            let a = list[i];
            list[i] = 0;
            let b = self.run(list);
            if a > b {
                return b;
            }
            return a;
        }
        0
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_amount_of_elements_that_sum_up_to_k() {
        let test_cases = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 10, 4), // Four elements sum up to 10
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3, 1),  // Only 1 + 2
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], -1, 0), // No two elements sum up to -1
            (vec![1], 1, 0),                              // Single Element
            (Vec::new(), 1, 0),                           // Empty Vector
        ];

        for (list, k, expected) in test_cases.iter() {
            assert_eq!(
                AmountOfElementsThatSumUpToK.run((list.clone(), *k)),
                *expected
            );
        }
    }

    #[test]
    fn test_get_min_element_or_zero() {
        let test_cases = [
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0), // Minimum element is 1
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0], 0),  // Minimum element is 0
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, -1], -1), // Minimum element is -1
            (vec![1], 0),                             // Single Element
            (Vec::new(), 0),                          // Empty Vector
        ];

        for (list, expected) in test_cases.iter() {
            assert_eq!(GetMinElementOrZero.run(list.clone()), *expected);
        }
    }
}
