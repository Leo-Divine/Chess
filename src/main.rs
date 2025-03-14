use std::sync::{Arc, Mutex};

use app::GameBoard;
use iced::Point;
use ui::UI;

mod app;
mod ui;

fn main() {
    let mut board: GameBoard = GameBoard {
        grid: Arc::new(Mutex::new(GameBoard::add_starting_pieces())),
    };
    println!("{:?}", board.get_piece_at(0, 3));
    board.test_piece_move();
    println!("{:?}", board.get_piece_at(0, 3));

    iced::run("Chess", UI::update, UI::view);
}

#[derive(Debug, Clone)]
pub enum Message {
    CursorEntered,
    CursorLeft,
    CursorMoved(Point),
    LeftButtonPressed,
    LeftButtonReleased,
}
