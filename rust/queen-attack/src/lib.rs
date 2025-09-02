#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        //        todo!(
        //            "Construct a ChessPosition struct, given the following rank, file: ({rank}, {file}). If the position is invalid return None."
        //        );
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // todo!("Given the chess position {position:?}, construct a Queen struct.");
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        // todo!("Determine if this Queen can attack the other Queen {other:?}");
        let mp = &self.position;
        let op = &other.position;
        mp.rank == op.rank
            || mp.file == op.file
            || ((mp.rank - op.rank).abs() == (mp.file - op.file).abs())
    }
}
