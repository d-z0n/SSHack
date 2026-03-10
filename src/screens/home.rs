use ratatui::{
    crossterm::event::{KeyCode, KeyModifiers},
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Paragraph},
};

use crate::conf::Conf;
use crate::screens::{
    login::LoginScreen,
    register::RegisterScreen,
    screen::{Screen, draw_screen_border},
};

pub struct HomeScreen {
    selected: u8,
    conf: Conf,
}

impl Screen for HomeScreen {
    fn handle_input(
        &mut self,
        key: (KeyCode, KeyModifiers),
    ) -> Option<Box<dyn Screen + Sync + Send>> {
        match key {
            (KeyCode::Enter, _) => return self.submit(),
            (KeyCode::Tab, _) | (KeyCode::Down, _) => self.focus_next(),
            (KeyCode::BackTab, KeyModifiers::SHIFT) | (KeyCode::Up, _) => self.focus_prev(),
            _ => (),
        };
        None
    }
    fn render(&mut self, f: &mut ratatui::Frame) {
        let area = draw_screen_border(
            f,
            vec!["HOME"],
            0,
            "QUIT: <CTRL+Q> - NAVIGATE: <UP|DOWN|TAB> - SELECT: <ENTER>",
            None,
            None,
            &self.conf,
        );

        let [_, banner_part, body, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Max(self.conf.banner.lines().count() as u16 + 1),
            Constraint::Length(6), // six lines for login/register box
            Constraint::Fill(1),
        ])
        .areas(area);

        let [_, banner, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(self.conf.banner.lines().map(|x| x.len()).max().unwrap_or(0) as u16),
            Constraint::Fill(1),
        ])
        .areas(banner_part);
        f.render_widget(
            Paragraph::new(Text::raw(&self.conf.banner))
                .fg(self.conf.theme.base08)
                .bg(self.conf.theme.base00),
            banner,
        );

        let [_, col, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .areas(body);
        let [_, login, register, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(col);

        let color = match self.selected {
            0 => Style::new()
                .fg(self.conf.theme.base08)
                .bg(self.conf.theme.base00),
            _ => Style::new()
                .fg(self.conf.theme.base05)
                .bg(self.conf.theme.base00),
        };

        f.render_widget(
            Paragraph::new("LOGIN")
                .block(Block::bordered())
                .style(color),
            login,
        );

        let color = match self.selected {
            1 => Style::new()
                .fg(self.conf.theme.base08)
                .bg(self.conf.theme.base00),
            _ => Style::new()
                .fg(self.conf.theme.base05)
                .bg(self.conf.theme.base00),
        };
        f.render_widget(
            Paragraph::new("REGISTER")
                .block(Block::bordered())
                .style(color),
            register,
        );
    }
}

impl HomeScreen {
    pub fn new(conf: Conf) -> Self {
        Self { selected: 0, conf }
    }
    fn submit(&mut self) -> Option<Box<dyn Screen + Sync + Send>> {
        match self.selected {
            0 => Some(Box::new(LoginScreen::new(self.conf.clone()))),
            1 => Some(Box::new(RegisterScreen::new(self.conf.clone()))),
            _ => None,
        }
    }

    fn focus_next(&mut self) {
        self.selected += 1;
        self.selected = 1.min(self.selected);
    }

    fn focus_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }
}
