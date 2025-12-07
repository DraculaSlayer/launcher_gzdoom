use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use color_eyre::Result as OtherResult;
use ratatui::{text::Text, 
    Frame,
    symbols,
    widgets::{Paragraph, Block, Tabs, List, ListDirection, ListItem, ListState},
    layout::{Constraint, Layout, Rect, Margin},
    style::{Style, Stylize, Color, Modifier}};
use std::io::Result;
use ratatui::widgets::StatefulWidget;
use ratatui::prelude::Widget;
use crate::scan_directory;
use crate::execute;

pub struct App {
    app_state: AppState,
    items_wads: Vec<String>,
    items_pk3: Vec<String>,
    tabs: Vec<String>,
    tabs_index: usize,
    selected_item_tabs: Vec<isize>,
    style: StyleElement
}

struct AppState {
    direction: ListState
}

struct StyleElement {
    style_list: Style
}

impl App {
    pub fn new() -> Self {

        let state = AppState{direction: ListState::default()};

        let style = StyleElement{style_list: Style::default().fg(Color::Cyan)};

        color_eyre::install().expect("FAILED");

        let mut terminal = ratatui::init();

        let scan_dir = scan_directory::ScanDir::new();

        let mut result = Self {
            app_state: state,
            items_wads: scan_dir.list_wad(true),
            items_pk3:  scan_dir.list_pk3(true),
            tabs: vec!["Wads".to_string(), "Mods".to_string(), "Config".to_string()],
            tabs_index: 0,
            selected_item_tabs: vec![-1, -1, -1],
            style: style
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
        
        //Draw the list section select
        if self.tabs_index == 0 {
            self.draw_list(frame, self.items_wads.clone());
        }
        if self.tabs_index == 1 {
            self.draw_list(frame, self.items_pk3.clone());
        }

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
                KeyCode::Tab => self.select_next_tabs(),
                KeyCode::F(5) => self.execute()?,
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
       
        self.style.style_list = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);

        if let Some(select) = self.app_state.direction.selected() {
            self.selected_item_tabs[self.tabs_index] = (select as isize);
        }

    }

    fn select_next_tabs(&mut self) {

        if self.tabs_index >= (self.tabs.len() - 1) {
            self.tabs_index = 0;
        }else {
            self.tabs_index += 1;
        }

    }

    fn draw_tabs(&mut self, frame: &mut Frame) {
        let index = self.tabs_index as usize;

        Tabs::new(self.tabs.clone())
             .block(Block::bordered())
             .style(Style::default().white())
             .highlight_style(Style::default().yellow())
             .select(index)
             .divider(symbols::DOT)
             .padding("->", "<-")
             .render(frame.area(), frame.buffer_mut());

    }

    fn draw_list(&mut self, frame: &mut Frame, list: Vec<String>) {
        let list = List::new(list)
            .block(Block::bordered())
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop)
            .highlight_style(self.style.style_list)
            .highlight_symbol(">>");
            
        frame.render_stateful_widget(list,
                                    frame.area().inner(Margin { vertical: 2, horizontal: 2}),
                                    &mut self.app_state.direction);
    }

    fn execute(&self) -> std::io::Result<()> {

        let scan_dir = scan_directory::ScanDir::new();

        let list_wad = scan_dir.list_wad(false);
        let list_pk3 = scan_dir.list_pk3(false);

        let mut wad: String = String::new();

        let mut list: Vec<String> = Vec::new();

        if self.selected_item_tabs[0] == -1 {
            println!("Falta un WAD");
            return Ok(());
        }else {
            list.push(list_wad[self.selected_item_tabs[0] as usize].clone()).clone();
        }

        if self.selected_item_tabs[1] == -1 {list.push("None".to_string())}else {
            list.push(list_pk3[self.selected_item_tabs[1] as usize].clone()).clone();
        }

        wad = list_wad[self.selected_item_tabs[0] as usize].clone();

        execute::ExecuteDoom::execute(wad, list);

        Ok(())
    }
}
