use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph, Widget},
    Terminal,
};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the TUI app
    if let Err(e) = run_app(&mut terminal).await {
        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        eprintln!("Error: {}", e);
    }

    // Restore the terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            // Layout split between the left and right panes
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(1),  // Add a padding between left and right panes
                    Constraint::Percentage(68),
                ].as_ref())
                .split(f.size());

            // Left pane
            let ports_list = List::new(vec![
                ListItem::new("COM1"),
                ListItem::new("COM2"),
                ListItem::new("COM3"),
                ListItem::new("COM4"),
            ]).block(Block::default().title("Available Ports").borders(Borders::ALL));
            f.render_widget(ports_list, chunks[0]);  // Render these in left chunk/pane.

            // Right pane layout: Filters, incoming data, input area
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ].as_ref())
                .split(chunks[2]);

            // Filters
            let filters = List::new(vec![
                ListItem::new("Hex"),
                ListItem::new("Binary"),
                ListItem::new("Text"),
                ListItem::new("Serial Plotter"),
            ]).block(Block::default().title("Filters").borders(Borders::ALL));

            // Data
            let incoming_data = Paragraph::new("Incoming data will be displayed here!")
                .block(Block::default().title("Data").borders(Borders::ALL));
            
            // Send data
            let input_area = Paragraph::new("Type data to send here!")
                .block(Block::default().title("Send Data").borders(Borders::ALL));

            // Render the right chunks
            f.render_widget(filters, right_chunks[0]);
            f.render_widget(incoming_data, right_chunks[1]);
            f.render_widget(input_area, right_chunks[2]);
        })?;

        // Handle input
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                // catch Ctrl+C
                KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
