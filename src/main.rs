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
use std::num::ParseIntError;
use std::time::Instant;

// Define the cli args
#[derive(Parser, Debug)]
#[command(author = "vim-is-life", about = "A CLI timer program.")]
struct Args {
    /// The time to count down from. Accepted syntax: 15m, 1min, 3secs, 4s.
    /// Will default to seconds if no suffix is passed.
    time: String,
}

/// Struct to represent the time to count down from
#[derive(Debug)]
struct CountdownTime {
    mins_and_secs: (u64, u64),
    complete_secs: u64,
}

/// Function to parse a time given as a string into a CountdownTime instance.
fn parse_time(given_time: &str) -> Result<CountdownTime, ParseIntError> {
    let test = (
        given_time.contains("min"),
        given_time.contains('m'),
        given_time.contains("sec"),
        given_time.contains('s'),
    );
    match test {
        (true, _, _, _) => {
            let mins: u64 = given_time.split("min").collect::<String>().parse()?;
            Ok(CountdownTime {
                mins_and_secs: (mins, 0),
                complete_secs: mins * 60,
            })
        }
        (_, true, _, _) => {
            let mins: u64 = given_time.split('m').collect::<String>().parse()?;
            Ok(CountdownTime {
                mins_and_secs: (mins, 0),
                complete_secs: mins * 60,
            })
        }
        (_, _, true, _) => {
            let secs: u64 = given_time.split("sec").collect::<String>().parse()?;
            Ok(CountdownTime {
                mins_and_secs: (0, secs),
                complete_secs: secs,
            })
        }
        (_, _, _, true) => {
            let secs: u64 = given_time.split('s').collect::<String>().parse()?;
            Ok(CountdownTime {
                mins_and_secs: (0, secs),
                complete_secs: secs,
            })
        }
        _ => {
            let secs: u64 = given_time.parse()?;
            Ok(CountdownTime {
                mins_and_secs: (0, secs),
                complete_secs: secs,
            })
        }
    }
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

    let time = parse_time(&args.time).expect("Unable to parse the time correctly.");
    countdown(time.complete_secs);
}
