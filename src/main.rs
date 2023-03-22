/*
 * want to have
 * - TODO logic to make the display update and have the timer do what it does
 * - TODO a display of the timer
 * - TODO accepting cli params to set the timer
 * - TODO pretty print time with the timer
 * - TODO functionality to play a ticking sound every second
 * - TODO add functionality to be a stopwatch as well
 */

use clap::Parser;
use std::time::Instant;

// Define the cli args
#[derive(Parser, Debug)]
#[command(author = "vim-is-life", about = "A CLI timer program.")]
struct Args {
    /// The time, currently in seconds, to count down from.
    time: u64,
}

// We make the parameter a u64 because the current time will come as a u64.
fn countdown(end_time: u64) {
    let start_time = Instant::now();
    let mut last_time: u64 = 0;
    loop {
        let current_time = start_time.elapsed().as_secs();

        if current_time == end_time {
            println!("done!");
            break;
        } else if current_time != last_time {
            println!("{}", end_time - current_time);
            last_time += 1;
        }
    }
}

fn main() {
    let args = Args::parse();

    countdown(args.time);
}
