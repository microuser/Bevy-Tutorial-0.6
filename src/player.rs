use bevy::prelude::*;

use crate::AsciiSheet;
use crate::TILE_SIZE;

#[derive(Component)]
 pub struct Player{
    speed: f32
}

#[derive(Component, Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_movement);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    {
        let mut sprite = TextureAtlasSprite::new(1);
        sprite.color = Color::rgb(0.3, 0.3, 0.9);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let player = commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: sprite,
                texture_atlas: ascii.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 900.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Player"))
            .insert(Player{
                speed: 3.0
            })
            .id();

        let mut background_sprite = TextureAtlasSprite::new(0);
        background_sprite.color = Color::rgb(0.5, 0.5, 0.5);
        background_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let background = commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: background_sprite,
                texture_atlas: ascii.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Background"))
            .id(); //gets entity after creation
        commands.entity(player).push_children(&[background]);
    }
}

fn player_movement(
    //#observe the tuple parenthesis nested in the query
    mut player_query: Query<(&Player, &mut Transform)>,

    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
){
    //WARNING, dont let this get to zero: Query Single Error? self.get_single_mut().unwrap()
    //Constant speed is achieved using bevy's delta_seconds()
    //TILE_SIZE allows scaling speed relative to world.

    let ( player, mut transform) = player_query.single_mut();
    if keyboard.pressed(KeyCode::W){
        transform.translation.y += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S){
        transform.translation.y -= player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A){
        transform.translation.x += player.speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D){
        transform.translation.x -= player.speed * TILE_SIZE * time.delta_seconds();
    }
}
