use iced::Point;
use ui::UI;

mod grid;
mod ui;

fn main() -> iced::Result {
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
