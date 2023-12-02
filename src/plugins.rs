use bevy::prelude::*;

pub struct CustomDefaultPlugin;

impl Plugin for CustomDefaultPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }), // .set(ImagePlugin::default_nearest()),
        );
    }
}
