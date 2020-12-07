#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen{
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            return None;
        }

        Some(ChessPosition(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let position_1 = &self.position;
        let position_2 = &other.position;

        if  position_1.0 == position_2.0 || 
            position_1.1 == position_2.1 ||
            (position_1.0 - position_2.0).abs() == (position_1.1 - position_2.1).abs() {
            return true;
        } else {
            return false;
        }
    }
}
