use crate::{
    board::Board,
    piece::{Color, Position},
    Message,
};
use iced::{
    advanced::graphics::image::image_rs::ImageFormat,
    color,
    event::{self, Event},
    widget::{
        column, container, mouse_area, row, scrollable,
        scrollable::{Direction, Scrollbar},
        svg, text, Button, Column, Container, MouseArea, Row, Space, Text,
    },
    window::{self, icon::from_file_data, settings::PlatformSpecific, Icon, Level, Settings},
    Alignment, Element, Length, Point, Size, Subscription, Theme,
};
use images::get_image;

#[derive(Default)]
pub struct UI {
    window_size: Size,
    cursor_active: bool,
    cursor_position: Point,
    board: Board,
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
                let box_width = self.window_size.width / 12f32;
                let box_height = self.window_size.height / 8f32;
                let position = Position::new(
                    (self.cursor_position.x / box_width).floor() as u8,
                    (self.cursor_position.y / box_height).floor() as u8,
                );
                if position.x > 7 || position.y > 7 {
                    return;
                }
                self.grabbed_piece_pos = position;
            }
            Message::LeftButtonReleased => {
                let box_width = self.window_size.width / 12f32;
                let box_height = self.window_size.height / 8f32;
                let position = Position::new(
                    (self.cursor_position.x / box_width).floor() as u8,
                    (self.cursor_position.y / box_height).floor() as u8,
                );
                if position.x > 7 || position.y > 7 {
                    return;
                }
                let new_position = position;
                match self.board.move_piece(self.grabbed_piece_pos, new_position) {
                    Some(notation) => self.previous_moves.push(notation),
                    None => println!("Invalid Move"),
                }
            }
            Message::RestartButtonPressed => {
                self.board = Board::default();
                self.previous_moves.clear();
            }
            Message::WindowEventOccurred(event) => {
                if let Event::Window(window::Event::Resized(size)) = event {
                    self.window_size = size;
                }
            }
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let mut screen = Row::new();
        screen = screen.push(self.make_chess_board());
        let mut info_text: Column<'_, Message> = Column::new().width(Length::FillPortion(1));

        let title = text("Chess")
            .size(24)
            .width(Length::Fill)
            .align_x(Alignment::Center);

        let turn: Text = text!(
            "{}",
            if self.board.turn == Color::White {
                "It's White's Turn"
            } else {
                "It's Black's Turn"
            }
        )
        .size(20)
        .width(Length::Fill)
        .align_x(Alignment::Center);

        let header: Column<Message> = column![row![
            text!("#").width(50),
            text!("White").width(50),
            text!("Black").width(50)
        ]
        .spacing(40)]
        .width(Length::Fill)
        .align_x(Alignment::Center);

        let previous_moves = scrollable(
            self.make_previous_moves_table()
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .anchor_bottom()
        .direction(Direction::Vertical(Scrollbar::new()))
        .height(Length::FillPortion(2));

        let restart_button: Button<Message> = Button::new("Restart Game")
            .width(Length::Fill)
            .on_press(Message::RestartButtonPressed);

        info_text = info_text.push(title);
        info_text = info_text.push(turn);
        info_text = info_text.push(header);
        info_text = info_text.push(previous_moves);
        info_text = info_text.push(Space::with_height(Length::FillPortion(1)));
        info_text = info_text.push(restart_button);
        screen = screen.push(info_text);
        screen.into()
    }
    fn make_chess_board(&self) -> MouseArea<'_, Message> {
        let mut chess_board = Column::new().width(Length::FillPortion(2));

        for c in 0..4 {
            let mut odd_row = Row::new();
            let mut even_row = Row::new();
            for r in 0..4 {
                let white_container: Container<'_, Message> = container(svg(get_image(
                    self.board[Position::new(r * 2, c * 2)].color,
                    self.board[Position::new(r * 2, c * 2)].piece_type,
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| {
                    container::Style::default().background(color!(0x00E3_C16F))
                });

                let black_container: Container<'_, Message> = container(svg(get_image(
                    self.board[Position::new(r * 2 + 1, c * 2)].color,
                    self.board[Position::new(r * 2 + 1, c * 2)].piece_type,
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| {
                    container::Style::default().background(color!(0x00B8_8B4A))
                });

                odd_row = odd_row.push(white_container);
                odd_row = odd_row.push(black_container);
            }
            chess_board = chess_board.push(odd_row);

            for r in 0..4 {
                let black_container: Container<'_, Message> = container(svg(get_image(
                    self.board[Position::new(r * 2, c * 2 + 1)].color,
                    self.board[Position::new(r * 2, c * 2 + 1)].piece_type,
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| {
                    container::Style::default().background(color!(0x00B8_8B4A))
                });

                let white_container: Container<'_, Message> = container(svg(get_image(
                    self.board[Position::new(r * 2 + 1, c * 2 + 1)].color,
                    self.board[Position::new(r * 2 + 1, c * 2 + 1)].piece_type,
                )))
                .width(Length::FillPortion(1))
                .height(Length::FillPortion(1))
                .style(|_theme: &Theme| {
                    container::Style::default().background(color!(0x00E3_C16F))
                });

                even_row = even_row.push(black_container);
                even_row = even_row.push(white_container);
            }
            chess_board = chess_board.push(even_row);
        }

        mouse_area(chess_board)
            .on_press(Message::LeftButtonPressed)
            .on_move(Message::CursorMoved)
            .on_release(Message::LeftButtonReleased)
    }
    fn make_previous_moves_table(&self) -> Column<'_, Message> {
        const BOX_WIDTH: u16 = 50;
        const BOX_SPACING: u16 = 40;

        let mut previous_moves: Column<'_, Message> = Column::new();
        for i in 0..self.previous_moves.len() / 2 {
            let row: Row<Message> = row![
                text!("{}.", i + 1).width(BOX_WIDTH),
                text!("{}", self.previous_moves[i * 2]).width(BOX_WIDTH),
                text!("{}", self.previous_moves[i * 2 + 1]).width(BOX_WIDTH),
            ]
            .spacing(BOX_SPACING);
            previous_moves = previous_moves.push(row);
        }
        if self.previous_moves.len() % 2 == 1 {
            let row: Row<Message> = row![
                text!("{}.", self.previous_moves.len() / 2 + 1).width(BOX_WIDTH),
                text!("{}", self.previous_moves[self.previous_moves.len() - 1]).width(BOX_WIDTH),
                text!("").width(BOX_WIDTH),
            ]
            .spacing(BOX_SPACING);
            previous_moves = previous_moves.push(row);
        }
        previous_moves
    }
    pub fn window() -> Settings {
        let icon: Result<Icon, iced::window::icon::Error> =
            from_file_data(include_bytes!("BlackKnight.png"), Some(ImageFormat::Png));
        Settings {
            size: Size::new(1200.0, 800.0),
            position: iced::window::Position::Default,
            min_size: Some(Size::new(800.0, 533.0)),
            max_size: Some(Size::new(1800.0, 1200.0)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: true,
            level: Level::Normal,
            icon: Some(icon.unwrap()),
            platform_specific: PlatformSpecific::default(),
            exit_on_close_request: true,
        }
    }
    pub fn trigger_window_event(&self) -> Subscription<Message> {
        event::listen().map(Message::WindowEventOccurred)
    }
}

mod images {
    use iced::widget::svg::Handle;

    use crate::piece::{Color, PieceType};

    // White's pieces
    const WHITE_PAWN: &[u8] = include_bytes!("pieces/WhitePawn.svg");
    const WHITE_KNIGHT: &[u8] = include_bytes!("pieces/WhiteKnight.svg");
    const WHITE_BISHOP: &[u8] = include_bytes!("pieces/WhiteBishop.svg");
    const WHITE_ROOK: &[u8] = include_bytes!("pieces/WhiteRook.svg");
    const WHITE_QUEEN: &[u8] = include_bytes!("pieces/WhiteQueen.svg");
    const WHITE_KING: &[u8] = include_bytes!("pieces/WhiteKing.svg");

    // Black's pieces
    const BLACK_PAWN: &[u8] = include_bytes!("pieces/BlackPawn.svg");
    const BLACK_KNIGHT: &[u8] = include_bytes!("pieces/BlackKnight.svg");
    const BLACK_BISHOP: &[u8] = include_bytes!("pieces/BlackBishop.svg");
    const BLACK_ROOK: &[u8] = include_bytes!("pieces/BlackRook.svg");
    const BLACK_QUEEN: &[u8] = include_bytes!("pieces/BlackQueen.svg");
    const BLACK_KING: &[u8] = include_bytes!("pieces/BlackKing.svg");

    pub fn get_image(color: Color, piece_type: PieceType) -> Handle {
        Handle::from_memory(match color {
            Color::White => match piece_type {
                PieceType::Pawn => WHITE_PAWN,
                PieceType::Knight => WHITE_KNIGHT,
                PieceType::Bishop => WHITE_BISHOP,
                PieceType::Rook => WHITE_ROOK,
                PieceType::Queen => WHITE_QUEEN,
                PieceType::King => WHITE_KING,
                PieceType::None => &[],
            },
            Color::Black => match piece_type {
                PieceType::Pawn => BLACK_PAWN,
                PieceType::Knight => BLACK_KNIGHT,
                PieceType::Bishop => BLACK_BISHOP,
                PieceType::Rook => BLACK_ROOK,
                PieceType::Queen => BLACK_QUEEN,
                PieceType::King => BLACK_KING,
                PieceType::None => &[],
            },
        })
    }
}
