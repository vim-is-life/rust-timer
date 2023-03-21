/*
 * want to have
 * - TODO logic to make the display update and have the timer do what it does
 * - TODO a display of the timer
 * - TODO accepting cli params to set the timer
 * - TODO pretty print time with the timer
 * - TODO functionality to play a ticking sound every second
 * - TODO add functionality to be a stopwatch as well
 */

use std::time::Instant;

// We make the parameter a u64 because the current time will come as a u64.
fn countdown(end_time: u64) {
    let start_time = Instant::now();
    let mut last_time: u64 = 0;
    loop {
        let current_time = start_time.elapsed().as_secs();

        if current_time == end_time {
            println!("done! {} seconds have passed", current_time);
            break;
        } else if current_time != last_time {
            println!("{} seconds have passed", current_time);
            last_time += 1;
        }
    }
}

fn main() {
    countdown(5);
}
