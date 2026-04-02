use std::time::Duration;

use crossterm::event::{Event, poll, read};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::system_information::SystemInformation;

pub struct App {
    system_information: SystemInformation,
    should_quit: bool,
}

impl App {
    pub fn init() -> Self {
        App {
            system_information: SystemInformation::init(),
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> color_eyre::Result<()> {
        color_eyre::install()?;
        ratatui::run(|terminal| self.app(terminal))?;
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let mut output: String = String::new();

        output.push_str(&format!("cpu: {}\n", self.system_information.cpu));
        output.push_str(&format!("memory: {}\n", self.system_information.memory));
        for disk_info in self.system_information.disk.iter() {
            output.push_str(&format!("{}: {}\n", disk_info.name, disk_info.percent));
        }
        let paragraph = Paragraph::new(output)
            .block(Block::default().borders(Borders::ALL).title("System Stats"));

        frame.render_widget(paragraph, frame.area());
    }

    fn check_for_input(&mut self) -> std::io::Result<()> {
        if poll(Duration::from_millis(250))? && matches!(read()?, Event::Key(_)) {
            self.should_quit = true;
        }
        Ok(())
    }

    fn app(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.should_quit {
            self.system_information = SystemInformation::init();
            terminal.draw(|frame| self.render(frame))?;
            self.check_for_input()?;
        }
        Ok(())
    }
}
