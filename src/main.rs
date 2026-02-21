use iced;
use iced::widget::{column, container, row, text, Grid};
use iced::{Element, Fill};
#[derive(Debug, Clone)]
enum Message {}
const ROWS: usize = 20;
const COLS: usize = 10;
#[derive(Debug)]
struct State {
    grid: [[Tile; ROWS]; COLS],
}

impl std::default::Default for State {
    fn default() -> Self {
        State {
            grid: [[Tile::default(); ROWS]; COLS],
        }
    }
}
#[derive(Debug, Default, Copy, Clone)]
struct Tile {
    has_tile: bool,
}
impl Tile {
    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}
fn update(state: &mut State, message: Message) {
    match message {}
}

fn view(state: &State) -> Element<'_, Message> {
    let total_elements = ROWS * COLS;
    container(
        Grid::from_iter(state.grid.iter().map(|i| i.view()))
            .columns(10)
            .height(Fill),
    )
    .max_width(1000)
    .max_height(2000)
    .into()
}
