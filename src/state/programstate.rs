use super::viztype::VizType;

#[derive(Debug)]
pub struct ProgramState {
    pub viz_type: VizType,
    pub human_plays: bool,
    pub tick_speed: u64,
}