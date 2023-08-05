use bevy::{
    prelude::*,
    window::PresentMode, render::{camera::ScalingMode, texture::ImageSampler}
};
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "This is window, hello hi".to_string(),
                        resolution: (1024., 768.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                }
            ),
            LdtkPlugin))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false
            },
            set_clear_color: SetClearColor::No,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, tilemap_sampler_fix)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(96.);
    camera.transform.translation += Vec3 {x: 64., y: -48., z: 0.};

    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("worldmap.ldtk"),
        ..Default::default()
    });
}

fn tilemap_sampler_fix(
    mut ev_asset: EventReader<AssetEvent<Image>>, 
    mut assets: ResMut<Assets<Image>>
) {
    for ev in ev_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(texture) = assets.get_mut(&handle) {
                    info!("Setting texture sampling to nearest");
                    texture.sampler_descriptor = ImageSampler::nearest();
                }
            },
            _ => {}
        }
    }
}