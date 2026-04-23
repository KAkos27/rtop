use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
};

use crate::system_information::ProcessInformation;

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

pub fn create_input_widget<'a>(input: &'a str) -> Paragraph<'a> {
    Paragraph::new(input).block(
        Block::default()
            .title("Filter (f - filter, c - clear)")
            .borders(Borders::ALL),
    )
}

pub fn create_processes_table<'a>(
    process_info: &Vec<ProcessInformation>,
    sort_by_cpu: bool,
) -> Table<'a> {
    let rows = process_info.iter().map(|p| {
        Row::new([
            p.pid.to_string(),
            p.name.to_string(),
            p.cpu_usage.to_string(),
            p.memory_usage.to_string(),
        ])
    });

    let mut cpu_color: Color = Color::White;
    let mut memory_color: Color = Color::White;

    if sort_by_cpu {
        cpu_color = Color::Blue;
    } else {
        memory_color = Color::Blue;
    }

    let header_cells = vec![
        Cell::from("PID").style(Style::new().bold()),
        Cell::from("Name").style(Style::new().bold()),
        Cell::from("CPU usage").style(Style::new().bold().fg(cpu_color)),
        Cell::from("Memory usage").style(Style::new().bold().fg(memory_color)),
    ];

    let header = Row::new(header_cells).bottom_margin(1);

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
