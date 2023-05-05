use rand::prelude::*;
use rusty_engine::prelude::*;

struct GameState {
    health: u8,
    lost: bool,
}

const PLAYER_SPEED: f32 = 1000.0;
const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();

    // game setup goes here
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarGreen);
    player1.translation.x = -500.0;
    player1.layer = 10.0;
    player1.collision = true;

    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadLine{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = -600.0 + 150.0 * i as f32;
    }

    let mut obstacles = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingCarBlack,
    ];

    for (i, preset) in obstacles.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
    }
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.3);

    game.add_logic(game_logic);
    game.run(GameState {
        health: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
    let mut direction = 0.0;
    if engine.keyboard_state.just_pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.just_pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    if player1.translation.y > 360.0 || player1.translation.y < -360.0 {
        game_state.health = 0;
        engine.should_exit = true;
    }

    for s in engine.sprites.values_mut() {
        if s.label.starts_with("roadLine") {
            s.translation.x -= ROAD_SPEED * engine.delta_f32;
            if s.translation.x < -675.0 {
                s.translation.x += 1500.0;
            }
        }
        if s.label.starts_with("obstacle") {
            s.translation.x -= ROAD_SPEED * engine.delta_f32;
            if s.translation.x < -800.0 {
                s.translation.x = thread_rng().gen_range(800.0..1600.0);
                s.translation.y = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }
}
