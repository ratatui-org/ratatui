#[cfg(feature = "termion")]
#[test]
fn backend_termion_should_only_write_diffs() -> Result<(), Box<dyn std::error::Error>> {
    use std::{fmt::Write, io::Cursor};

    let mut bytes = Vec::new();
    let mut stdout = Cursor::new(&mut bytes);
    {
        use ratatui::{backend::TermionBackend, layout::Rect, widgets::Paragraph, Terminal};
        let backend = TermionBackend::new(&mut stdout);
        let area = Rect::new(0, 0, 3, 1);
        let mut terminal = Terminal::new(backend)?;
        terminal.resize_buffer_abs(3, 1);
        terminal.render_widget(&Paragraph::new("a"), &area);
        terminal.flush()?;
        terminal.render_widget(&Paragraph::new("ab"), &area);
        terminal.flush()?;
        terminal.render_widget(&Paragraph::new("abc"), &area);
        terminal.flush()?;
    }

    let expected = {
        use termion::{color, cursor, style};
        let mut s = String::new();
        // First draw
        write!(s, "{}", cursor::Goto(1, 1))?;
        s.push('a');
        write!(s, "{}", color::Fg(color::Reset))?;
        write!(s, "{}", color::Bg(color::Reset))?;
        write!(s, "{}", style::Reset)?;
        write!(s, "{}", cursor::Hide)?;
        // Second draw
        write!(s, "{}", cursor::Goto(2, 1))?;
        s.push('b');
        write!(s, "{}", color::Fg(color::Reset))?;
        write!(s, "{}", color::Bg(color::Reset))?;
        write!(s, "{}", style::Reset)?;
        write!(s, "{}", cursor::Hide)?;
        // Third draw
        write!(s, "{}", cursor::Goto(3, 1))?;
        s.push('c');
        write!(s, "{}", color::Fg(color::Reset))?;
        write!(s, "{}", color::Bg(color::Reset))?;
        write!(s, "{}", style::Reset)?;
        write!(s, "{}", cursor::Hide)?;
        // Terminal drop
        write!(s, "{}", cursor::Show)?;
        s
    };
    assert_eq!(std::str::from_utf8(&bytes)?, expected);

    Ok(())
}
