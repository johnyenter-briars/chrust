use clap::{App, Arg};

use super::viztype::VizType;

#[derive(Debug)]
pub struct ProgramState {
    pub viz_type: VizType,
    pub human_plays: bool,
    pub tick_speed: u64,
}


pub fn get_args() -> Result<ProgramState, Box<dyn std::error::Error>> {
    let matches = App::new("Chrust")
        .version("0.1")
        .author("John YB. <jyenterbriars@gmail.com>")
        .about("Simple Chess Engine")
        .arg(
            Arg::new("viz")
                .short('z')
                .long("visualization_mode")
                .value_name("GUI|TERM")
                .about("Sets the method of visualization for the app")
                .takes_value(true)
                .default_value("TERM")
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
        .get_matches();

    //this is horrible and i'm sorry
    let viz_type = matches.value_of("viz").ok_or("idk")?;
    let human_plays = matches.value_of("hplay").ok_or("idk")?;
    let tick_speed = matches.value_of("tick").ok_or("idk")?;

    let program_state = ProgramState {
        viz_type: match viz_type {
            "GUI" => VizType::GUI,
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
    };

    Ok(program_state)
}