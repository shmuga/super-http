use clap::Clap;

#[derive(Clap, Clone)]
#[clap(version = "1.0", author = "Mark O. <mail.ormark@gmail.com>")]
pub struct Opts {
    /// Some input. Because this isn't an Option<T> it's required to be used
    pub input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
}

pub fn get_opts() -> Opts {
    let opts: Opts = Opts::parse();
    opts
}
