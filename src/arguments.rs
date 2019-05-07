use std::env;
use getopts::Options;

#[derive(Debug)]
pub struct Arguments {
  pub input_file: Option<String>,
  pub output_file: Option<String>,
  pub debug: bool,
}
impl Arguments {
    pub fn parse() -> Arguments {
        let args: Vec<String> = env::args().collect();

        let mut opts = Options::new();
        opts.optopt("i", "input", "set input record file name.", "NAME");
        opts.optopt("o", "output", "set output record file name.", "NAME");
        opts.optflag("d", "debug", "Debug mode.");

        let matches = opts.parse(&args[1..])
            .unwrap_or_else(|f| panic!(f.to_string()));

        Arguments {
            input_file: matches.opt_str("input"),
            output_file: matches.opt_str("output"),
            debug: matches.opt_present("debug"),
        }
    }
}
