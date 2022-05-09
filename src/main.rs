use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    /*
     * Overview: of sprite loading using atlas
     * Sequence: 1) Load Image,
     *           2) Define image as texture atlas,
     *           3) Place the atlas in an accessible place
     *           4) keep a reference using a custom resource
     */
    let _height: f32 = 900.0;

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Bevy Tutorial".to_string(),
            resizable: true,
            cursor_locked: false,
            cursor_visible: true,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        //.add_startup_system(spawn_background)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    {
        let mut sprite = TextureAtlasSprite::new(1);
        sprite.color = Color::rgb(0.3, 0.3, 0.9);
        sprite.custom_size = Some(Vec2::splat(1.0));
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
            .id();

    let mut background_sprite = TextureAtlasSprite::new(0);
    background_sprite.color = Color::rgb(0.5,0.5,0.5);
    background_sprite.custom_size = Some(Vec2::splat(1.0));
    let background = commands
        .spawn_bundle(
            SpriteSheetBundle{
                    sprite: background_sprite,
                    texture_atlas: ascii.0.clone(),
                    transform: Transform { 
                        translation: Vec3::new(0.0,0.0,-1.0),
                        ..Default::default()
                    },
                    ..Default::default()
            }
        )
        .insert(Name::new("Background"))
        .id(); //gets entity after creation
    commands.entity(player).push_children(&[background]);
}
}

fn spawn_background(mut commands: Commands, ascii: Res<AsciiSheet>) {
    {
        let mut background = TextureAtlasSprite::new(3);
        background.color = Color::rgb(0.5, 0.5, 0.5);
        background.custom_size = Some(Vec2::splat(1.5));
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: background,
                texture_atlas: ascii.0.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Background"));
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera: OrthographicCameraBundle = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

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
