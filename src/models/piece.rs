use ansi_term::Color::{self, Red, Yellow};

use super::step::Direction;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ColoredPiece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

pub struct MoveCapability {
    pub direction: Direction,
    pub distance: i16,
    pub must_take: bool,
    pub can_take: bool,
}

impl PieceKind {
    pub fn to_symbol(&self) -> String {
        match self {
            PieceKind::Pawn => "\u{f0859}".to_string(),
            PieceKind::Bishop => "\u{f085c}".to_string(),
            PieceKind::Knight => "\u{f0858}".to_string(),
            PieceKind::Rook => "\u{f085b}".to_string(),
            PieceKind::Queen => "\u{f085a}".to_string(),
            PieceKind::King => "\u{f0857}".to_string(),
        }
    }

    pub fn encode(&self) -> String {
        match self {
            PieceKind::Pawn => "P".to_string(),
            PieceKind::Bishop => "B".to_string(),
            PieceKind::Knight => "N".to_string(),
            PieceKind::Rook => "R".to_string(),
            PieceKind::Queen => "Q".to_string(),
            PieceKind::King => "K".to_string(),
        }
    }

    pub fn decode(input: char) -> Result<PieceKind, String> {
        let input: char = input.to_ascii_lowercase();
        match input {
            'p' => Ok(PieceKind::Pawn),
            'b' => Ok(PieceKind::Bishop),
            'n' => Ok(PieceKind::Knight),
            'r' => Ok(PieceKind::Rook),
            'q' => Ok(PieceKind::Queen),
            'k' => Ok(PieceKind::King),
            c => Err(format!("failed decoding piece char {}", c)),
        }
    }
}

impl PieceColor {
    pub fn ansi_color(&self) -> Color {
        match self {
            PieceColor::White => Yellow,
            PieceColor::Black => Red,
        }
    }

    pub fn invert(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn decode(c: char) -> Result<Self, String> {
        let c = c.to_ascii_lowercase();
        match c {
            'w' => Ok(Self::White),
            'b' => Ok(Self::Black),
            _ => Err(format!("Invalid color! {}", c)),
        }
    }
}

impl ColoredPiece {
    pub fn to_colored_symbol(&self) -> String {
        let symbol = self.kind.to_symbol();
        let color = self.color.ansi_color();
        color.paint(symbol).to_string()
    }

    pub fn piece_move_capabilities(&self) -> Vec<MoveCapability> {
        match self.kind {
            PieceKind::Pawn => {
                let direction_y = if self.color == PieceColor::White {
                    1
                } else {
                    -1
                };
                vec![
                    MoveCapability {
                        direction: Direction(0, direction_y),
                        distance: 2,
                        must_take: false,
                        can_take: false,
                    },
                    MoveCapability {
                        direction: Direction(1, direction_y),
                        distance: 1,
                        must_take: true,
                        can_take: false,
                    },
                    MoveCapability {
                        direction: Direction(-1, direction_y),
                        distance: 1,
                        must_take: true,
                        can_take: false,
                    },
                ]
            }
            PieceKind::Rook => vec![
                MoveCapability {
                    direction: Direction(0, 1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(0, -1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, 0),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 0),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
            ],
            PieceKind::Knight => vec![
                MoveCapability {
                    direction: Direction(1, 2),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, -2),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 2),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, -2),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(2, 1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(2, -1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-2, 1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-2, -1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
            ],
            PieceKind::Bishop => vec![
                MoveCapability {
                    direction: Direction(1, 1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, -1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, -1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
            ],
            PieceKind::Queen => vec![
                MoveCapability {
                    direction: Direction(1, 1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, -1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, -1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(0, 1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(0, -1),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, 0),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 0),
                    distance: -1,
                    must_take: false,
                    can_take: true,
                },
            ],
            PieceKind::King => vec![
                MoveCapability {
                    direction: Direction(1, 1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, -1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, -1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(0, 1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(0, -1),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(1, 0),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
                MoveCapability {
                    direction: Direction(-1, 0),
                    distance: 1,
                    must_take: false,
                    can_take: true,
                },
            ],
        }
    }
}
