mod sort;
use sort::*;
mod search;
use search::*;
mod algorithm;
use algorithm::Algorithm;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::usize;

fn main() {
    runtime_suite(
        SelectionSort,
        "runtime_selection_sort.csv",
        &[(0..20000, 100), (20000..51000, 1000)],
        |size| {
            let range = size as isize;
            init_random_list(size, -range, range)
        },
    );

    runtime_suite(
        BubbleSort,
        "runtime_selection_sort.csv",
        &[(0..20000, 100), (20000..51000, 1000)],
        |size| {
            let range = size as isize;
            init_random_list(size, -range, range)
        },
    );

    runtime_suite(
        BinarySearch,
        "runtime_binary_search.csv",
        &[
            (0..20000, 100),
            (20000..51000, 1000),
            (51000..110000, 10000),
            (110000..1000000, 100000),
        ],
        |size| {
            let list: Vec<isize> = (0..size as isize).collect();
            let target = (size - 1) as isize;
            (list, target)
        },
    );

    runtime_suite(
        LinearSearch,
        "runtime_linear_search.csv",
        &[
            (0..20000, 100),
            (20000..51000, 1000),
            (51000..110000, 10000),
            (110000..1000000, 100000),
        ],
        |size| {
            let list: Vec<isize> = (0..size as isize).collect();
            let target = (size - 1) as isize;
            (list, target)
        },
    );
}

fn runtime_suite<A, Args, R>(
    algo: A,
    file_name: &str,
    ranges_sizes_and_steps: &[(std::ops::Range<usize>, usize)],
    input_generator: impl Fn(usize) -> Args,
) where
    A: Algorithm<Args, R> + Clone,
    Args: Clone,
{
    let mut file = setup_file(file_name, "size,duration_ns");

    for &(ref range, step_size) in ranges_sizes_and_steps {
        for l in range.clone().step_by(step_size) {
            let input = input_generator(l);

            // TODO Nachfragen, ob sinnvoll
            //let algo_clone = algo.clone();

            let start = std::time::Instant::now();
            algo.run(input.clone());
            let duration = start.elapsed();

            if l % (step_size * 10) == 0 {
                println!("Input size {} processed in {}ns", l, duration.as_nanos());
            }

            writeln!(file, "{},{}", l, duration.as_nanos()).expect("Unable to write to file");
        }
    }
}

fn setup_file(file_name: &str, header: &str) -> File {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();

    // Write header if file is new
    if file.metadata().unwrap().len() == 0 {
        writeln!(file, "{}", header).unwrap();
    }

    file
}
