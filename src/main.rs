use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

#[cfg(test)]
pub mod test;


#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}


impl App{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events().wrap_err("handle events failed")?;
        }
        Ok(())
    }

    fn draw (&self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) -> Result<()> {
        self.counter += 1;
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        if self.counter == 0 {
            bail!("Cant go past 0");
        }else {
            self.counter -= 1;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()>{
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            //change later
            KeyCode::Left => self.decrement_counter()?,
            KeyCode::Right => self.increment_counter()?,
            _ => {}
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()>{
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("Erreur handling key event:\n{key_event:#?}")),
            _ => Ok(())
        }
    }
}

impl Widget for &App {
    fn render (self, area: Rect, buf: &mut Buffer){
        let title = Line::from(" WifiTui ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),

        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}



