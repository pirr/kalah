use kalah::game;

use game::game_config::*;
use game::game_field::*;
use game::game_process::*;

#[test]
fn start_game() {
    let game_config = GameConfig::build(6,6).unwrap();
    
    assert_eq!(game_config.hole_nums, 6);
    assert_eq!(game_config.stone_nums_in_hole, 6);
    
    let game_field = GameField::build(&game_config);

    assert_eq!(game_field.side_one.holes.len(), 6);
    assert_eq!(game_field.side_two.holes.len(), 6);
    assert_eq!(game_field.side_one.holes[0].stones.len(), 6);
    assert_eq!(game_field.side_two.holes[0].stones.len(), 6);
}

#[test]
fn make_move() {
    let game_config = GameConfig::build(6, 6).unwrap();
    
    assert_eq!(game_config.hole_nums, 6);
    assert_eq!(game_config.stone_nums_in_hole, 6);
    
    let game_field = GameField::build(&game_config);

    let player_one_name = "P1".to_string();
    let player_two_name = "P2".to_string();

    let mut game_process = GameProcess::build(game_field, player_one_name, player_two_name, game_config);

    assert!(game_process.is_player_one_turn);

    _ = game_process.move_stones_from_hole(1);

    assert_eq!(game_process.game_field.side_one.holes[1].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[2].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[3].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[4].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[5].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[0].stones.len(), 0);
    assert_eq!(game_process.player_one.score, 1);
    assert_eq!(game_process.total_turns, 1);
    assert!(game_process.is_player_one_turn);

    match game_process.move_stones_from_hole(1) {
        Ok(_) => panic!("Expected error but got Ok"),
        Err(err) => assert_eq!(err, "Selected hole is empty"),
    }

    // nothing changed
    assert_eq!(game_process.game_field.side_one.holes[1].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[2].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[3].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[4].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[5].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[0].stones.len(), 0);
    assert_eq!(game_process.total_turns, 1);
    assert_eq!(game_process.player_one.score, 1);
    assert!(game_process.is_player_one_turn);

    assert_eq!(game_process.game_field.side_two.holes[0].stones.len(), 6);

    // player one move and turn on player two
    _ = game_process.move_stones_from_hole(2);
    assert_eq!(game_process.game_field.side_one.holes[2].stones.len(), 8);
    assert_eq!(game_process.game_field.side_one.holes[3].stones.len(), 8);
    assert_eq!(game_process.game_field.side_one.holes[4].stones.len(), 8);
    assert_eq!(game_process.game_field.side_one.holes[5].stones.len(), 8);

    assert_eq!(game_process.game_field.side_two.holes[0].stones.len(), 7);
    assert_eq!(game_process.game_field.side_one.holes[1].stones.len(), 0);

    assert_eq!(game_process.game_field.side_two.holes[0].stones.len(), 7);

    assert_eq!(game_process.total_turns, 2);
    assert_eq!(game_process.player_one.score, 2);
    assert!(!game_process.is_player_one_turn);
}