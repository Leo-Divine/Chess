use std::sync::{Arc, Mutex};

use grid::{Board, Grid};
use iced::Point;
use ui::UI;

mod grid;
mod ui;

fn main() -> iced::Result {
    let grid: Board = Arc::new(Mutex::new(Grid::new()));
    println!("{:?}", grid.lock().unwrap()[(0, 0)]);
    iced::application("Chess", UI::update, UI::view).run()
}

#[derive(Debug, Clone)]
pub enum Message {
    CursorEntered,
    CursorLeft,
    CursorMoved(Point),
    LeftButtonPressed,
    LeftButtonReleased,
}
