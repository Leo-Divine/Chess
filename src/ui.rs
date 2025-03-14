use crate::Message;
use iced::{
    widget::{container, mouse_area, Column, Container, Row},
    Element, Point,
};

struct Board {}

//Check with clover to see if an easier method exists
impl Board {
    fn create_odd_row() -> Element<'static, Message> {
        let mut row = Row::new();
        for _r in 0..4 {
            let black_container: Container<'_, Message> =
                container("").width(100).height(100).style(container::dark);
            let white_container: Container<'_, Message> = container("").width(100).height(100);
            row = row.push(black_container);
            row = row.push(white_container);
        }

        row.into()
    }

    fn create_even_row() -> Element<'static, Message> {
        let mut row = Row::new();
        for _r in 0..4 {
            let black_container: Container<'_, Message> =
                container("").width(100).height(100).style(container::dark);
            let white_container: Container<'_, Message> = container("").width(100).height(100);
            row = row.push(white_container);
            row = row.push(black_container);
        }

        row.into()
    }
}

#[derive(Default)]
pub struct UI {
    cursor_active: bool,
    cursor_position: Point,
}

impl UI {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::CursorEntered => {
                self.cursor_active = true;
            }
            Message::CursorLeft => {
                self.cursor_active = false;
            }
            Message::CursorMoved(position) => {
                self.cursor_position = position;
            }
            Message::LeftButtonPressed => {
                println!(
                    "({}, {})",
                    (self.cursor_position.x / 100f32).floor(),
                    (self.cursor_position.y / 100f32).floor()
                );
            }
            Message::LeftButtonReleased => {}
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let mut board = Column::new();

        for _c in 0..4 {
            board = board.push(Board::create_odd_row());
            board = board.push(Board::create_even_row());
        }

        mouse_area(board)
            .on_press(Message::LeftButtonPressed)
            .on_move(Message::CursorMoved)
            .into()
    }
}
