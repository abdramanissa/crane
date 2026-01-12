#[allow(unused_imports)]
// use crate::project::{create, Project};
use clap::Parser;
mod project;
use project::{Languages, Project};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    language: Languages,
}

#[allow(warnings)]
fn main() {
    let args = Args::parse();

    let project: Project = Project::new(args.name, args.language);
    project.create();
}
