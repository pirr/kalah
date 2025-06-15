use kalah::*;

#[test]
fn start_game() {
    let args = ["", "6", "6"];
    let game_config = GameConfig::build(&args).unwrap();
    
    assert_eq!(game_config.hole_nums, 6);
    assert_eq!(game_config.stone_nums_in_hole, 6);
    
    let game_field = GameField::build(&game_config);

    assert_eq!(game_field.side_one.holes.len(), 6);
    assert_eq!(game_field.side_two.holes.len(), 6);
    assert_eq!(game_field.side_one.holes[0].stones.len(), 6);
    assert_eq!(game_field.side_two.holes[0].stones.len(), 6);

}

fn make_move() {
    let args = ["", "6", "6"];
    let game_config = GameConfig::build(&args).unwrap();
    
    assert_eq!(game_config.hole_nums, 6);
    assert_eq!(game_config.stone_nums_in_hole, 6);
    
    let game_field = GameField::build(&game_config);

    // move_stone(game_field.side_one);
}