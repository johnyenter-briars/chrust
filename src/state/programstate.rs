use clap::{App, Arg};

use crate::ChrustResult;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProgramState {
    pub num_plies: u32,
}

pub fn get_args() -> ChrustResult<ProgramState> {
    let matches = App::new("Chrust")
        .version("0.2.0")
        .author("John YB. <jyenterbriars@gmail.com>")
        .about("Simple Chess Engine")
        .arg(
            Arg::new("plies")
                .short('p')
                .long("num_plies")
                .value_name("positive integer")
                .takes_value(true)
                .default_value("3"),
        )
        .get_matches();

    let num_plies = matches
        .value_of("plies")
        .ok_or("unable to parse arguments :(")?;

    let program_state = ProgramState {
        num_plies: num_plies.parse()?,
    };

    Ok(program_state)
}
