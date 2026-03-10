use std::error::Error;

use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::{
    conf::Conf,
    database::{Flag, User},
    screens::{
        home::HomeScreen,
        leaderboard::LeaderboardScreen,
        screen::{Screen, draw_screen_border},
    },
};

enum BrowseScreenState {
    Browse,
    Submit,
}

pub struct BrowseScreen {
    state: BrowseScreenState,
    user: User,
    flags: Vec<Flag>,
    error: Option<String>,
    table_state: TableState,
    scroll: u16,
    submission: String,
    conf: Conf,
}

impl Screen for BrowseScreen {
    fn handle_input(
        &mut self,
        key: (KeyCode, KeyModifiers),
    ) -> Option<Box<dyn Screen + Sync + Send>> {
        self.error = None;
        match key {
            (KeyCode::Enter, _) => return self.submit(),
            (KeyCode::Esc, _) => return self.escape(),
            (KeyCode::Tab, _) | (KeyCode::Down, _) => self.focus_next(),
            (KeyCode::BackTab, KeyModifiers::SHIFT) | (KeyCode::Up, _) => self.focus_prev(),
            (KeyCode::Backspace, _) => self.erase(),
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => return self.reload(),
            (KeyCode::Right, KeyModifiers::CONTROL) => {
                return Some(Box::new(LeaderboardScreen::new(
                    Some(self.user.clone()),
                    self.conf.clone(),
                )));
            }
            (KeyCode::Char(c), _) => self.write_char(c),
            _ => (),
        };
        None
    }
    fn render(&mut self, f: &mut Frame) {
        let commands = match self.state {
            BrowseScreenState::Browse => {
                "QUIT<CTRL+Q> LOG OUT<ESC> NAV<UP|DOWN|TAB> SELECT<ENTER> RLOAD<CTRL+R> NAV TABS<CTRL+LEFT|RIGHT>"
            }
            BrowseScreenState::Submit => {
                "QUIT<CTRL+Q> BROWSE<ESC> SCROLL<UP|DOWN> SUBMIT FLAG<ENTER> RLOAD<CTRL+R> NAV TABS<CTRL+LEFT|RIGHT>"
            }
        };
        let area = draw_screen_border(
            f,
            vec!["FLAGS", "LEADERBOARD"],
            0,
            commands,
            self.error.as_deref(),
            Some(&self.user),
            &self.conf,
        );
        let [col1, col2] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);

        self.draw_table(f, col1);
        if let Err(e) = self.draw_preview(f, col2) {
            self.error = Some(e.to_string());
        };
    }
}

impl BrowseScreen {
    fn submit(&mut self) -> Option<Box<dyn Screen + Sync + Send>> {
        match self.state {
            BrowseScreenState::Browse => {
                self.state = BrowseScreenState::Submit;
            }
            BrowseScreenState::Submit => {
                let Some(index) = self.table_state.selected() else {
                    self.error = Some("no flag selected to submit".to_string());
                    return None;
                };
                if let Some(flag) = self.flags.get(index) {
                    if flag.flag() != self.submission {
                        self.error = Some("incorrect flag submitted".to_string());
                        return None;
                    }
                    if let Err(e) = flag.mark_solved_for_user(self.user.id()) {
                        self.error = Some(e.to_string());
                        return None;
                    };
                    self.submission.clear();
                    self.state = BrowseScreenState::Browse;
                    self.reload();
                }
            }
        }
        None
    }

    pub fn new(user: User, conf: Conf) -> Self {
        let (flags, error) = match Flag::get_all_with_user(&user) {
            Ok(flags) => (flags, None),
            Err(e) => (vec![], Some(e.to_string())),
        };
        Self {
            state: BrowseScreenState::Browse,
            table_state: TableState::default().with_selected(0),
            user,
            flags,
            error,
            scroll: 0,
            submission: String::new(),
            conf,
        }
    }

    fn focus_next(&mut self) {
        match self.state {
            BrowseScreenState::Browse => self.table_state.select_next(),
            BrowseScreenState::Submit => self.scroll = self.scroll.saturating_add(1),
        }
    }

    fn focus_prev(&mut self) {
        match self.state {
            BrowseScreenState::Browse => self.table_state.select_previous(),
            BrowseScreenState::Submit => self.scroll = self.scroll.saturating_sub(1),
        }
    }

    fn escape(&mut self) -> Option<Box<dyn Screen + Sync + Send>> {
        match self.state {
            BrowseScreenState::Browse => Some(Box::new(HomeScreen::new(self.conf.clone()))),
            BrowseScreenState::Submit => {
                self.scroll = 0;
                self.submission.clear();
                self.state = BrowseScreenState::Browse;
                return None;
            }
        }
    }

    fn erase(&mut self) {
        match self.state {
            BrowseScreenState::Browse => (),
            BrowseScreenState::Submit => {
                let _ = self.submission.pop();
            }
        }
    }

    fn write_char(&mut self, c: char) {
        match self.state {
            BrowseScreenState::Browse => (),
            BrowseScreenState::Submit => self.submission.push(c),
        }
    }

    fn draw_table(&mut self, f: &mut Frame, a: Rect) {
        let header = ["Name", "Description", "Points", "Solved"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .fg(self.conf.theme.base08)
            .bg(self.conf.theme.base00)
            .italic()
            .bold()
            .height(1);

        let rows = self.flags.iter().enumerate().map(|(i, f)| {
            f.row_parts()
                .into_iter()
                .map(Cell::from)
                .collect::<Row>()
                .fg(self.conf.theme.base05)
                .bg(if i % 2 == 1 {
                    self.conf.theme.base00
                } else {
                    self.conf.theme.base01
                })
                .height(1)
        });

        let table = Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Fill(2),
                Constraint::Fill(1),
                // seven for 'Solved' +1
                Constraint::Length(7),
            ],
        )
        .header(header)
        .row_highlight_style(
            Style::new()
                .fg(self.conf.theme.base05)
                .bg(self.conf.theme.base03),
        )
        .highlight_symbol(" >")
        .block(
            Block::new().borders(Borders::RIGHT).border_style(
                Style::new()
                    .fg(self.conf.theme.base01)
                    .bg(self.conf.theme.base00),
            ),
        );

        f.render_stateful_widget(table, a, &mut self.table_state);
    }

    fn draw_preview(&self, f: &mut Frame<'_>, a: Rect) -> Result<(), Box<dyn Error>> {
        let flag = self
            .flags
            .get(
                self.table_state
                    .selected()
                    .ok_or("failed to get selected flag")?,
            )
            .ok_or("failed to get selected flag")?;
        let [header, description, submission] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(a);

        let style1 = match self.state {
            BrowseScreenState::Browse => Style::new()
                .fg(self.conf.theme.base04)
                .bg(self.conf.theme.base00),
            BrowseScreenState::Submit => Style::new()
                .fg(self.conf.theme.base07)
                .bg(self.conf.theme.base00),
        };

        let style2 = match self.state {
            BrowseScreenState::Browse => Style::new()
                .fg(self.conf.theme.base04)
                .bg(self.conf.theme.base00),
            BrowseScreenState::Submit => Style::new()
                .fg(self.conf.theme.base08)
                .bg(self.conf.theme.base00),
        };

        let title = Paragraph::new(format!(
            "{}\nPoints - {}{}",
            flag.name(),
            flag.points(),
            if flag.solved() { " - SOLVED" } else { "" }
        ))
        .style(style2)
        .bold()
        .italic()
        .centered()
        .block(Block::new().borders(Borders::BOTTOM).border_style(style1));
        f.render_widget(title, header);

        let description_text = Paragraph::new(flag.description())
            .wrap(ratatui::widgets::Wrap { trim: false })
            .scroll((self.scroll, 0))
            .style(style1);

        f.render_widget(description_text, description);

        let input_box = Paragraph::new(self.submission.as_str())
            .block(Block::bordered().title_top("Submit Flag").style(style2));

        f.render_widget(input_box, submission);

        Ok(())
    }

    fn reload(&mut self) -> Option<Box<dyn Screen + Sync + Send>> {
        let _ = self.user.reload().is_err_and(|x| {
            self.error = Some(x.to_string());
            true
        });
        match Flag::get_all_with_user(&self.user) {
            Ok(flags) => self.flags = flags,
            Err(e) => self.error = Some(e.to_string()),
        };
        None
    }
}
