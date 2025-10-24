use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{text::Text, 
    Frame, 
    widgets::{Paragraph, Block, List, ListDirection, ListItem, ListState},
    layout::{Constraint, Layout, Rect, Margin},
    style::{Style, Stylize}};
use std::io::Result;
use ratatui::widgets::StatefulWidget;

pub struct App {
    app_state: AppState
}

pub struct AppState {
    direction: ListState
}

impl App {
    pub fn new() -> Self {
        let gg = AppState{direction: ListState::default()};
        let mut terminal = ratatui::init();
        let mut result = Self {
            app_state: gg
        };
        result.run(&mut terminal);
        ratatui::restore();
        result
    }

    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        //Ciclo principal
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if self.handle_events()? {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

//        let vertical = Layout::vertical([Fill(1)]);
//        let [borde] = vertical.margin(3).areas(frame.area());

//        frame.render_widget(Block::bordered().title("gg"), borde);

        let items = ["Item 1", "Item 2", "Item 3"];

        let list = List::new(items)
            .block(Block::bordered().title("List"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        frame.render_stateful_widget(list, frame.area().inner(Margin { vertical: 2, horizontal: 2}), &mut self.app_state.direction);

//        StatefulWidget::render(list, frame.area(), frame.buffer_mut(), &mut self.app_state.direction);

    }

    fn handle_events(&mut self) -> Result<bool> {
         match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => self.select_next(),
                KeyCode::Down => self.select_previous(),
                // handle: other key events
                _ => {}
            },
            // handle other events
            _ => {}
         }
        Ok(false)
    }

    fn select_next(&mut self) {
        self.app_state.direction.select_next();
    }
    
    fn select_previous(&mut self) {
        self.app_state.direction.select_previous();
    }
}
