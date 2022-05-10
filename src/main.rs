#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};
//use bevy::DefaultPlugins;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECTRATIO: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod player;
mod debug;

use player::PlayerPlugin; //contained in file: player.rs
use debug::DebugPlugin;
fn main() {
    /*
     * Overview: of sprite loading using atlas
     * Sequence: 1) Load Image,
     *           2) Define image as texture atlas,
     *           3) Place the atlas in an accessible place
     *           4) keep a reference using a custom resource
     */
    let height: f32 = 900.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * ASPECTRATIO,
            height: height,
            title: "Bevy Tutorial".to_string(),
            resizable: true,
            cursor_locked: false,
            cursor_visible: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugins(DebugPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera: OrthographicCameraBundle = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * ASPECTRATIO;
    camera.orthographic_projection.left = -1.0 * ASPECTRATIO;

    camera.orthographic_projection.scaling_mode = ScalingMode::None; //pixel art

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

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
