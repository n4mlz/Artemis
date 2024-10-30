use crate::*;
use std::{collections::VecDeque, fmt};
use termion::cursor;

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_state(f, self, None)?;

        write!(f, "{}", cursor::Goto(0, 26))?;

        fmt::Result::Ok(())
    }
}

pub struct PairState(pub State, pub State);

impl fmt::Display for PairState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_state(f, &self.0, Some((0, 0)))?;
        display_state(f, &self.1, Some((48, 0)))?;

        write!(f, "{}", cursor::Goto(0, 26))?;

        fmt::Result::Ok(())
    }
}

pub fn display_state(
    f: &mut fmt::Formatter,
    state: &State,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    write_board(f, &state.board, offset)?;
    write_current_piece(f, state.current_piece, offset)?;
    write_next_pieces(f, &state.next_pieces, offset)?;
    write_hold_piece(f, state.hold_piece, offset)?;

    fmt::Result::Ok(())
}

const PIECE_COLOR: termion::color::Rgb = termion::color::Rgb(0, 154, 208);
const FRAME_COLOR: termion::color::Rgb = termion::color::Rgb(85, 96, 106);

fn write_pos(
    f: &mut fmt::Formatter,
    pos: (i32, i32),
    color: termion::color::Rgb,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    let cursor = match offset {
        Some(offset) => cursor::Goto(pos.0 as u16 + offset.0, pos.1 as u16 + offset.1),
        None => cursor::Goto(pos.0 as u16, pos.1 as u16),
    };

    // write cell
    write!(
        f,
        "{}{}  {}",
        cursor,
        termion::color::Bg(color),
        termion::color::Bg(termion::color::Reset)
    )
}

fn write_field_cell(
    f: &mut fmt::Formatter,
    cell: (i32, i32),
    color: termion::color::Rgb,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    if cell.1 > 23 {
        return fmt::Result::Ok(());
    }

    write_pos(f, (cell.0 * 2 + 14, 24 - cell.1), color, offset)
}

fn write_board(f: &mut fmt::Formatter, board: &Board, offset: Option<(u16, u16)>) -> fmt::Result {
    for i in -1..40 {
        for j in -1..11 {
            if board.occupied(j, i) {
                let color = if !(0..10).contains(&j) || !(0..40).contains(&i) {
                    FRAME_COLOR
                } else {
                    PIECE_COLOR
                };
                write_field_cell(f, (j, i), color, offset)?;
            }
        }
    }

    fmt::Result::Ok(())
}

fn write_piece(
    f: &mut fmt::Formatter,
    piece: Piece,
    pos: (i32, i32),
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    let piece_state = PieceState {
        piece,
        rotation: RotationState::North,
    };

    for (x, y) in piece_state.cells() {
        write_pos(
            f,
            (pos.0 + (1 + x) * 2, pos.1 + (1 - y)),
            PIECE_COLOR,
            offset,
        )?;
    }

    fmt::Result::Ok(())
}

fn write_current_piece(
    f: &mut fmt::Formatter,
    current_piece: Option<Piece>,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    let piece = match current_piece {
        Some(piece) => piece,
        None => return fmt::Result::Ok(()),
    };

    write_piece(f, piece, (20, 1), offset)?;

    fmt::Result::Ok(())
}

fn write_one_of_next_pieces(
    f: &mut fmt::Formatter,
    piece: Piece,
    idx: i32,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    write_piece(f, piece, (38, idx * 4 + 1), offset)?;

    fmt::Result::Ok(())
}

fn write_next_pieces(
    f: &mut fmt::Formatter,
    next_pieces: &VecDeque<Piece>,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    for (idx, piece) in next_pieces.iter().enumerate() {
        if idx >= 5 {
            break;
        }
        write_one_of_next_pieces(f, *piece, idx as i32, offset)?;
    }

    fmt::Result::Ok(())
}

fn write_hold_piece(
    f: &mut fmt::Formatter,
    hold_piece: Option<Piece>,
    offset: Option<(u16, u16)>,
) -> fmt::Result {
    let piece = match hold_piece {
        Some(piece) => piece,
        None => return fmt::Result::Ok(()),
    };

    write_piece(f, piece, (2, 1), offset)?;

    fmt::Result::Ok(())
}
