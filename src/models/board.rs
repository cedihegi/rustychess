use super::{
    field_content::FieldContent,
    location::Location,
    piece::{ColoredPiece, PieceColor, PieceKind},
    step::{Direction, Step, StepKind},
};

#[derive(Clone, Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    fields: Vec<FieldContent>,
    turn: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::standard_board()
    }
}

impl Board {
    /// Creates an empty field
    pub fn new(width: usize, height: usize) -> Self {
        let fields = (0..width * height).map(|_| FieldContent::Empty).collect();
        Self {
            height,
            width,
            fields,
            turn: 0,
        }
    }

    pub fn location_contains_piece(
        &self,
        location: &Location,
        piece_kinds: Vec<PieceKind>,
        target_color: PieceColor,
    ) -> bool {
        let field_opt = self.field_at_location(location);
        if let Some(FieldContent::Occupied {
            piece: ColoredPiece { kind, color },
            ..
        }) = field_opt
        {
            piece_kinds.contains(kind) && *color == target_color
        } else {
            false
        }
    }

    pub fn location_piece_unmoved(&self, location: &Location) -> bool {
        let field_opt = self.field_at_location(location);
        matches!(field_opt, Some(FieldContent::Occupied { turn: 0, .. }))
    }

    pub fn location_is_emtpy(&self, location: &Location) -> bool {
        let field_opt = self.field_at_location(location);
        match field_opt {
            Some(FieldContent::Empty) => true,
            Some(_) => false,
            None => panic!("indexing error"),
        }
    }

    pub fn find_king(&self, target_color: PieceColor) -> Option<Location> {
        self.locations().into_iter().find(|loc| {
            if let Some(FieldContent::Occupied {
                piece:
                    ColoredPiece {
                        color,
                        kind: PieceKind::King,
                    },
                ..
            }) = self.field_at_location(loc)
            {
                *color == target_color
            } else {
                false
            }
        })
    }

    pub fn locations(&self) -> Vec<Location> {
        (0..(self.height * self.height))
            .map(|idx| self.revert_compute_location(idx))
            .collect()
    }

    /// add a direction to a location, getting a new location
    pub fn add_location_direction(
        &self,
        location: &Location,
        direction: &Direction,
    ) -> Option<Location> {
        let res = Location {
            x: (location.x as isize + direction.0) as usize,
            y: (location.y as isize + direction.1) as usize,
        };
        if self.in_bounds(&res) {
            Some(res)
        } else {
            None
        }
    }

    fn compute_index_xy(&self, x: usize, y: usize) -> usize {
        // check that index is within bounds?
        self.width * y + x
    }

    fn revert_compute_location(&self, idx: usize) -> Location {
        let (x, y) = self.revert_compute_index(idx);
        Location { x, y }
    }
    fn revert_compute_index(&self, idx: usize) -> (usize, usize) {
        let y = idx / self.width;
        let x = idx % self.width;
        (x, y)
    }

    pub fn turn_color(&self) -> PieceColor {
        if self.turn.is_multiple_of(2) {
            PieceColor::White
        } else {
            PieceColor::Black
        }
    }

    pub fn field_at_xy(&self, x: usize, y: usize) -> Option<&FieldContent> {
        let index = self.compute_index_xy(x, y);
        self.fields.get(index)
    }
    pub fn field_at_xy_mut(&mut self, x: usize, y: usize) -> Option<&mut FieldContent> {
        let index = self.compute_index_xy(x, y);
        self.fields.get_mut(index)
    }

    pub fn field_at_location(&self, location: &Location) -> Option<&FieldContent> {
        self.field_at_xy(location.x, location.y)
    }

    pub fn field_at_location_mut(&mut self, location: &Location) -> Option<&mut FieldContent> {
        self.field_at_xy_mut(location.x, location.y)
    }

    pub fn in_bounds(&self, location: &Location) -> bool {
        location.x < self.width && location.y < self.height
    }

    fn set_field_empty(&mut self, location: &Location) {
        let field = self
            .field_at_location_mut(location)
            .expect("Failed getting field");
        *field = FieldContent::Empty;
    }

    pub fn put_piece_on_field_xy(
        &mut self,
        colored_piece: ColoredPiece,
        (x, y): (usize, usize),
        turn: usize,
    ) {
        let index = self.compute_index_xy(x, y);
        let field = self
            .fields
            .get_mut(index)
            .expect("Indexing invalid field!!");
        *field = FieldContent::Occupied {
            piece: colored_piece,
            turn,
        }
    }

    pub fn put_piece_on_location(
        &mut self,
        colored_piece: ColoredPiece,
        location: &Location,
        turn: usize,
    ) {
        self.put_piece_on_field_xy(colored_piece, (location.x, location.y), turn);
    }

    pub fn apply_step_kind(&mut self, step_kind: &StepKind) -> Result<(), String> {
        match step_kind {
            StepKind::GoTo(step) => self.apply_step(step)?,
            StepKind::Castle {
                king_step,
                rook_step,
            } => {
                self.apply_step(king_step)?;
                self.apply_step(rook_step)?;
            }
            StepKind::Promote {
                step: Step { from, to },
                piece,
            } => {
                self.put_piece_on_location(
                    ColoredPiece {
                        kind: *piece,
                        color: self.turn_color(),
                    },
                    to,
                    self.turn,
                );
                self.set_field_empty(from);
            }
        }
        self.turn += 1;
        Ok(())
    }

    pub fn apply_step(&mut self, step: &Step) -> Result<(), String> {
        let moved_field = self.field_at_location(&step.from);
        if let Some(FieldContent::Occupied { piece, .. }) = moved_field {
            self.put_piece_on_location(*piece, &step.to, self.turn);
            self.set_field_empty(&step.from);
            Ok(())
        } else {
            Err(format!("Empty or non-existent field: {:?}", step.from))
        }
    }

    pub fn to_pretty_string(&self) -> String {
        let line = "-".repeat(self.width * 4 + 1);
        let middle_line = "|---".repeat(self.width) + "|\n";
        let mut result = line.clone();
        result += "\n";
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let field = self.field_at_xy(x, y).unwrap();
                let icon = field.to_pretty_string();
                result += &format!("| {icon} ");
            }
            result += "|\n";
            if y != 0 {
                result += &middle_line;
            }
        }
        result += &line;
        result
    }

    pub fn standard_board() -> Self {
        let mut board = Self::new(8, 8);

        // standard pieces:
        let white_pawn = ColoredPiece {
            kind: PieceKind::Pawn,
            color: PieceColor::White,
        };
        let black_pawn = ColoredPiece {
            kind: PieceKind::Pawn,
            color: PieceColor::Black,
        };
        for i in 0..board.width {
            board.put_piece_on_field_xy(white_pawn, (i, 1), 0);
            board.put_piece_on_field_xy(black_pawn, (i, 6), 0);
        }

        let white_rook = ColoredPiece {
            kind: PieceKind::Rook,
            color: PieceColor::White,
        };
        let black_rook = ColoredPiece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
        };
        board.put_piece_on_field_xy(white_rook, (0, 0), 0);
        board.put_piece_on_field_xy(white_rook, (7, 0), 0);
        board.put_piece_on_field_xy(black_rook, (0, 7), 0);
        board.put_piece_on_field_xy(black_rook, (7, 7), 0);

        let white_knight = ColoredPiece {
            kind: PieceKind::Knight,
            color: PieceColor::White,
        };
        let black_knight = ColoredPiece {
            kind: PieceKind::Knight,
            color: PieceColor::Black,
        };
        board.put_piece_on_field_xy(white_knight, (1, 0), 0);
        board.put_piece_on_field_xy(white_knight, (6, 0), 0);
        board.put_piece_on_field_xy(black_knight, (1, 7), 0);
        board.put_piece_on_field_xy(black_knight, (6, 7), 0);

        let white_bishop = ColoredPiece {
            kind: PieceKind::Bishop,
            color: PieceColor::White,
        };
        let black_bishop = ColoredPiece {
            kind: PieceKind::Bishop,
            color: PieceColor::Black,
        };
        board.put_piece_on_field_xy(white_bishop, (2, 0), 0);
        board.put_piece_on_field_xy(white_bishop, (5, 0), 0);
        board.put_piece_on_field_xy(black_bishop, (2, 7), 0);
        board.put_piece_on_field_xy(black_bishop, (5, 7), 0);

        let white_queen = ColoredPiece {
            kind: PieceKind::Queen,
            color: PieceColor::White,
        };
        let black_queen = ColoredPiece {
            kind: PieceKind::Queen,
            color: PieceColor::Black,
        };
        board.put_piece_on_field_xy(white_queen, (3, 0), 0);
        board.put_piece_on_field_xy(black_queen, (3, 7), 0);

        let white_king = ColoredPiece {
            kind: PieceKind::King,
            color: PieceColor::White,
        };
        let black_king = ColoredPiece {
            kind: PieceKind::King,
            color: PieceColor::Black,
        };
        board.put_piece_on_field_xy(white_king, (4, 0), 0);
        board.put_piece_on_field_xy(black_king, (4, 7), 0);

        board
    }
}
