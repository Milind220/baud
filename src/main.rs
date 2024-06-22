use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::Text,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

// TODO: Implement the ui 
// TODO: Get list of ports and display them
// TODO: Make it possible to select a port and connect to it
// TODO: Show port data in the right pane
// TODO: Implement a way to send data to the port
// TODO: Implement a way to save the data to a file

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
            // Layout split betwen the left and right panes
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(f.size());
            // Left pane
            let ports_list = List::new(vec![
                ListItem::new(Text::raw("COM1")),
                ListItem::new(Text::raw("COM2")),
                ListItem::new(Text::raw("COM3")),
                ListItem::new(Text::raw("COM4")),
            ]).block(Block::default().title("Available Ports").borders(Borders::ALL));
            f.render_widget(ports_list, chunks[0]);  // Render these in left chunk/pane.
            // Right pane
            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                    ]
                    .as_ref()
                )
                .split(chunks[1]);
            // Filters
            let filters = List::new(vec![
                ListItem::new(Text::raw("Hex")),
                ListItem::new(Text::raw("Binary")),
                ListItem::new(Text::raw("Text")),
                ListItem::new(Text::raw("Serial Plotter")),
            ]).block(Block::default().title("Filters").borders(Borders::ALL));
            f.render_widget(filters, right_chunks[0]);  // Render these in right chunk/pane, top.

            // Data
            let incoming_data = Paragraph::new("Incoming data will be displayed here!").block(Block::default().title("Data").borders(Borders::ALL));
            f.render_widget(incoming_data, right_chunks[1]);  // Render these in right chunk/pane, middle.
            
            // Send data
            let input_area = Paragraph::new("Type data to send here!").block(Block::default().title("Send Data").borders(Borders::ALL));
            f.render_widget(input_area, right_chunks[2]);  // Render these in right chunk/pane, bottom.
        })?;

        // Handle input

        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                // catch Ctrl+C
                KeyCode::Char('c') if event.modifiers.contains(event::KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}



