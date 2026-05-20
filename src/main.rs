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

    frame.render_widget(git_log().expect("error git"), layout[0]);
    frame.render_widget("Right side | diff", layout[1]);
}

fn git_log() -> Result<String, std::io::Error> {
    use std::process::Command;

    let output = Command::new("git")
        .args(["log", "--oneline", "--graph"])
        .output()?;

    if !output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
