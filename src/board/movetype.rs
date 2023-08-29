
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MoveType {
    EnPassant = 1,
    Castle = 2,
    Promotion = 3,
    CaptureOrMove = 4,
}