mod opts;
mod process;
pub use opts::{CsvOpts, Opts, OutputFormat, SubCommand};
pub use process::{Record, gen_pass, process_csv};
