use std::fs::read_to_string;
use std::path::PathBuf;

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

use grrs::find_matches;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let content = read_to_string(&args.path).with_context(|_| format!("could not read `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
