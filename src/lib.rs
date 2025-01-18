pub mod openqa_job;
pub use openqa_job::OpenQAJob;

use std::io::{self, BufRead, BufReader, Read, Write};

/// Processes input lines and outputs aggregated `OpenQA` test URLs
///
/// # Errors
///
/// This function will return an error if there is an issue with reading from the input
/// or writing to the output.
pub fn process_input<R: Read, W: Write>(input: R, mut output: W) -> io::Result<()> {
    let mut jobs: Vec<OpenQAJob> = BufReader::new(input)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            line.split("->")
                .nth(1)
                .map(str::trim)
                .and_then(OpenQAJob::from_url)
        })
        .collect();

    jobs.sort();
    jobs.dedup();
    aggregate_consecutive_jobs(&mut jobs);

    let output_str = if OpenQAJob::all_same_domain(&jobs) {
        OpenQAJob::format_compact_output(&jobs)
    } else {
        jobs.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    };

    writeln!(output, "openqa-mon {output_str}")
}

fn aggregate_consecutive_jobs(jobs: &mut Vec<OpenQAJob>) {
    let mut i = 0;
    while i < jobs.len().saturating_sub(1) {
        if jobs[i].is_consecutive_with(&jobs[i + 1]) {
            jobs[i].consecutive_count += 1;
            jobs.remove(i + 1);
        } else {
            i += 1;
        }
    }
}