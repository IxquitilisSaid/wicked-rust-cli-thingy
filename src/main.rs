use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "raul", short = "d", default_value = "8")]
    raul: usize,

    #[structopt(short)]
    which_champ: String,

    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let the_champ = args.which_champ;
    let mut file = " ";

    if the_champ == "twoTimer" {
        file = "theTwoTimer.txt";
    }

    if the_champ == "forsenCD" {
        file = "forsenCD.txt";
    }

    read_file(file)?
        .lines()
        .take(args.raul)
        .for_each(|line| println!("{}", line));

    Ok(())
}