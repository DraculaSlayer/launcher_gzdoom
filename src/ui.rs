use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use color_eyre::Result as OtherResult;
use ratatui::{text::Text, 
    Frame,
    symbols,
    widgets::{Paragraph, Block, Tabs, List, ListDirection, ListItem, ListState},
    layout::{Constraint, Layout, Rect, Margin},
    style::{Style, Stylize}};
use std::io::Result;
use ratatui::widgets::StatefulWidget;
use ratatui::prelude::Widget;

pub struct App {
    app_state: AppState,
    items: Vec<String>,
    tabs: Vec<String>,
    tabs_index: usize
}

struct AppState {
    direction: ListState
}

impl App {
    pub fn new() -> Self {
        let gg = AppState{direction: ListState::default()};
        color_eyre::install().expect("FALLO");
        let mut terminal = ratatui::init();
        let mut result = Self {
            app_state: gg,
            items: Vec::new(),
            tabs: Vec::new(),
            tabs_index: 0
        };
        result.run(&mut terminal);
        ratatui::restore();
        result
    }

    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        //Main loop
        loop {
            terminal.draw(|frame| self.draw_main(frame))?;
            if self.handle_events()? {
                break Ok(());
            }
        }
    }

    fn draw_main(&mut self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};
        
        self.items = vec!["Item 1".to_string(), "Item 2".to_string(), "Item 3".to_string()];
        self.tabs = vec!["Tabs1".to_string(), "Tabs2".to_string(), "Tabs3".to_string()];

        //Draw the list section
        self.draw_list(frame);

        //Draw the tab section
        self.draw_tabs(frame);

    }

    fn handle_events(&mut self) -> Result<bool> {
         match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => self.select_next(),
                KeyCode::Down => self.select_previous(),
                KeyCode::Enter => self.selected(),
                KeyCode::Char('h') => self.select_next_tabs(),
                KeyCode::Char('j') => self.select_previous_tabs(),
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
    fn selected(&mut self) {

        if let Some(select) = self.app_state.direction.selected() {
            println!("{}", self.items[select]);
        }
    }
    fn select_next_tabs(&mut self) {
        if self.tabs_index < (self.tabs.len() -1) {
            self.tabs_index += 1;
        }
    }
    fn select_previous_tabs(&mut self) {
        if self.tabs_index > 0 {
            self.tabs_index -= 1;
        }
    }

    fn draw_tabs(&mut self, frame: &mut Frame) {
        let index = self.tabs_index as usize;

        let tabs = Tabs::new(self.tabs.clone())
                    .block(Block::bordered())
                    .style(Style::default().white())
                    .highlight_style(Style::default().yellow())
                    .select(index)
                    .divider(symbols::DOT)
                    .padding("->", "<-")
                    .render(frame.area(), frame.buffer_mut());

    }

    fn draw_list(&mut self, frame: &mut Frame) {
        let list = List::new(self.items.clone())
            .block(Block::bordered().title("List"))
            .style(Style::new().white())
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);
            
        frame.render_stateful_widget(list,
                                    frame.area().inner(Margin { vertical: 2, horizontal: 2}),
                                    &mut self.app_state.direction);
    }
}
