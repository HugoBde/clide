use std::io::stdout;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, ExecutableCommand};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

use crate::claude::Client;

pub struct App {
    client:          Client,
    messages:        Vec<Message>,
    editing_message: String,
    exit:            bool,
}

impl App {
    pub fn new(api_key: String) -> Self {
        App {
            messages:        vec![],
            editing_message: String::new(),
            client:          Client::new(api_key),
            exit:            false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(std::time::Duration::from_millis(1000 / 60))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => self.exit = true,

                            KeyCode::Enter => self.send_message(),

                            // drop to discard the value and avoid having incompatible match arms
                            // types
                            KeyCode::Backspace => drop(self.editing_message.pop()),
                            KeyCode::Char(c) => self.editing_message.push(c),
                            _ => (),
                        };
                    }
                }
            }
        }

        Ok(())
    }

    fn send_message(&mut self) {
        self.messages.push(Message::from(&self.editing_message).user());
        let response = self.client.send_api_request(&self.editing_message);
        self.messages.push(Message::from(&response.content[0].text).assistant());
        self.editing_message.clear();
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.size();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(90), Constraint::Percentage(10)])
            .split(area);

        let title = Title::from(" CLIDE ")
            .position(Position::Top)
            .alignment(Alignment::Center);

        let lines = self.messages.iter().fold(vec![], |mut lines, msg| {
            let (header, content) = msg.render();
            lines.push(header);
            lines.push(content);
            return lines;
        });

        let chat = Paragraph::new(lines)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_set(border::THICK),
            )
            .wrap(Wrap {
                trim: true
            });

        let text_input = Paragraph::new(self.editing_message.as_str())
            .block(Block::default().borders(Borders::ALL).border_set(border::THICK));

        frame.render_widget(chat, layout[0]);
        frame.render_widget(text_input, layout[1]);
    }

    pub fn init(&self) -> Result<()> {
        let mut stdout = std::io::stdout();
        stdout.execute(EnterAlternateScreen)?;
        enable_raw_mode()?;

        Ok(())
    }

    pub fn clean_up(&self) -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}

struct Message {
    content: String,
    role:    MessageRole,
}

impl Message {
    fn from(content: &str) -> Message {
        Message {
            content: String::from(content),
            role:    MessageRole::User,
        }
    }

    fn user(mut self) -> Message {
        self.role = MessageRole::User;
        return self;
    }

    fn assistant(mut self) -> Message {
        self.role = MessageRole::Assistant;
        return self;
    }

    fn render(&self) -> (Line, Line) {
        return (
            Line::from(vec![match self.role {
                MessageRole::User => Span::styled("you", Style::default().fg(Color::Green)),
                MessageRole::Assistant => Span::styled("Claude", Style::default().fg(Color::Yellow)),
            }]),
            Line::from(self.content.as_str()),
        );
    }
}

enum MessageRole {
    User,
    Assistant,
}
