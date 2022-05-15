#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::ScalingMode};
//use bevy::window::WindowMode;

pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const TILE_SIZE: f32 = 0.1;

#[derive(Component)]
pub struct AsciiSheet(pub Handle<TextureAtlas>);

mod player;
mod debug;
mod ascii;

use player::PlayerPlugin; //contained in file: player.rs
use debug::DebugPlugin;
use ascii::AsciiPlugin;
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
            width: height * ASPECT_RATIO,
            height: height,
            title: "Bevy Tutorial".to_string(),
            resizable: true,
            cursor_locked: false,
            cursor_visible: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AsciiPlugin)
        .add_plugin(DebugPlugin)
        //.add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera: OrthographicCameraBundle = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * ASPECT_RATIO;
    camera.orthographic_projection.left = -1.0 * ASPECT_RATIO;

    camera.orthographic_projection.scaling_mode = ScalingMode::None; //pixel art

    commands.spawn_bundle(camera);
}