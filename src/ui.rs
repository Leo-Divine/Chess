use crate::{grid::Board, Message};
use iced::{
    widget::{container, mouse_area, svg, Column, Container, Row},
    Element, Point,
};

#[derive(Default)]
pub struct UI {
    board: Board,
    cursor_active: bool,
    cursor_position: Point,
    grabbed_piece_position: (usize, usize),
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
                self.grabbed_piece_position = (
                    (self.cursor_position.x / 100f32).floor() as usize,
                    (self.cursor_position.y / 100f32).floor() as usize,
                );
            }
            Message::LeftButtonReleased => {
                let mut handle = self.board.lock().unwrap();
                let new_position: (usize, usize) = (
                    (self.cursor_position.x / 100f32).floor() as usize,
                    (self.cursor_position.y / 100f32).floor() as usize,
                );

                handle.move_piece(self.grabbed_piece_position, new_position);
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let handle = self.board.lock().unwrap();
        let mut board = Column::new();

        for c in 0..4 {
            let mut odd_row = Row::new();
            let mut even_row = Row::new();
            for r in 0..4 {
                let black_container: Container<'_, Message> =
                    container(svg(format!("src/pieces/{:?}.svg", handle[(r * 2, c * 2)])))
                        .width(100)
                        .height(100)
                        .style(container::dark);
                let white_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2 + 1, c * 2)]
                )))
                .width(100)
                .height(100);
                odd_row = odd_row.push(black_container);
                odd_row = odd_row.push(white_container);
            }
            board = board.push(odd_row);

            for r in 0..4 {
                let white_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2, c * 2 + 1)]
                )))
                .width(100)
                .height(100);
                let black_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2 + 1, c * 2 + 1)]
                )))
                .width(100)
                .height(100)
                .style(container::dark);
                even_row = even_row.push(white_container);
                even_row = even_row.push(black_container);
            }
            board = board.push(even_row);
        }

        mouse_area(board)
            .on_press(Message::LeftButtonPressed)
            .on_move(Message::CursorMoved)
            .on_release(Message::LeftButtonReleased)
            .into()
    }
    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dracula
    }
}
