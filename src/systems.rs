use crate::components::MainGameCamera;
use crate::resources::ShowFps;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};

const CLEAR_COLOR: Color = Color::rgb(0.035, 0., 0.070);
const VIEWPORT_RATIO_WIDTH: f32 = 16.;
const VIEWPORT_RATIO_HEIGHT: f32 = 9.;

const RESOLUTION: (f32, f32) = (1280., 720.);

pub const VIEWPORT_RELATIVE_WIDTH: usize = 32;
pub const VIEWPORT_RELATIVE_HEIGHT: usize = 18;

const SUBUNITS_PER_UNIT: f32 = 20.;
const WIDTH_IN_SUBUNITS: f32 = VIEWPORT_RELATIVE_WIDTH as f32 * SUBUNITS_PER_UNIT;
const HEIGHT_IN_SUBUNITS: f32 = VIEWPORT_RELATIVE_HEIGHT as f32 * SUBUNITS_PER_UNIT;

pub fn setup(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();

    window.title = "Falling Sand".into();
    window.present_mode = PresentMode::AutoNoVsync;
    window.mode = WindowMode::Windowed;
    // GET CLOSEST PIXELPERFECT RESOLUTION TO RESOLUTION
    window.resolution = RESOLUTION.into();
    window.resolution = (
        (RESOLUTION.0 / WIDTH_IN_SUBUNITS).round() * WIDTH_IN_SUBUNITS,
        (RESOLUTION.1 / HEIGHT_IN_SUBUNITS).round() * HEIGHT_IN_SUBUNITS,
    )
        .into();

    commands.insert_resource(ShowFps(true));

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(VIEWPORT_RATIO_WIDTH, VIEWPORT_RATIO_HEIGHT, 999.),
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(CLEAR_COLOR),
            },
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                    width: VIEWPORT_RELATIVE_WIDTH as f32,
                    height: VIEWPORT_RELATIVE_HEIGHT as f32,
                },
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
