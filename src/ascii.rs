use bevy:: prelude::*;

use crate::TILE_SIZE;
pub struct AsciiPlugin;
struct AsciiSheet(Handle<TextureAtlas>);

impl Plugin for AsciiPlugin {
    fn build(&self, app: &mut App){
        app
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);

    }
}

fn spawn_ascii_sprite(
    commands: &mut Commands,
    ascii : &Res<AsciiSheet>,
    index: usize,
    color: Color,
    translation: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    sprite.color = color;

    commands
        .spawn_bundle(
            SpriteSheetBundle {
                sprite: sprite.clone(),
                texture_atlas: ascii.0.clone(),
                transform: Transform {
                    translation: translation,
                    ..Default::default()
                }, ..Default::default()
            }
        ).id()
    }
    
fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
        let image = assets.load("Ascii.png");
        let atlas = TextureAtlas::from_grid_with_padding(
            image,
            Vec2::splat(9.0),
            16,
            16,
            Vec2::splat(2.0)
            );
        //note: sprite sheet ascii with padding.
        let atlas_handle = texture_atlases.add(atlas);
        commands.insert_resource(AsciiSheet(atlas_handle));

}
