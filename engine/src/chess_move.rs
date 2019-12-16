use model::classification::Classification;

pub struct ChessMove {
    pub piece: Classification,
    pub destination: (i32, i32),
    pub origin: (i32, i32)
}
