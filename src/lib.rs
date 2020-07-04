use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};
use console::{Term, Style};
use notifica::notify;

// Basic pomodoro timer
pub fn pomodoro(work: i32, rest: i32) {
    // Set up information variables
    let mut work_count: i32 = 0;
    let mut rest_count: i32 = 0;

    // Timer loop
    let work_secs: i32 = work * 60;
    let rest_secs: i32 = rest * 60;
    let term = Term::stdout();
    term.clear_screen().expect("Could not clear screen.");

    loop {
        notify("🍅", "Time to work.");
        term.clear_screen().expect("Could not clear screen.");
        user_info(true, work_count, rest_count, work_secs, rest_secs);

        // Work timer
        let work_bar = ProgressBar::new(work_secs as u64);
        work_bar.set_style(ProgressStyle::default_bar()
                           .template("{spinner:.red} [{elapsed_precise}] [{bar:60.orange/red}]")
                           .progress_chars("#🍅-"));
        for _i in 0..work_secs {
            work_bar.inc(1);
            thread::sleep(time::Duration::from_secs(1));
        }
        work_count += 1;

        notify("🍅", "Time to rest.");
        term.clear_screen().expect("Could not clear screen.");
        user_info(false, work_count, rest_count, work_secs, rest_secs);

        // Rest timer
        let rest_bar = ProgressBar::new(rest_secs as u64);
        rest_bar.set_style(ProgressStyle::default_bar()
                           .template("{spinner:.red} [{elapsed_precise}] [{bar:60.orange/red}]")
                           .progress_chars("#🍅-"));
        for _i in 0..rest_secs {
            rest_bar.inc(1);
            thread::sleep(time::Duration::from_secs(1));
        }
        rest_count += 1;
    }
}

// Gather timer-related information
pub fn user_info(work: bool, work_count: i32, rest_count: i32, work_secs: i32, rest_secs: i32) {
    let work_time = secs_to_time(work_count * work_secs);
    let rest_time = secs_to_time(rest_count * rest_secs);
    let total_time = secs_to_time((work_count * work_secs) + (rest_count * rest_secs));
    let cycle = match work {
        true => "Work",
        false => "Rest",
    };

    let cyan = Style::new().cyan();
    let message: String = format!("\n Press `Ctrl + C` to end session. \n\n Information: \n *  You have completed {} of work - [{} cycle(s)]\n *  You have completed {} of rest - [{} cycle(s)]\n *  The timer has run for a total of {} \n\n This is a {} cycle.\n", cyan.apply_to(work_time), cyan.apply_to(work_count), cyan.apply_to(rest_time), cyan.apply_to(rest_count), cyan.apply_to(total_time), cyan.apply_to(cycle));
    println!("{}", message);
}

// Convert seconds to a nice time string
pub fn secs_to_time(mut secs: i32) -> std::string::String {
    // Determine the values (this is done strangely as there is no modulo in Rust)
    let mut hours = 0;
    if secs % 3600 == 0 {
        hours = secs / 3600;
    }
    secs -= hours * 3600;
    let mut mins = 0;
    if secs % 60 == 0 {
        mins = secs / 60;
    }
    secs -= mins * 60;

    // Pad with zeros if neccessary
    let mut hour_string = hours.to_string();
    if hour_string.len() == 1 {
        hour_string = format!("{}{}", "0", hour_string);
    }
    let mut min_string = mins.to_string();
    if min_string.len() == 1 {
        min_string = format!("{}{}", "0", min_string);
    }
    let mut sec_string = secs.to_string();
    if sec_string.len() == 1 {
        sec_string = format!("{}{}", "0", sec_string);
    }

    // Format the string
    let time_string = format!("{}:{}:{}", hour_string, min_string, sec_string);
    return time_string;
}
