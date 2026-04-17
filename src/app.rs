use crossterm::event::{self, KeyCode, KeyEventKind, poll};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Position, Rect},
    widgets::TableState,
};
use sysinfo::{Disks, MINIMUM_CPU_UPDATE_INTERVAL, Signal, System};

use crate::{system_information::SystemInformation, ui};

enum InputMode {
    Normal,
    Editing,
}

pub struct App {
    system_information: SystemInformation,
    system: System,
    disks: Disks,
    should_quit: bool,
    should_sort: bool,
    sort_by_cpu: bool,
    table_state: TableState,
    input: String,
    character_index: usize,
    input_mode: InputMode,
    messages: String,
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
            should_sort: true,
            sort_by_cpu: true,
            table_state: TableState::default(),
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: String::new(),
            character_index: 0,
        };
        app.table_state.select_first();

        app
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        ratatui::run(|terminal| self.app(terminal))?;
        Ok(())
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    const fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        self.messages = self.input.clone();
        self.input.clear();
        self.reset_cursor();
        self.should_sort = true;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn render_top(&mut self, frame: &mut Frame, chunk: Rect) {
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
            .split(chunk);

        frame.render_widget(
            ui::create_cpu_widget(self.system_information.cpu_information.percentage),
            top_chunks[0],
        );

        let cores = &self.system_information.cpu_information.cores;
        const COLS: usize = 2;
        let col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, COLS as u32); COLS])
            .split(top_chunks[1]);

        for col in 0..COLS {
            let col_cores: Vec<(usize, f32)> = cores
                .iter()
                .enumerate()
                .filter(|(i, _)| i % COLS == col)
                .map(|(i, &v)| (i, v))
                .collect();

            let row_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3); col_cores.len()])
                .split(col_chunks[col]);

            for (row, (i, usage)) in col_cores.iter().enumerate() {
                frame.render_widget(
                    ui::create_core_gauge(format!("C{i}"), *usage as u16),
                    row_chunks[row],
                );
            }
        }
    }

    fn render_bottom(&mut self, frame: &mut Frame, chunk: Rect) {
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunk);

        let mut left_constraints = vec![Constraint::Length(3)];
        for _ in &self.system_information.disk {
            left_constraints.push(Constraint::Length(3));
        }
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(left_constraints)
            .split(bottom_chunks[0]);

        frame.render_widget(
            ui::create_memory_widget(self.system_information.memory as u16),
            left_chunks[0],
        );
        for (i, disk) in self.system_information.disk.iter().enumerate() {
            frame.render_widget(
                ui::create_disk_widget(disk.name.as_str(), disk.percent as u16),
                left_chunks[i + 1],
            );
        }

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(bottom_chunks[1]);

        frame.render_widget(ui::create_input_widget(&self.input), right_chunks[0]);
        match self.input_mode {
            InputMode::Normal => {}

            #[expect(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                right_chunks[0].x + self.character_index as u16 + 1,
                right_chunks[0].y + 1,
            )),
        }
        self.render_process_table(frame, right_chunks[1]);
    }

    fn render_process_table(&mut self, frame: &mut Frame, chunk: Rect) {
        let table =
            ui::create_processes_table(&self.system_information.processes, self.sort_by_cpu);
        frame.render_stateful_widget(table, chunk, &mut self.table_state);
    }

    fn render(&mut self, frame: &mut Frame) {
        let core_count = self.system_information.cpu_information.cores.len() as u16;
        let top_height = ((core_count + 1) / 2 * 3).max(3);

        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(top_height), Constraint::Min(0)])
            .split(frame.area());

        self.render_top(frame, main_chunks[0]);
        self.render_bottom(frame, main_chunks[1]);
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

    fn sort(&mut self) {
        if self.should_sort {
            if self.sort_by_cpu {
                self.system_information
                    .processes
                    .sort_by(|a, b| b.cpu_usage.total_cmp(&a.cpu_usage));
            } else {
                self.system_information
                    .processes
                    .sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            }
        }
    }

    fn enter_insert_mode(&mut self) {
        self.should_sort = false;
        self.input_mode = InputMode::Editing;
    }

    fn escape_insert_mode(&mut self) {
        self.input = String::new();
        self.should_sort = true;
        self.input_mode = InputMode::Normal;
    }

    fn check_for_input(&mut self) -> std::io::Result<()> {
        if poll(MINIMUM_CPU_UPDATE_INTERVAL)? {
            if let Some(key) = event::read()?.as_key_press_event() {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                        KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                        KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
                        KeyCode::Char('x') | KeyCode::Backspace => self.kill_process(),
                        KeyCode::Char('g') => self.table_state.select_first(),
                        KeyCode::Char('G') => self.table_state.select_last(),
                        KeyCode::Char('s') => self.sort_by_cpu = !self.sort_by_cpu,
                        KeyCode::Char('f') => self.enter_insert_mode(),
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.escape_insert_mode(),
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
        Ok(())
    }

    fn app(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_quit {
            self.system.refresh_all();
            self.system_information = SystemInformation::get_system_info(&self.system, &self.disks);
            self.sort();
            terminal.draw(|frame| self.render(frame))?;
            self.check_for_input()?;
        }
        Ok(())
    }
}
