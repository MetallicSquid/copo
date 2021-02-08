use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

// User input
#[derive(StructOpt)]
struct Cli {
    /// Duration of work timer in minutes.
    work: String,
    /// Duration of rest timer in minutes.
    rest: String,
}

fn main() -> Result<(), ExitFailure> {
    // Parse through user inputs and flags
    let args = Cli::from_args();
    let work = args
        .work
        .trim()
        .parse::<i32>()
        .with_context(|_| format!("Could not read work duration `{}`", args.work))?;
    let rest = args
        .rest
        .trim()
        .parse::<i32>()
        .with_context(|_| format!("Could not read rest duration `{}`", args.rest))?;

    // Run Pomodoro timer
    copo::pomodoro(work, rest);
    Ok(())
}
