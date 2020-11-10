mod engine;

use bevy::prelude::*;
use engine::{
    board,
    board::{Board, BoardLocation, Square, SquareModifier, Tile},
    game,
    game::{Bag, Game, TurnAction, Word},
    player,
    player::Player,
};

fn startup(mut commands: Commands) {
    let mut game = game::Game::new();
    let players: Vec<(Entity, Player)> = (0..4)
        .map(|_| (Entity::new(), Player::new(&mut game)))
        .collect();
    let turn_order: TurnOrder = players
        .iter()
        .map(|(entity, _player)| entity.id())
        .collect();
    players
        .iter()
        .fold(&mut commands, |commands, (entity, player)| {
            commands.spawn_as_entity(*entity, (player,))
        })
        .insert_resource(turn_order)
        .insert_resource(game);
}

type TurnOrder = Vec<u32>;

fn player_turn_handler(
    mut turn: ResMut<u32>,
    turn_order: Res<TurnOrder>,
    mut game: ResMut<Game>,
    player: &mut Player,
) {
    turn_order.iter().map(|id| println!("{:?}", id));
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(startup.system())
        .run();

    // let mut game = game::Game::new();

    // let mut players: Vec<player::Player> = vec![];

    // for _ in 0..4 {
    //     let new_player = player::Player::new(&mut game);
    //     players.push(new_player);
    // }

    // let mut current_player_number = 0;

    // loop {
    //     let current_player = match players.get_mut(current_player_number) {
    //         Some(p) => p,
    //         None => continue,
    //     };

    //     let action = game::TurnAction::DiscardTiles(vec![0, 1, 2]);

    //     match action {
    //         game::TurnAction::PlayWord(x) => current_player.play_word(&x, &game.board),
    //         game::TurnAction::DiscardTiles(x) => {
    //             game.bag.return_tiles(current_player.remove_tiles(x));
    //         }
    //     }
    //     current_player
    //         .hand
    //         .extend(game.bag.draw_tiles(7 - current_player.hand.len()));

    //     current_player_number += 1;
    //     if current_player_number == 4 {
    //         current_player_number = 0;
    //     }

    //     for player in players {
    //         println!("{:#?}", player.hand)
    //     }

    //     break;
    // }
}
