use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Borders, Cell, Gauge, Paragraph, Row, Table},
};

use crate::system_information::ProcessInformation;

fn gauge_color(percent: u16) -> Color {
    match percent {
        0..=50 => Color::Green,
        51..=75 => Color::Yellow,
        _ => Color::Red,
    }
}

fn cpu_value_color(usage: f32) -> Color {
    match usage as u16 {
        0..=10 => Color::Green,
        11..=50 => Color::Yellow,
        _ => Color::Red,
    }
}

fn mem_value_color(bytes: u64) -> Color {
    let mb = bytes / (1024 * 1024);
    match mb {
        0..=500 => Color::Green,
        501..=2000 => Color::Yellow,
        _ => Color::Red,
    }
}

fn format_memory(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    const KB: u64 = 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

fn gauge_block<'a>(title: impl Into<Span<'a>>) -> Block<'a> {
    Block::default()
        .title(title.into().bold().fg(Color::Cyan))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::DarkGray))
}

fn styled_gauge<'a>(block: Block<'a>, percent: u16) -> Gauge<'a> {
    let color = gauge_color(percent);
    Gauge::default()
        .block(block)
        .gauge_style(Style::new().fg(color).bg(Color::DarkGray))
        .percent(percent)
        .label(Span::styled(
            format!("{percent}%"),
            Style::new().fg(Color::White).bold(),
        ))
        .use_unicode(true)
}

pub fn create_cpu_widget<'a>(percent: u16) -> Gauge<'a> {
    styled_gauge(gauge_block("CPU"), percent)
}

pub fn create_core_gauge<'a>(label: String, percent: u16) -> Gauge<'a> {
    styled_gauge(gauge_block(label), percent)
}

pub fn create_memory_widget<'a>(percent: u16) -> Gauge<'a> {
    styled_gauge(gauge_block("MEMORY"), percent)
}

pub fn create_disk_widget<'a>(name: &'a str, percent: u16) -> Gauge<'a> {
    styled_gauge(gauge_block(name), percent as u16)
}

pub fn create_input_widget<'a>(input: &'a str, is_editing: bool) -> Paragraph<'a> {
    let (border_color, title) = if is_editing {
        (
            Color::Cyan,
            Span::styled("Filter", Style::new().fg(Color::Cyan).bold()),
        )
    } else {
        (
            Color::DarkGray,
            Span::styled(
                "Filter (f - filter, c - clear)",
                Style::new().fg(Color::DarkGray),
            ),
        )
    };

    Paragraph::new(input).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(border_color)),
    )
}

pub fn create_processes_table<'a>(
    process_info: &Vec<ProcessInformation>,
    sort_by_cpu: bool,
) -> Table<'a> {
    let rows: Vec<Row> = process_info
        .iter()
        .map(|p| {
            Row::new([
                Cell::from(p.pid.to_string()).style(Style::new().fg(Color::DarkGray)),
                Cell::from(p.name.to_string()).style(Style::new().fg(Color::White)),
                Cell::from(format!("{:.1}%", p.cpu_usage))
                    .style(Style::new().fg(cpu_value_color(p.cpu_usage)).bold()),
                Cell::from(format_memory(p.memory_usage))
                    .style(Style::new().fg(mem_value_color(p.memory_usage))),
            ])
        })
        .collect();

    let (cpu_header_style, mem_header_style) = if sort_by_cpu {
        (
            Style::new().bold().fg(Color::Cyan).underlined(),
            Style::new().bold().fg(Color::Gray),
        )
    } else {
        (
            Style::new().bold().fg(Color::Gray),
            Style::new().bold().fg(Color::Cyan).underlined(),
        )
    };

    let header = Row::new([
        Cell::from("PID").style(Style::new().bold().fg(Color::Gray)),
        Cell::from("Name").style(Style::new().bold().fg(Color::Gray)),
        Cell::from("CPU%").style(cpu_header_style),
        Cell::from("Memory").style(mem_header_style),
    ])
    .bottom_margin(1);

    let widths = [
        Constraint::Percentage(15),
        Constraint::Percentage(45),
        Constraint::Percentage(15),
        Constraint::Percentage(25),
    ];

    Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(Span::styled("Processes", Style::new().fg(Color::Cyan).bold(),
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::DarkGray)),
        )
        .column_spacing(1)
        .row_highlight_style(Style::new().bg(Color::DarkGray).bold().fg(Color::White))
        .highlight_symbol(" ▶ ")
}
