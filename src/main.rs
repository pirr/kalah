use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use kalah::{GameConfig, GameField, GameProcess};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{char, error::Error, io};


fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let game_config = GameConfig {stone_nums_in_hole: 6, hole_nums: 6};
    let game_field = GameField::build(&game_config);
    let player_one_name = "Player1".to_string();
    let player_two_name = "Player2".to_string();
    let mut game_process = GameProcess::build(game_field, player_one_name, player_two_name, game_config);

    // Game loop
    let res = run_app(&mut terminal, &mut game_process);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, game_process: &mut GameProcess) -> io::Result<()> {

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.area());

            let mut side_strs_vec = Vec::new();
            for side in [&game_process.game_field.side_one, &game_process.game_field.side_two] {
                let mut side_str_vec = Vec::new();
                
                for hole in &side.holes {
                    side_str_vec.push(format!("[{}]", hole.stones.len().to_string()));
                }
                
                side_strs_vec.push(side_str_vec.join(" "));
            }

            let top_row = Paragraph::new(format!("{}: {}", "P2".to_string(), side_strs_vec[1].to_string()));
            let bottom_row = Paragraph::new(format!("{}: {}", "P1".to_string(), side_strs_vec[0].to_string()));
            
            let player_turn_str = match game_process.is_player_one_turn {
                true => format!("Is {} turn", game_process.player_one.name),
                _ => format!("Is {} turn", game_process.player_two.name),
            };
            let block = Block::default()
                .title(format!("Kalah - Press 1–6 to move, q to quit. {}", player_turn_str))
                .borders(Borders::ALL);

            f.render_widget(block, f.area());
            f.render_widget(top_row, chunks[0]);
            f.render_widget(bottom_row, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) if c >= '1' && c <= char::from_digit(game_process.game_config.hole_nums as u32, 10).unwrap() => {
                        let hole_num = c.to_digit(10).unwrap() as usize;

                        // Now you can use the selected hole
                        let _ = game_process.move_stones_from_hole(hole_num);
                    }

                    KeyCode::Char('q') => return Ok(()),

                    _ => {}
                }
            }
        }
    }
}
