use clap::{App, Arg};

use super::viztype::VizType;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProgramState {
    pub viz_type: VizType,
    pub human_plays: bool,
    pub tick_speed: u64,
    pub num_plies: u32,
}

pub fn get_args() -> Result<ProgramState, Box<dyn std::error::Error>> {
    let matches = App::new("Chrust")
        .version("0.2.0")
        .author("John YB. <jyenterbriars@gmail.com>")
        .about("Simple Chess Engine")
        .arg(
            Arg::new("viz")
                .short('z')
                .long("visualization_mode")
                .value_name("TERM|WEB")
                .about("Sets the method of visualization for the app")
                .takes_value(true)
                .default_value("WEB")
        )
        .arg(
            Arg::new("hplay")
                .short('h')
                .long("human_plays")
                .value_name("true|false")
                .about("Sets whether or not the human player will play the game. If false, the human player makes random decisions")
                .takes_value(true)
                .default_value("true")
        )
        .arg(
            Arg::new("tick")
                .short('t')
                .long("tick_speed")
                .value_name("positive integer")
                .about("Sets the interval between moves. (Milliseconds)")
                .takes_value(true)
                .default_value("1000")
        )
        .arg(
            Arg::new("plies")
                .short('p')
                .long("num_plies")
                .value_name("positive integer")
                .about("Number of turns into the future the AI will look ahead.")
                .takes_value(true)
                .default_value("3")
        )
        .get_matches();

    //this is horrible and i'm sorry
    let viz_type = matches.value_of("viz").ok_or("unable to parse arguments :(")?;
    let human_plays = matches.value_of("hplay").ok_or("unable to parse arguments :(")?;
    let tick_speed = matches.value_of("tick").ok_or("unable to parse arguments :(")?;
    let num_plies = matches.value_of("plies").ok_or("unable to parse arguments :(")?;

    let program_state = ProgramState {
        viz_type: match viz_type {
            "TERM" => VizType::TERM,
            "WEB" => VizType::WEB,
            _ => {
                return Err(Box::from(
                    "You must pass in a vaid argument for the -v flag!",
                ));
            }
        },
        human_plays: match human_plays {
            "true" => true,
            "false" => false,
            _ => {
                return Err(Box::from(
                    "You must pass in a vaid argument for the -h flag!",
                ));
            }
        },
        tick_speed: tick_speed.parse()?,
        num_plies: num_plies.parse()?,
    };

    Ok(program_state)
}
