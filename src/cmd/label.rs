use anyhow::{bail, Result};
use clap::Parser;
use names::Generator as NamesGenerator;

#[derive(Parser)]
#[clap(visible_aliases = ["l", "name" ])]
/// Generate random name
pub struct LabelArgs {
    /// Number of names to generate
    pub times: u32,
}

pub fn generate_label(args: &LabelArgs) -> Result<()> {
    if args.times == 0 {
        bail!("times argument must be greater than zero.");
    }

    let mut generator = NamesGenerator::default();

    for _ in 0..args.times {
        println!("{}", generator.next().unwrap());
    }

    Ok(())
}
