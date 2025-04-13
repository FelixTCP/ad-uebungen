mod sort;
use sort::*;
mod search;
use search::*;
mod reorder;
use reorder::*;
mod misc;
use misc::*;
mod hirsch;
use hirsch::*;
mod algorithm;
use algorithm::Algorithm;

use std::fs::{File, OpenOptions};
use std::io::Write;

fn main() {
    runtime_suite(
        HirschIndex,
        "runtime_hirsch_index.csv",
        &[
            (0..20000, 100),
            (20000..50000, 1000),
            (50000..100000, 10000),
        ],
        |size| {
            let mut list = vec![0; size * 10];

            for i in 0..list.len() {
                list[i] = rand::random::<usize>() % size;
            }

            list
        },
    );
}

fn runtime_suite<A, Args, R>(
    algo: A,
    file_name: &str,
    ranges_sizes_and_steps: &[(std::ops::Range<usize>, usize)],
    input_generator: impl Fn(usize) -> Args,
) where
    A: Algorithm<Args, R>,
{
    let mut file = setup_file(file_name, "size,duration_ns");

    for &(ref range, step_size) in ranges_sizes_and_steps {
        for l in range.clone().step_by(step_size) {
            let input = input_generator(l);

            // TODO Nachfragen, ob sinnvoll
            //let algo_clone = algo.clone();

            let start = std::time::Instant::now();
            algo.run(input);
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
