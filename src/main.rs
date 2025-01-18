#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

use std::io::{self, BufRead, BufReader, Read, Write};
use oqa_jobfilter::OpenQAJob;

fn main() -> io::Result<()> {
    process_input(io::stdin(), io::stdout())
}

/// Processes input lines and outputs aggregated `OpenQA` test URLs
///
/// # Errors
///
/// This function will return an error if there is an issue with reading from the input
/// or writing to the output.
pub fn process_input<R: Read, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut tests: Vec<OpenQAJob> = BufReader::new(input)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            line.split("->")
                .nth(1)
                .map(str::trim)
                .and_then(OpenQAJob::from_url)
        })
        .collect();

    tests.sort();
    tests.dedup();

    // Aggregate consecutive tests
    let mut i = 0;
    while i < tests.len().saturating_sub(1) {
        if tests[i].is_consecutive_with(&tests[i + 1]) {
            tests[i].consecutive_count += 1;
            tests.remove(i + 1);
        } else {
            i += 1;
        }
    }

    let output_str = if OpenQAJob::all_same_domain(&tests) {
        OpenQAJob::format_compact_output(&tests)
    } else {
        tests.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    };

    writeln!(output, "openqa-mon {output_str}")
}
