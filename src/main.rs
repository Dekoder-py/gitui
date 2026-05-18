use ratatui::{
    DefaultTerminal, Frame, crossterm,
    layout::{Constraint, Layout},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(50)])
        .split(frame.area());

    frame.render_widget("Left side | log", layout[0]);
    frame.render_widget("Right side | diff", layout[1]);
}
