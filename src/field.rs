use crate::{command::Move, draw::CellArt, error::BishopArtError};
use std::convert::TryInto;

pub struct Field {
    current_cell_index: usize,
    cells: Vec<Cell>,
}

const FIELD: [&str; 9] = [
    /*               1111111*/
    /*     01234567890123456*/
    /*1*/ "aTTTTTTTTTTTTTTTb",
    /*2*/ "LMMMMMMMMMMMMMMMR",
    /*3*/ "LMMMMMMMMMMMMMMMR",
    /*4*/ "LMMMMMMMMMMMMMMMR",
    /*5*/ "LMMMMMMMMMMMMMMMR",
    /*6*/ "LMMMMMMMMMMMMMMMR",
    /*7*/ "LMMMMMMMMMMMMMMMR",
    /*8*/ "LMMMMMMMMMMMMMMMR",
    /*9*/ "cBBBBBBBBBBBBBBBd",
];

const FIELD_ROWS: usize = FIELD.len();
const FIELD_COLUMNS: usize = FIELD[0].len();
const FIELD_LENGTH: usize = FIELD_ROWS * FIELD_COLUMNS;
const CENTER_INDEX: usize = FIELD_LENGTH / 2;
const FRAME_HORIZONTAL: &str = "═";
const FRAME_VERTICAL: &str = "║";
const FRAME_CORNER: &str = "╬";

impl Field {
    pub fn new() -> Self {
        let mut cells: Vec<_> = FIELD
            .join("")
            .chars()
            .map(|ch| match ch {
                'M' => CellKind::Middle,
                'T' => CellKind::Top,
                'B' => CellKind::Bottom,
                'L' => CellKind::Left,
                'R' => CellKind::Right,
                'a' => CellKind::TopLeft,
                'b' => CellKind::TopRight,
                'c' => CellKind::BottomLeft,
                'd' => CellKind::BottomRight,
                _ => unreachable!("Table is predefined, can't have invalid chars"),
            })
            .map(|kind| Cell {
                art: CellArt::Empty,
                kind,
            })
            .collect();

        cells[CENTER_INDEX].art = CellArt::Start;

        Field {
            cells,
            current_cell_index: CENTER_INDEX,
        }
    }

    pub fn make_move(&mut self, direction: Move) {
        let current_cell = &mut self.cells[self.current_cell_index];
        let offset = current_cell.get_offset(direction);
        let new_cell_index = (self.current_cell_index as isize) + offset;
        self.current_cell_index = new_cell_index
            .try_into()
            .map_err(|_| BishopArtError::InvalidCellIndex {
                index: new_cell_index,
            })
            .expect("Couldn't make a move");
        let new_cell = &mut self.cells[self.current_cell_index];
        new_cell.inc();
    }

    pub fn finalize(&mut self) {
        let current_cell = &mut self.cells[self.current_cell_index];
        current_cell.art = CellArt::End;
    }

    pub fn draw(&self) {
        let field_columns_plus_border = FIELD_COLUMNS;
        let border: String = [
            FRAME_CORNER,
            &(0..field_columns_plus_border)
                .into_iter()
                .map(|_| FRAME_HORIZONTAL)
                .collect::<String>(),
            FRAME_CORNER,
        ]
        .join("");
        println!("{}", border);

        let field = self
            .cells
            .chunks_exact(FIELD_COLUMNS)
            .map(|row| {
                row.iter()
                    .map(|cell| cell.art.to_string())
                    .collect::<String>()
            })
            .fold(String::new(), |mut acc, el| {
                acc.push_str(FRAME_VERTICAL);
                acc.push_str(&el);
                acc.push_str(FRAME_VERTICAL);
                acc.push('\n');
                acc
            });

        print!("{}", field);
        println!("{}", border);
    }
}

struct Cell {
    kind: CellKind,
    art: CellArt,
}

impl Cell {
    pub fn get_offset(&self, direction: Move) -> isize {
        match direction {
            Move::NorthWest => match self.kind {
                CellKind::Middle | CellKind::Bottom | CellKind::Right | CellKind::BottomRight => {
                    -18
                }
                CellKind::Left | CellKind::BottomLeft => -17,
                CellKind::Top | CellKind::TopRight => -1,
                CellKind::TopLeft => 0,
            },
            Move::NorthEast => match self.kind {
                CellKind::Middle | CellKind::Bottom | CellKind::Left | CellKind::BottomLeft => -16,
                CellKind::Right | CellKind::BottomRight => -17,
                CellKind::Top | CellKind::TopLeft => 1,
                CellKind::TopRight => 0,
            },
            Move::SouthWest => match self.kind {
                CellKind::TopLeft | CellKind::Left => 17,
                CellKind::Top | CellKind::Middle | CellKind::Right | CellKind::TopRight => 16,
                CellKind::Bottom | CellKind::BottomRight => -1,
                CellKind::BottomLeft => 0,
            },
            Move::SouthEast => match self.kind {
                CellKind::Middle | CellKind::Top | CellKind::Left | CellKind::TopLeft => 18,
                CellKind::Right | CellKind::TopRight => 17,
                CellKind::Bottom | CellKind::BottomLeft => 1,
                CellKind::BottomRight => 0,
            },
        }
    }

    pub fn inc(&mut self) {
        self.art = self.art.inc();
    }
}

enum CellKind {
    Middle,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
