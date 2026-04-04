use std::{time::Duration, usize};

use crossterm::event::{self, KeyCode, poll};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Gauge, Row, Table, TableState},
};
use sysinfo::{Disks, Signal, System};

use crate::system_information::SystemInformation;

pub struct App {
    system_information: SystemInformation,
    system: System,
    disks: Disks,
    should_quit: bool,
    table_state: TableState,
}

impl App {
    pub fn init() -> Self {
        let mut system: System = System::new_all();
        let disks: Disks = Disks::new_with_refreshed_list();
        system.refresh_all();

        let mut app = App {
            system_information: SystemInformation::get_system_info(&system, &disks),
            system,
            disks,
            should_quit: false,
            table_state: TableState::default(),
        };
        app.table_state.select_first();

        app
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        ratatui::run(|terminal| self.app(terminal))?;
        Ok(())
    }

    fn render_processes_table(&mut self, frame: &mut Frame, area: Rect) {
        let header = Row::new(["PID", "Name", "CPU usage", "Memory usage"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows = self.system_information.processes.iter().map(|p| {
            Row::new([
                p.pid.to_string(),
                p.name.to_string(),
                p.cpu_usage.to_string(),
                p.memory_usage.to_string(),
            ])
        });

        let footer = Row::new(["Showing the first 25 processes", ""]);
        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .footer(footer.italic())
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold().light_blue())
            .highlight_symbol(" > ");

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render(&mut self, frame: &mut Frame) {
        let mut constraints: Vec<Constraint> = vec![Constraint::Length(3), Constraint::Length(3)];
        let mut current_chunk_index: usize = 0;

        for _ in self.system_information.disk.iter() {
            constraints.push(Constraint::Length(3));
        }

        constraints.push(Constraint::Min(0));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        frame.render_widget(
            Gauge::default()
                .block(Block::default().title("CPU").borders(Borders::ALL))
                .percent(self.system_information.cpu as u16),
            chunks[current_chunk_index],
        );
        current_chunk_index += 1;

        frame.render_widget(
            Gauge::default()
                .block(Block::default().title("MEMORY").borders(Borders::ALL))
                .percent(self.system_information.memory as u16),
            chunks[current_chunk_index],
        );
        current_chunk_index += 1;

        for disk in self.system_information.disk.iter() {
            frame.render_widget(
                Gauge::default()
                    .block(
                        Block::default()
                            .title(disk.name.as_str())
                            .borders(Borders::ALL),
                    )
                    .percent(disk.percent as u16),
                chunks[current_chunk_index],
            );
            current_chunk_index += 1;
        }

        self.render_processes_table(frame, chunks[current_chunk_index]);
        // current_chunk_index += 1;
    }

    fn kill_process(&self) {
        if let Some(selected_index) = self.table_state.selected() {
            if let Some(selected_process) = self.system_information.processes.get(selected_index) {
                if let Some(process) = self.system.process(selected_process.pid) {
                    process.kill_with(Signal::Kill);
                }
            }
        }
    }

    fn check_for_input(&mut self) -> std::io::Result<()> {
        if poll(Duration::from_millis(50))? {
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
                    KeyCode::Char('x') | KeyCode::Backspace => self.kill_process(),
                    KeyCode::Char('g') => self.table_state.select_first(),
                    KeyCode::Char('G') => self.table_state.select_last(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn app(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_quit {
            self.system.refresh_all();
            self.system_information = SystemInformation::get_system_info(&self.system, &self.disks);
            terminal.draw(|frame| self.render(frame))?;
            self.check_for_input()?;
        }
        Ok(())
    }
}
