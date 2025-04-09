use iced::Point;
use ui::UI;

mod board;
mod piece;
mod ui;
fn main() -> iced::Result {
    iced::application("Chess", UI::update, UI::view)
        .window(UI::window())
        .subscription(UI::trigger_window_event)
        .run()
}

#[derive(Debug, Clone)]
pub enum Message {
    CursorEntered,
    CursorLeft,
    CursorMoved(Point),
    LeftButtonPressed,
    LeftButtonReleased,
    RestartButtonPressed,
    WindowEventOccurred(iced::event::Event),
}
