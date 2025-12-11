use clap::Parser;
use RCLI::{Opts, SubCommand,process_csv};

fn main() -> Result<(),anyhow::Error> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
