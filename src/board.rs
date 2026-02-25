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
        State {
            grid: [Tile::default(); ROWS * COLS],
            time: 0,
            active_piece: Piece::default(),
            bag: [None; 7],
            playing: true,
        }
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
        self.playing = self.check_fall();
        self.active_piece.places.iter_mut().for_each(|index| {
            *index += 4;
            self.grid[*index].has_tile = true
        });
    }
    pub fn tick(&mut self) {
        self.time += 1;
        if self.check_fall() {
            self.active_piece.places.iter_mut().for_each(|index| {
                self.grid[*index].has_tile = false;
                *index += 10;
                self.grid[*index].has_tile = true;
            })
        } else {
            self.add_piece(Piece::default());
        }
    }
    fn check_fall(&self) -> bool {
        self.active_piece.places.iter().all(|index| {
            if let Some(tile) = self.grid.get(index + 10) {
                !tile.has_tile
            } else {
                false
            }
        })
    }
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
impl Piece {}
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
