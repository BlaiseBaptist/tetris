use super::Message;
use iced::widget::canvas;
use iced::widget::{column, text, Canvas};
use iced::{Color, Element, Fill, Point, Rectangle, Size};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub const ROWS: usize = 20;
pub const COLS: usize = 10;

#[derive(Debug)]
pub struct State {
    grid: [Tile; ROWS * COLS],
    time: u32,
    active_piece: Piece,
    bag: Vec<Tetrominoe>,
    playing: bool,
    score: u32,
}

impl Default for State {
    fn default() -> Self {
        let mut new_state = State {
            grid: [Tile::default(); ROWS * COLS],
            time: 0,
            active_piece: Piece::default(),
            bag: Vec::new(),
            playing: true,
            score: 0,
        };
        new_state.spawn_piece();
        new_state
    }
}

impl State {
    pub fn view(&self) -> Element<'_, Message> {
        let canvas_widget = Canvas::new(BoardCanvas(self))
            .width(Fill)
            .height(Fill);

        if !self.playing {
            column![
                text("GAME OVER").size(30),
                text(format!("Score: {}", self.score)).size(20),
                canvas_widget,
            ]
            .height(Fill)
            .into()
        } else {
            column![
                text(format!("Score: {}", self.score)).size(20),
                canvas_widget,
            ]
            .height(Fill)
            .into()
        }
    }

    fn next_from_bag(&mut self) -> Tetrominoe {
        if self.bag.is_empty() {
            let mut pieces = [
                Tetrominoe::J,
                Tetrominoe::L,
                Tetrominoe::S,
                Tetrominoe::Z,
                Tetrominoe::I,
                Tetrominoe::O,
                Tetrominoe::T,
            ];
            pieces.shuffle(&mut thread_rng());
            self.bag = pieces.to_vec();
        }
        self.bag.pop().unwrap()
    }

    fn spawn_piece(&mut self) {
        if !self.playing {
            return;
        }
        let piece_type = self.next_from_bag();
        let new_piece = Piece::new(piece_type);
        match new_piece.try_places() {
            Err(_) => {
                self.playing = false;
            }
            Ok(places) => {
                if places.iter().any(|&idx| self.grid[idx].piece_type.is_some()) {
                    self.playing = false;
                    return;
                }
                self.active_piece = new_piece;
                for &idx in places.iter() {
                    self.grid[idx].piece_type = Some(piece_type);
                }
            }
        }
    }

    pub fn tick(&mut self) {
        if !self.playing {
            return;
        }
        self.time += 1;
        // Pieces fall once every 50 ticks (500 ms at 10 ms/tick)
        if self.time.is_multiple_of(50)
            && self.move_active(1, 0) != MoveResult::Moved {
                self.clear_lines();
                self.spawn_piece();
            }
    }

    pub fn take_action(&mut self, action: super::Action) {
        if !self.playing {
            return;
        }
        match action {
            super::Action::Left => {
                self.move_active(0, -1);
            }
            super::Action::Right => {
                self.move_active(0, 1);
            }
            super::Action::SoftDrop => {
                if self.move_active(1, 0) != MoveResult::Moved {
                    self.clear_lines();
                    self.spawn_piece();
                }
            }
            super::Action::FinishFall => {
                while self.move_active(1, 0) == MoveResult::Moved {}
                self.clear_lines();
                self.spawn_piece();
            }
            super::Action::ClockWise => {
                self.spin(true);
            }
            super::Action::CounterClockWise => {
                self.spin(false);
            }
        }
    }

    fn move_active(&mut self, dr: isize, dc: isize) -> MoveResult {
        let mut new_piece = self.active_piece;
        new_piece.row += dr;
        new_piece.col += dc;

        let new_places = match new_piece.try_places() {
            Err(_) => return MoveResult::Blocked,
            Ok(p) => p,
        };

        let old_places = self.active_piece.places();

        for &idx in new_places.iter() {
            if !old_places.contains(&idx) && self.grid[idx].piece_type.is_some() {
                return MoveResult::Blocked;
            }
        }

        for &idx in old_places.iter() {
            self.grid[idx].piece_type = None;
        }
        self.active_piece = new_piece;
        for &idx in new_places.iter() {
            self.grid[idx].piece_type = Some(self.active_piece.piece_type);
        }
        MoveResult::Moved
    }

    fn spin(&mut self, clockwise: bool) {
        if matches!(self.active_piece.piece_type, Tetrominoe::O) {
            return;
        }

        let new_rotation = if clockwise {
            (self.active_piece.rotation + 1) % 4
        } else {
            (self.active_piece.rotation + 3) % 4
        };

        let old_places = self.active_piece.places();
        let kicks: &[(isize, isize)] = &[(0, 0), (0, -1), (0, 1), (0, -2), (0, 2), (-1, 0)];

        for &(kr, kc) in kicks {
            let candidate = Piece {
                rotation: new_rotation,
                row: self.active_piece.row + kr,
                col: self.active_piece.col + kc,
                ..self.active_piece
            };

            if let Ok(new_places) = candidate.try_places() {
                let valid = new_places
                    .iter()
                    .all(|&idx| old_places.contains(&idx) || self.grid[idx].piece_type.is_none());

                if valid {
                    for &idx in old_places.iter() {
                        self.grid[idx].piece_type = None;
                    }
                    self.active_piece = candidate;
                    for &idx in new_places.iter() {
                        self.grid[idx].piece_type = Some(self.active_piece.piece_type);
                    }
                    return;
                }
            }
        }
    }

    fn clear_lines(&mut self) {
        let mut rows_cleared = 0u32;
        let mut row = ROWS - 1;
        loop {
            let start = row * COLS;
            if self.grid[start..start + COLS]
                .iter()
                .all(|t| t.piece_type.is_some())
            {
                for r in (1..=row).rev() {
                    for c in 0..COLS {
                        self.grid[r * COLS + c] = self.grid[(r - 1) * COLS + c];
                    }
                }
                for c in 0..COLS {
                    self.grid[c] = Tile::default();
                }
                rows_cleared += 1;
            } else {
                if row == 0 {
                    break;
                }
                row -= 1;
            }
        }
        self.score += match rows_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => 0,
        };
    }
}

// ---------------------------------------------------------------------------
// Canvas rendering
// ---------------------------------------------------------------------------

struct BoardCanvas<'a>(&'a State);

impl<'a> canvas::Program<Message> for BoardCanvas<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let state = self.0;
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Compute the largest square tile that fits the canvas
        let tile = (bounds.width / COLS as f32).min(bounds.height / ROWS as f32);
        let board_w = tile * COLS as f32;
        let board_h = tile * ROWS as f32;
        // Centre the board in the canvas
        let ox = (bounds.width - board_w) / 2.0;
        let oy = (bounds.height - board_h) / 2.0;

        // Board background
        frame.fill_rectangle(
            Point::new(ox, oy),
            Size::new(board_w, board_h),
            Color::from_rgb(0.08, 0.08, 0.08),
        );

        let gap = (tile * 0.04).max(1.0);

        for row in 0..ROWS {
            for col in 0..COLS {
                let cell = &state.grid[row * COLS + col];
                let x = ox + col as f32 * tile + gap;
                let y = oy + row as f32 * tile + gap;
                let cell_size = tile - 2.0 * gap;

                let color = match cell.piece_type {
                    Some(t) => t.color(),
                    None => Color::from_rgb(0.14, 0.14, 0.14),
                };

                frame.fill_rectangle(Point::new(x, y), Size::new(cell_size, cell_size), color);
            }
        }

        vec![frame.into_geometry()]
    }
}

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

#[derive(PartialEq, Debug)]
enum MoveResult {
    Moved,
    Blocked,
}

#[derive(Debug, Copy, Clone)]
struct Piece {
    piece_type: Tetrominoe,
    rotation: u8,
    row: isize,
    col: isize,
}

impl Default for Piece {
    fn default() -> Self {
        Piece {
            piece_type: Tetrominoe::I,
            rotation: 0,
            row: 1,
            col: 4,
        }
    }
}

impl Piece {
    fn new(piece_type: Tetrominoe) -> Self {
        Piece {
            piece_type,
            rotation: 0,
            row: 1,
            col: 4,
        }
    }

    fn offsets(&self) -> [(isize, isize); 4] {
        apply_rotation(self.piece_type.base_offsets(), self.rotation)
    }

    fn try_places(&self) -> Result<[usize; 4], ()> {
        let offsets = self.offsets();
        let mut places = [0usize; 4];
        for (i, (dr, dc)) in offsets.iter().enumerate() {
            let r = self.row + dr;
            let c = self.col + dc;
            if r < 0 || r >= ROWS as isize || c < 0 || c >= COLS as isize {
                return Err(());
            }
            places[i] = r as usize * COLS + c as usize;
        }
        Ok(places)
    }

    fn places(&self) -> [usize; 4] {
        self.try_places().unwrap()
    }
}

/// Clockwise rotation transform: (r, c) → (c, −r), applied `rotation` times.
fn apply_rotation(offsets: [(isize, isize); 4], rotation: u8) -> [(isize, isize); 4] {
    offsets.map(|(r, c)| match rotation % 4 {
        0 => (r, c),
        1 => (c, -r),
        2 => (-r, -c),
        3 => (-c, r),
        _ => unreachable!(),
    })
}

#[derive(Debug, Copy, Clone, PartialEq)]
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
    fn base_offsets(self) -> [(isize, isize); 4] {
        match self {
            Tetrominoe::I => [(0, -1), (0, 0), (0, 1), (0, 2)],
            Tetrominoe::O => [(0, 0), (0, 1), (1, 0), (1, 1)],
            Tetrominoe::T => [(-1, 0), (0, -1), (0, 0), (0, 1)],
            Tetrominoe::S => [(-1, 0), (-1, 1), (0, -1), (0, 0)],
            Tetrominoe::Z => [(-1, -1), (-1, 0), (0, 0), (0, 1)],
            Tetrominoe::J => [(-1, -1), (0, -1), (0, 0), (0, 1)],
            Tetrominoe::L => [(-1, 1), (0, -1), (0, 0), (0, 1)],
        }
    }

    fn color(self) -> Color {
        match self {
            Tetrominoe::I => Color::from_rgb(0.0, 0.9, 0.9),
            Tetrominoe::O => Color::from_rgb(0.9, 0.9, 0.0),
            Tetrominoe::T => Color::from_rgb(0.6, 0.0, 0.8),
            Tetrominoe::S => Color::from_rgb(0.0, 0.8, 0.0),
            Tetrominoe::Z => Color::from_rgb(0.9, 0.0, 0.0),
            Tetrominoe::J => Color::from_rgb(0.0, 0.3, 0.9),
            Tetrominoe::L => Color::from_rgb(0.9, 0.5, 0.0),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Tile {
    piece_type: Option<Tetrominoe>,
}
