use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
struct Args {
    /// Lets you input mutiple files to create using spaces
    name: Vec<String>,
}

fn main() {

    let args = Args::parse();

    args.name.into_iter().for_each(|name|{

        File::create(name);

    });

}
