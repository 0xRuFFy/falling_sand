use crate::components::MainGameCamera;
use crate::resources::ShowFps;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};

pub const PIXELS_PER_UNIT: f32 = 16.0;
const CLEAR_COLOR: Color = Color::rgb(0.035, 0., 0.070);
const RESOLUTION: (f32, f32) = (1280., 720.);

pub fn setup(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    window.title = "Falling Sand".into();
    window.present_mode = PresentMode::AutoNoVsync;
    window.mode = WindowMode::Windowed;
    // GET CLOSEST PIXELPERFECT RESOLUTION TO RESOLUTION
    window.resolution = RESOLUTION.into();

    commands.insert_resource(ShowFps(true));

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                window.width() / (2.0 * PIXELS_PER_UNIT),
                window.height() / (2.0 * PIXELS_PER_UNIT),
                999.0,
            ),
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
            },
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::WindowSize(PIXELS_PER_UNIT),
                ..default()
            },
            ..default()
        },
        MainGameCamera,
    ));
}

pub fn testing_exit(mut exit: EventWriter<bevy::app::AppExit>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        println!("ESC pressed, exiting...");
        exit.send(bevy::app::AppExit);
    }
}
