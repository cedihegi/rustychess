use super::{
    location::Location,
    piece::{PieceColor, PieceKind},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    pub from: Location,
    pub to: Location,
}

#[derive(Debug, Clone)]
pub struct Direction(pub isize, pub isize);

#[derive(Debug, Clone, PartialEq)]
pub enum StepKind {
    GoTo(Step),
    Promote { step: Step, piece: PieceKind },
    Castle { king_step: Step, rook_step: Step },
}

impl Step {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            from: Location {
                x: from.0,
                y: from.1,
            },
            to: Location { x: to.0, y: to.1 },
        }
    }

    pub fn encode(&self) -> String {
        format!("{}{}", self.from.encode(), self.to.encode())
    }
}

impl Direction {
    pub fn reverse(&self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl StepKind {
    pub fn possible_promotions(step: Step) -> Vec<StepKind> {
        vec![
            StepKind::Promote {
                step,
                piece: PieceKind::Pawn,
            },
            StepKind::Promote {
                step,
                piece: PieceKind::Rook,
            },
            StepKind::Promote {
                step,
                piece: PieceKind::Knight,
            },
            StepKind::Promote {
                step,
                piece: PieceKind::Bishop,
            },
            StepKind::Promote {
                step,
                piece: PieceKind::Queen,
            },
            StepKind::Promote {
                step,
                piece: PieceKind::King,
            },
        ]
    }

    pub fn target(&self) -> Option<Location> {
        match self {
            StepKind::GoTo(step) => Some(step.to),
            StepKind::Promote { step, .. } => Some(step.to),
            _ => None,
        }
    }

    // only simple encoding
    pub fn encode(&self) -> String {
        match self {
            StepKind::GoTo(step) => step.encode(),
            StepKind::Promote { step, piece } => format!("{}={}", step.encode(), piece.encode()),
            StepKind::Castle { rook_step, .. } => {
                if rook_step.from.x == 0 {
                    // long castle:
                    "0-0-0".to_string()
                } else {
                    "0-0".to_string()
                }
            }
        }
    }

    pub fn decode(input: &str, current_color: PieceColor) -> Result<Self, String> {
        let rank = if current_color == PieceColor::White {
            0
        } else {
            7
        };
        match input {
            "O-O" => {
                let rook_step = Step::new((7, rank), (5, rank));
                let king_step = Step::new((4, rank), (6, rank));
                Ok(StepKind::Castle {
                    king_step,
                    rook_step,
                })
            }
            //"O-O-O" => Ok(Move::CastlingQueenside),
            _ if input.len() == 4 => {
                let (start, end) = input.split_at(2);
                Ok(StepKind::GoTo(Step {
                    from: Location::decode(start)?,
                    to: Location::decode(end)?,
                }))
            }
            _ if input.contains('=') => {
                if let Some(eq_pos) = input.find('=') {
                    let promotion = input.chars().nth(eq_pos + 1).ok_or("Invalid promotion")?;
                    let piece = PieceKind::decode(promotion)?;
                    let (start, end) = input.split_at(eq_pos);
                    Ok(StepKind::Promote {
                        step: Step {
                            from: Location::decode(start)?,
                            to: Location::decode(end)?,
                        },
                        piece,
                    })
                } else {
                    Err("Invalid promotion format".to_string())
                }
            }
            _ => Err("Invalid move format".to_string()),
        }
    }
}
