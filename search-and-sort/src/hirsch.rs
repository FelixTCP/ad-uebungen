use super::algorithm::Algorithm;

pub struct HirschIndex;
impl Algorithm<Vec<usize>, usize> for HirschIndex {
    fn run(&self, list: Vec<usize>) -> usize {
        let mut citations = vec![0; list.len()];
        let mut needed_citations = (0..(list.len() + 1) as isize).collect::<Vec<isize>>();

        for i in 0..list.len() {
            let paper = list[i];
            citations[paper] += 1;
            let achived_citations_for_paper = citations[paper];
            needed_citations[achived_citations_for_paper] -= 1;
        }

        let mut h_index = 0;
        for i in 0..list.len() + 1 {
            if needed_citations[i] > 0 {
                return h_index;
            }
            h_index = i;
        }
        h_index
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_hirsch_index() {
        let test_cases = vec![
            (vec![0, 2, 3, 3, 0, 3, 1], 2),
            (vec![0], 1),
            (vec![0, 0, 0, 0, 0], 1),
            (vec![0, 1, 2, 3, 4], 1),
            (Vec::new(), 0),
        ];

        for (input, expected) in test_cases {
            println!("Input: {:?}", input);
            assert_eq!(HirschIndex.run(input), expected);
        }
    }
}
