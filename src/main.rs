use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub mod game;

use game::game_config::{ GameConfig };
use game::game_field::{ GameField };
use game::game_process::{ GameProcess };
use game::game_status::{ GameStatus };
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

    let game_config = GameConfig {stone_nums_in_hole: 6, hole_nums: 6, is_check_pipe_rule: true};
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
        panic!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, game_process: &mut GameProcess) -> io::Result<()> {

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(2), Constraint::Length(1), Constraint::Min(0)].as_ref())
                .split(f.area());

            let mut side_strs = Vec::new();
            for side in [&game_process.game_field.side_one, &game_process.game_field.side_two] {
                let mut side_vec = Vec::new();
                
                for hole in &side.holes {
                    side_vec.push(hole.stones.len());
                }
                
                side_strs.push(side_vec);
            }

            side_strs[1] = side_strs[1].iter().copied().rev().collect();

            let side_strs_str: Vec<String> = side_strs
                .iter()
                .map(|ss| 
                    ss.iter()
                        .map(|el| format!("[{}]", el))
                        .collect::<Vec<String>>()
                        .join(" ")
                ).collect();

            let top_row = Paragraph::new(format!("{}: {}", "P2".to_string(), side_strs_str[1]));
            let bottom_row = Paragraph::new(format!("{}{}: {}", " ".repeat(4), side_strs_str[0], "P1".to_string()));
            let midle_row = Paragraph::new(format!("{}{}{}", game_process.player_two.score, " ".repeat(side_strs_str[0].len() + 5), game_process.player_one.score));
            
            let player_turn_str = match game_process.is_player_one_turn {
                true => format!("Is {} turn", game_process.player_one.name),
                _ => format!("Is {} turn", game_process.player_two.name),
            };
            let block = Block::default()
                .title(format!("Kalah - Press 1–{} to move, q to quit. {}", game_process.game_config.hole_nums, player_turn_str))
                .borders(Borders::ALL);

            f.render_widget(block, f.area());
            f.render_widget(top_row, chunks[0]);
            f.render_widget(midle_row, chunks[1]);
            f.render_widget(bottom_row, chunks[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {

                    KeyCode::Char(c) if c >= '1'
                        && c <= char::from_digit(game_process.game_config.hole_nums as u32, 10).unwrap() =>
                    {
                        let hole_num = c.to_digit(10).unwrap() as usize;

                        match game_process.move_stones_from_hole(hole_num) {
                            Ok(GameStatus::Finished) => {
                                // Clear screen and draw final score
                                terminal.draw(|f| {
                                    let score_text = format!(
                                        "🏁 Game Over!\n\nPlayer 1 Score: {}\nPlayer 2 Score: {}\n\nPress any key to exit.",
                                        game_process.player_one.score,
                                        game_process.player_two.score
                                    );

                                    let block = Block::default().title("Kalah").borders(Borders::ALL);
                                    let paragraph = Paragraph::new(score_text)
                                        .block(block)
                                        .alignment(ratatui::layout::Alignment::Center);

                                    f.render_widget(paragraph, f.area());
                                })?;

                                // Wait for user to press any key before quitting
                                loop {
                                    if let Event::Key(_) = event::read()? {
                                        return Ok(()); // Exit the game
                                    }
                                }
                            }

                            Ok(_) => continue, // Game continues

                            Err(e) => {
                                // Optional: Show error message if needed
                                // eprintln!("Move error: {}", e);
                                continue;
                            }
                        }
                    }

                    KeyCode::Char('q') => return Ok(()),

                    _ => {}
                }
            }
        }
    }
}
