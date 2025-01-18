#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
)]

use std::io;
use oqa_jobfilter::process_input;

fn main() -> io::Result<()> {
    process_input(io::stdin(), io::stdout())
}
