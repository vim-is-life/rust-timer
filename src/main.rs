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
use std::io::Write;
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

        // case where we just started
        if current_time == last_time {
            print!("{end_time}");
            last_time += 1;
        // case where we're going through and we're not just starting but we're
        // also not done.
        } else if current_time != last_time && current_time != end_time {
            print!("\r{:>5}", end_time - current_time);
            last_time += 1;
            std::io::stdout().flush().unwrap();
        // case where we've finished
        } else {
            println!("\ndone!");
            break;
        }
    }
}

fn main() {
    let args = Args::parse();

    countdown(args.time);
}
