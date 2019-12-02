use model::classification::Classification;

pub struct ChessMove {
    pub piece: Classification,
    pub destination: (u32, u32),
}
