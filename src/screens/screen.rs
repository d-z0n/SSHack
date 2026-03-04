use ratatui::Frame;
use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};

const BG_HEX: u32 = 0x2D2D2A;
const FG_HEX: u32 = 0x3F5E5A;
const HL_HEX: u32 = 0x20FC8F;

pub const STANDARD_COLOR: Style = Style::new()
    .bg(Color::from_u32(BG_HEX))
    .fg(Color::from_u32(FG_HEX));
pub const HIGHLIGHT_COLOR: Style = Style::new()
    .bg(Color::from_u32(BG_HEX))
    .fg(Color::from_u32(HL_HEX));

pub fn draw_screen_border(f: &mut Frame, title: &'static str, commands: &'static str) -> Rect {
    let area = f.area();
    let [body, command_bar] =
        Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]).areas(area);
    f.render_widget(
        Block::bordered().title_top(title).style(HIGHLIGHT_COLOR),
        body,
    );
    f.render_widget(
        Paragraph::new(Text::styled(commands, STANDARD_COLOR)).block(
            Block::bordered()
                .title_top("COMMANDS")
                .style(HIGHLIGHT_COLOR),
        ),
        command_bar,
    );
    body.inner(Margin::new(1, 1))
}

pub trait Screen {
    fn handle_input(&mut self, key: (KeyCode, KeyModifiers)) -> Option<Box<dyn Screen>>;
    fn render(&mut self, f: &mut Frame);
}

#[derive(Clone, Copy, Default)]
pub enum ScreenType {
    #[default]
    Register,
    Login,
}
