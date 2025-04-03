use crate::{board::Board, piece::Position, Message};
use iced::{
    color,
    widget::{container, mouse_area, row, svg, text, Column, Container, Row, Text},
    window::{icon::from_file, settings::PlatformSpecific, Icon, Level, Settings},
    Alignment, Element, Length, Point, Size, Theme,
};

#[derive(Default)]
pub struct UI {
    board: Board,
    cursor_active: bool,
    cursor_position: Point,
    grabbed_piece_pos: Position,
    previous_moves: Vec<String>,
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
                let position = Position::new(
                    (self.cursor_position.x / 100f32).floor() as u8,
                    (self.cursor_position.y / 100f32).floor() as u8,
                );
                if position.x > 7 || position.y > 7 {
                    return;
                }
                self.grabbed_piece_pos = position;
            }
            Message::LeftButtonReleased => {
                let position = Position::new(
                    (self.cursor_position.x / 100f32).floor() as u8,
                    (self.cursor_position.y / 100f32).floor() as u8,
                );
                if position.x > 7 || position.y > 7 {
                    return;
                }
                let new_position= position;
                self.board.move_piece(self.grabbed_piece_pos, new_position);
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let mut screen = Row::new();
        let mut chess_board = Column::new().width(Length::FillPortion(2));

        for c in 0..4 {
            let mut odd_row = Row::new();
            let mut even_row = Row::new();
            for r in 0..4 {
                let white_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{}{:?}.svg",
                    match self.board[Position::new(r * 2, c * 2)].is_white {
                        true => "White",
                        false => "Black",
                    },
                    self.board[Position::new(r * 2, c * 2)].piece_type
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| container::Style::default().background(color!(0xE3C16F)));
                let black_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{}{:?}.svg",
                    match self.board[Position::new(r * 2 + 1, c * 2)].is_white {
                        true => "White",
                        false => "Black",
                    },
                    self.board[Position::new(r * 2 + 1, c * 2)].piece_type
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| container::Style::default().background(color!(0xB88B4A)));
                odd_row = odd_row.push(white_container);
                odd_row = odd_row.push(black_container);
            }
            chess_board = chess_board.push(odd_row);

            for r in 0..4 {
                let black_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{}{:?}.svg",
                    match self.board[Position::new(r * 2, c * 2 + 1)].is_white {
                        true => "White",
                        false => "Black",
                    },
                    self.board[Position::new(r * 2, c * 2 + 1)].piece_type
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| container::Style::default().background(color!(0xB88B4A)));
                let white_container: Container<'_, Message> = container(svg(format!(
                    "src/pieces/{}{:?}.svg",
                    match self.board[Position::new(r * 2 + 1, c * 2 + 1)].is_white {
                        true => "White",
                        false => "Black",
                    },
                    self.board[Position::new(r * 2 + 1, c * 2 + 1)].piece_type
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| container::Style::default().background(color!(0xE3C16F)));
                even_row = even_row.push(black_container);
                even_row = even_row.push(white_container);
            }
            chess_board = chess_board.push(even_row);
        }
        screen = screen.push(chess_board);

        let mut info_text: Column<'_, Message> = Column::new().width(Length::FillPortion(1));
        let title = text("Chess")
            .size(24)
            .width(Length::Fill)
            .align_x(Alignment::Center);
        let turn: Text = text!("H")
            .size(20)
            .width(Length::Fill)
            .align_x(Alignment::Center);
        let previous_moves = self
            .make_previous_moves_table()
            .width(Length::Fill)
            .align_x(Alignment::Center);
        info_text = info_text.push(title);
        info_text = info_text.push(turn);
        info_text = info_text.push(previous_moves);
        screen = screen.push(info_text);

        mouse_area(screen)
            .on_press(Message::LeftButtonPressed)
            .on_move(Message::CursorMoved)
            .on_release(Message::LeftButtonReleased)
            .into()
    }
    fn make_previous_moves_table(&self) -> Column<'_, Message> {
        let box_width: u16 = 45;
        let box_spacing: u16 = 40;
        let mut previous_moves: Column<'_, Message> = Column::new();
        let header: Row<Message> = row![
            text!("#").width(box_width),
            text!("White").width(box_width),
            text!("Black").width(box_width)
        ]
        .spacing(box_spacing);
        previous_moves = previous_moves.push(header);
        for i in 0..self.previous_moves.len() / 2 {
            let row: Row<Message> = row![
                text!("{}.", i + 1).width(box_width),
                text!("{}", self.previous_moves[i * 2]).width(box_width),
                text!("{}", self.previous_moves[i * 2 + 1]).width(box_width),
            ]
            .spacing(box_spacing);
            previous_moves = previous_moves.push(row);
        }
        if self.previous_moves.len() % 2 == 1 {
            let row: Row<Message> = row![
                text!("{}.", self.previous_moves.len() / 2 + 1).width(box_width),
                text!("{}", self.previous_moves[self.previous_moves.len() - 1]).width(box_width),
                text!("").width(box_width),
            ]
            .spacing(box_spacing);
            previous_moves = previous_moves.push(row);
        }
        previous_moves
    }
    pub fn window() -> Settings {
        let icon: Result<Icon, iced::window::icon::Error> = from_file("src/BlackKnight.png");
        Settings {
            size: Size::new(1200.0, 800.0),
            position: iced::window::Position::Default,
            min_size: Some(Size::new(1200.0, 800.0)),
            max_size: Some(Size::new(1200.0, 800.0)),
            visible: true,
            resizable: false,
            decorations: true,
            transparent: true,
            level: Level::Normal,
            icon: Some(icon.unwrap()),
            platform_specific: PlatformSpecific::default(),
            exit_on_close_request: true,
        }
    }
}
