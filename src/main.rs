use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;

// User input
#[derive(StructOpt)]
struct Cli {
    work: String,
    rest: String,
}

fn main() -> Result<(), ExitFailure>{
    // Convert duration strings to integer
    let args = Cli::from_args();
    let work = args.work.trim().parse::<i32>()
        .with_context(|_| format!("Could not read work duration `{}`", args.work))?;
    let rest = args.rest.trim().parse::<i32>()
        .with_context(|_| format!("Could not read rest duration `{}`", args.rest))?;

    // Run Pomodoro timer
    copo::pomodoro(work, rest);
    Ok(())
}
