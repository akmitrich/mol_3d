use bevy::prelude::*;

mod resources;
mod atom;

const N_MOL: &str = "N_MOL";
const DENSITY: &str = "DENSITY";
const DELTA_T: &str = "DELTA_T";
const TAU: &str = "TAU";
const SCALE_X: f32 = 1.;
const SCALE_Y: f32 = 1.;
const SCALE_Z: f32 = 1.;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(atom::AtomPlugin)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
) {
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    resources::init(&mut commands);
}