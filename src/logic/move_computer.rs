use crate::models::{
    board::Board,
    field_content::FieldContent,
    location::Location,
    piece::{ColoredPiece, MoveCapability, PieceColor, PieceKind},
    step::{Step, StepKind},
};

use super::basic_evaluation::BasicEvaluation;

pub trait StepComputer {
    fn compute_steps(&self) -> Vec<StepKind>;
    fn has_check(&self, color_opt: Option<PieceColor>) -> bool;
    fn evaluate_basic(&self) -> BasicEvaluation;
}

impl StepComputer for Board {
    fn evaluate_basic(&self) -> BasicEvaluation {
        let possible_moves = self.compute_steps();
        let has_check = self.has_check(None);
        BasicEvaluation {
            has_check,
            has_checkmate: possible_moves.is_empty() && has_check,
            has_stalemate: possible_moves.is_empty() && !has_check,
            possible_moves,
        }
    }

    fn compute_steps(&self) -> Vec<StepKind> {
        let mut steps = self.compute_simple_steps(None);
        self.extend_promotions(&mut steps);
        steps.append(&mut self.castle_moves());
        self.filter_check_steps(&mut steps);

        steps
    }
    /// this function is ugly a.f.
    /// tries to be a little more efficient than just computing all the possible moves in a
    /// position.
    fn has_check(&self, color_opt: Option<PieceColor>) -> bool {
        self.checked_by_steps(color_opt, None)
    }
}

impl Board {
    pub fn checked_by_steps(
        &self,
        color_opt: Option<PieceColor>,
        steps_opt: Option<Vec<StepKind>>,
    ) -> bool {
        let color = color_opt.unwrap_or(self.turn_color());
        let king_position = self.find_king(color).expect("No king on the board");
        let steps = steps_opt.unwrap_or_else(|| self.compute_simple_steps(Some(color.invert())));
        steps.iter().any(|step| {
            let target = step.target();
            target.is_some() && target.unwrap() == king_position
        })
    }

    /// computes steps just based on the piece's capabilities,
    /// ignoring, whether there is a check
    pub fn compute_simple_steps(&self, color_opt: Option<PieceColor>) -> Vec<StepKind> {
        let color = color_opt.unwrap_or(self.turn_color());
        self.locations()
            .into_iter()
            .flat_map(|location| self.compute_field_steps(&location, color))
            .collect()
    }

    /// compute the possible castles
    pub fn castle_moves(&self) -> Vec<StepKind> {
        let color = self.turn_color();
        let rank = if color == PieceColor::White {
            0
        } else {
            self.height - 1
        };
        let king_location = Location { x: 4, y: rank };
        // 1. check king hasn't moved
        let king_field_opt = self.field_at_location(&king_location);
        match king_field_opt {
            Some(FieldContent::Occupied {
                piece:
                    ColoredPiece {
                        kind: PieceKind::King,
                        ..
                    },
                turn: 0,
            }) => {
                let mut res = vec![];
                // queenside:
                // contains rook that hasn't been moved
                let rook1_loc = Location::new(0, rank);
                if self.location_contains_piece(&rook1_loc, vec![PieceKind::Rook], color) && self.location_piece_unmoved(&rook1_loc) && (1..3).all(|x| self.location_is_emtpy(&Location::new(x, rank))) {
                    res.push(StepKind::Castle {
                        king_step: Step::new((4, rank), (2, rank)),
                        rook_step: Step::new((0, rank), (3, rank)),
                    })
                }

                // kingside
                let rook2_loc = Location::new(self.width - 1, rank);
                if self.location_contains_piece(&rook2_loc, vec![PieceKind::Rook], color) && self.location_piece_unmoved(&rook2_loc) && (5..=6).all(|x| self.location_is_emtpy(&Location::new(x, rank))) {
                    res.push(StepKind::Castle {
                        king_step: Step::new((4, rank), (6, rank)),
                        rook_step: Step::new((self.width - 1, rank), (5, rank)),
                    })
                }

                res
            }
            _ => vec![],
        }
    }

    pub fn filter_check_steps(&self, steps: &mut Vec<StepKind>) {
        let mut remove: Vec<usize> = vec![];
        let color = self.turn_color();
        for (index, step) in steps.iter().enumerate() {
            let mut new_board: Board = self.clone();
            new_board.apply_step_kind(step).unwrap();
            if new_board.has_check(Some(color)) {
                remove.push(index)
            }
        }
        remove.reverse();
        for idx in remove {
            steps.remove(idx);
        }
    }

    pub fn extend_promotions(&self, steps: &mut Vec<StepKind>) {
        let last_row = if self.turn_color() == PieceColor::White {
            self.height - 1
        } else {
            0
        };
        let mut promotable: Vec<usize> = vec![];
        for (index, step) in steps.iter().enumerate() {
            // check if it's a pawn, moving to last row
            if let StepKind::GoTo(Step { from, to }) = step {
                if to.y != last_row {
                    return;
                }
                // if this panics, something with the move computation is wrong
                let field = self.field_at_location(from).unwrap();
                if let FieldContent::Occupied {
                    piece:
                        ColoredPiece {
                            kind: PieceKind::Pawn,
                            ..
                        },
                    ..
                } = field
                {
                    promotable.push(index);
                }
            }
        }
        promotable.reverse();
        for index in promotable {
            let step_kind = steps.remove(index);
            if let StepKind::GoTo(step) = step_kind {
                steps.append(&mut StepKind::possible_promotions(step));
            } else {
                unreachable!()
            }
        }
    }

    pub fn compute_field_steps(
        &self,
        location: &Location,
        turn_color: PieceColor,
    ) -> Vec<StepKind> {
        let field_content_opt = self.field_at_location(location);
        if let Some(FieldContent::Occupied { piece, .. }) = field_content_opt {
            if piece.color != turn_color {
                return vec![];
            }
            piece
                .piece_move_capabilities()
                .into_iter()
                .flat_map(|cap| {
                    self.compute_steps_with_capablity(location, location, 0, piece.color, &cap)
                })
                .collect::<Vec<StepKind>>()
        } else {
            vec![]
        }
    }

    fn compute_steps_with_capablity(
        &self,
        origin_location: &Location,
        current_location: &Location,
        distance_traveled: u8,
        color: PieceColor,
        cap: &MoveCapability,
    ) -> Vec<StepKind> {
        // distance left?
        if cap.distance != -1 && distance_traveled as i16 >= cap.distance {
            return vec![];
        }

        // is next location on board?
        if let Some(next_location) = self.add_location_direction(current_location, &cap.direction) {
            // is there a piece at that location?
            let field_opt = self.field_at_location(&next_location);
            if let Some(FieldContent::Occupied { piece, .. }) = field_opt {
                // the case where there is a piece.
                if !cap.can_take || piece.color == color {
                    // if the piece is the same color, or the current piece is not
                    // allowed to take:
                    vec![]
                } else {
                    // piece is different color, and we are allowed to take:
                    vec![StepKind::GoTo(Step {
                        from: *origin_location,
                        to: next_location,
                    })]
                }
            } else {
                // the case where there is no piece
                if cap.must_take {
                    vec![]
                } else {
                    let mut res_recursive = self.compute_steps_with_capablity(
                        origin_location,
                        &next_location,
                        distance_traveled + 1,
                        color,
                        cap,
                    );
                    let step = StepKind::GoTo(Step {
                        from: *origin_location,
                        to: next_location,
                    });
                    res_recursive.push(step);
                    res_recursive
                }
            }
        } else {
            vec![]
        }
    }
}
