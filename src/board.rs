use super::Message;
use iced::widget::container;
use iced::widget::Grid;
use iced::Border;
use iced::Element;
use iced::Theme;
pub const ROWS: usize = 20;
pub const COLS: usize = 10;
#[derive(Debug)]
pub struct State {
    grid: [Tile; ROWS * COLS],
    time: u32,
    active_piece: Piece,
    bag: [Option<Tetrominoe>; 7],
    playing: bool,
}

impl std::default::Default for State {
    fn default() -> Self {
        let mut new_state = State {
            grid: [Tile::default(); ROWS * COLS],
            time: 0,
            active_piece: Piece::default(),
            bag: [None; 7],
            playing: true,
        };
        new_state.tick();
        new_state.tick();
        new_state.tick();
        new_state.tick();
        new_state.tick();
        new_state
    }
}
impl State {
    pub fn view(&self) -> Element<'_, super::Message> {
        Grid::from_iter(self.grid.iter().map(|i| i.view()))
            .columns(COLS)
            .into()
    }
    fn add_piece(&mut self, piece: Piece) {
        if !self.playing {
            return;
        }
        self.active_piece = piece;
        self.playing = self.check_move(COLS.try_into().unwrap(), true) == MoveResult::Moved;
        println!("adding new piece playing = {}", self.playing);
        self.active_piece.places.iter_mut().for_each(|index| {
            *index += 4;
            self.grid[*index].has_tile = true
        });
    }
    pub fn tick(&mut self) {
        self.time += 1;
        if self.time % 5 == 0 {
            if self.move_pieces(10, true) == MoveResult::HitBottom {
                self.add_piece(Piece::default());
            }
        }
    }
    pub fn take_action(&mut self, action: super::Action) {
        // println!("{:?}", self);
        match action {
            super::Action::Left => self.move_pieces(-1, false),
            super::Action::Right => self.move_pieces(1, false),
            super::Action::FinishFall => MoveResult::HitBottom,
            super::Action::CounterClockWise => self.spin_pieces(false),
            super::Action::ClockWise => self.spin_pieces(true),
        };
    }
    fn check_move(&self, amount: isize, can_wrap: bool) -> MoveResult {
        for index in self.active_piece.places.iter() {
            let new_index = amount.checked_add_unsigned(*index).unwrap();
            if new_index.is_negative() {
                // println!("moving before first square");
                return MoveResult::OutOfBounds;
            }
            let new_index = new_index.unsigned_abs();
            if new_index >= (ROWS * COLS) {
                // println!("moving off edge of board");
                return MoveResult::HitBottom;
            }
            if amount < 0 && !can_wrap {
                if (new_index + 1).is_multiple_of(COLS) {
                    // println!("trying to wrap when can_wrap = {}", can_wrap);
                    return MoveResult::NoWrapAllowed;
                }
            }
            if amount > 0 && !can_wrap {
                if new_index.is_multiple_of(COLS) {
                    // println!("trying to wrap when can_wrap = {}", can_wrap);
                    return MoveResult::NoWrapAllowed;
                }
            }
            if !self.active_piece.places.contains(&new_index) && self.grid[new_index].has_tile {
                return MoveResult::HitBottom;
            }
        }
        return MoveResult::Moved;
    }
    fn move_pieces(&mut self, amount: isize, can_wrap: bool) -> MoveResult {
        let check_move_result = self.check_move(amount, can_wrap);
        if check_move_result != MoveResult::Moved {
            return check_move_result;
        }
        self.active_piece
            .places
            .iter()
            .for_each(|index| self.grid[*index].has_tile = false);
        self.active_piece
            .places
            .iter_mut()
            .for_each(|index| *index = (*index).checked_add_signed(amount).unwrap());
        self.active_piece
            .places
            .iter()
            .for_each(|index| self.grid[*index].has_tile = true);
        return check_move_result;
    }
    fn spin_pieces(&mut self, clockwise: bool) -> MoveResult {
        //TODO
        todo!();
    }
}
#[derive(PartialEq, Debug)]
enum MoveResult {
    NoWrapAllowed,
    HitBottom,
    Moved,
    OutOfBounds,
}
#[derive(Debug, Copy, Clone)]
struct Piece {
    places: [usize; 4],
}
impl std::default::Default for Piece {
    fn default() -> Self {
        Piece {
            places: Tetrominoe::I.out(),
        }
    }
}
#[derive(Debug, Copy, Clone)]
enum Tetrominoe {
    J,
    L,
    S,
    Z,
    I,
    O,
    T,
}
impl Tetrominoe {
    pub fn out(&self) -> [usize; 4] {
        match self {
            Tetrominoe::I => [0, 1, 2, 3],
            _ => todo!(),
        }
    }
}
#[derive(Debug, Default, Copy, Clone)]
struct Tile {
    has_tile: bool,
}
impl Tile {
    pub fn view(&self) -> Element<'_, Message> {
        container("")
            .style(|theme: &Theme| {
                if self.has_tile {
                    container::Style::default().background(theme.palette().primary)
                } else {
                    container::Style::default().background(theme.palette().background)
                }
                .border(Border::default().color(theme.palette().text).width(1))
            })
            .into()
    }
}
