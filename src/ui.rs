use crate::{
    grid::{Board, Piece, Position},
    Message,
};
use iced::{
    color,
    widget::{container, mouse_area, svg, Column, Container, Row},
    Element, Point, Theme,
};

#[derive(Default)]
pub struct UI {
    board: Board,
    cursor_active: bool,
    cursor_position: Point,
    grabbed_piece: Piece,
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
                let handle = self.board.lock().unwrap();
                self.grabbed_piece = handle[(
                    (self.cursor_position.x / 100f32).floor() as usize,
                    (self.cursor_position.y / 100f32).floor() as usize,
                )]
                    .clone();
            }
            Message::LeftButtonReleased => {
                let mut handle = self.board.lock().unwrap();
                let new_position: Position = Position::new(
                    (self.cursor_position.x / 100f32).floor() as usize,
                    (self.cursor_position.y / 100f32).floor() as usize,
                );

                handle.move_piece(self.grabbed_piece.clone(), new_position);
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
                let white_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2, c * 2)].piece_type
                )))
                .width(100)
                .height(100)
                .style(|_theme: &Theme| container::Style::default().background(color!(0xE3C16F)));
                let black_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2 + 1, c * 2)].piece_type
                )))
                .width(100)
                .height(100)
                .style(|_theme: &Theme| container::Style::default().background(color!(0xB88B4A)));
                odd_row = odd_row.push(white_container);
                odd_row = odd_row.push(black_container);
            }
            board = board.push(odd_row);

            for r in 0..4 {
                let black_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2, c * 2 + 1)].piece_type
                )))
                .width(100)
                .height(100)
                .style(|_theme: &Theme| container::Style::default().background(color!(0xB88B4A)));
                let white_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{:?}.svg",
                    handle[(r * 2 + 1, c * 2 + 1)].piece_type
                )))
                .width(100)
                .height(100)
                .style(|_theme: &Theme| container::Style::default().background(color!(0xE3C16F)));
                even_row = even_row.push(black_container);
                even_row = even_row.push(white_container);
            }
            board = board.push(even_row);
        }

        mouse_area(board)
            .on_press(Message::LeftButtonPressed)
            .on_move(Message::CursorMoved)
            .on_release(Message::LeftButtonReleased)
            .into()
    }
}
