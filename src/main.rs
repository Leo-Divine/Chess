use iced::Point;
use ui::UI;

mod board;
mod piece;
mod ui;

fn main() -> iced::Result {
    iced::application("Chess", UI::update, UI::view)
        .window(UI::window())
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    CursorEntered,
    CursorLeft,
    CursorMoved(Point),
    LeftButtonPressed,
    LeftButtonReleased,
}
