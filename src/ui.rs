use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Row, Table},
};

use crate::system_information::ProcessInformation;

const HEADER_CELLS: [&str; 4] = ["PID", "Name", "CPU usage", "Memory usage"];

pub fn create_disk_widget<'a>(name: &'a str, percent: u16) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().title(name).borders(Borders::ALL))
        .percent(percent)
}

pub fn create_cpu_widget<'a>(percent: u16) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().title("CPU").borders(Borders::ALL))
        .percent(percent)
}

pub fn create_core_gauge<'a>(label: String, percent: u16) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().title(label).borders(Borders::ALL))
        .percent(percent)
}

pub fn create_memory_widget<'a>(percent: u16) -> Gauge<'a> {
    Gauge::default()
        .block(Block::default().title("MEMORY").borders(Borders::ALL))
        .percent(percent)
}

pub fn create_processes_table<'a>(process_info: &Vec<ProcessInformation>) -> Table<'a> {
    let rows = process_info.iter().map(|p| {
        Row::new([
            p.pid.to_string(),
            p.name.to_string(),
            p.cpu_usage.to_string(),
            p.memory_usage.to_string(),
        ])
    });
    let header = Row::new(HEADER_CELLS)
        .style(Style::new().bold())
        .bottom_margin(1);

    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(50),
        Constraint::Percentage(10),
        Constraint::Percentage(20),
    ];

    Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold().light_blue())
        .highlight_symbol(" > ")
}
