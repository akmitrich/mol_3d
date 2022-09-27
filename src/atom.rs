use bevy::prelude::*;
use d_vector::Real;
use mol_job::lennard_jones::LennardJones;

use crate::{
    resources::{Acc, Pos, Settings, TimeNow, Vel, Wrapper},
    SCALE_X, SCALE_Y, SCALE_Z,
};

pub struct AtomPlugin;

impl Plugin for AtomPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, setup_system)
            .add_system(single_step)
            .add_system(update)
            .add_system(check_vel);
    }
}

#[derive(Component)]
struct Atom;

#[derive(Component)]
struct Index(pub usize);

fn setup_system(
    mut commands: Commands,
    pos: Res<Pos>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (index, atom) in pos.0.iter().enumerate() {
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.25, ..Default::default() })),
                material: materials.add(Color::GREEN.into()),
                transform: Transform::from_translation(make_translation(atom.components())),
                ..default()
            })
            .insert(Atom)
            .insert(Index(index));
    }
}

fn single_step(
    mut pos: ResMut<Pos>,
    mut vel: ResMut<Vel>,
    mut acc: ResMut<Acc>,
    boundaries: Res<Wrapper>,
    mut time: ResMut<TimeNow>,
    settings: Res<Settings>,
) {
    let delta_t = settings.delta_t;
    let potential_energy = LennardJones::default();
    mol_job::verlet::single_step(
        delta_t,
        &mut pos.0,
        &mut vel.0,
        &mut acc.0,
        &boundaries.0,
        &potential_energy,
    );
    time.0 += delta_t;
}

fn update(pos: Res<Pos>, mut query: Query<(&mut Transform, &Index), With<Atom>>) {
    for (mut atom_transform, atom_index) in query.iter_mut() {
        atom_transform.translation =
            make_translation(pos.0.get(atom_index.0).unwrap().components());
    }
}

fn check_vel(vel: Res<Vel>, settings: Res<Settings>, boundaries: Res<Wrapper>, time: Res<TimeNow>) {
    for vel in vel.0.iter() {
        for (v_component, dimension) in vel
            .components()
            .iter()
            .zip(boundaries.0.dimensions().iter())
        {
            if (v_component * (settings.delta_t as Real)) > *dimension {
                println!("System explode at time = {}.", time.0);
                panic!("Please consider making DELTA_T smaller.");
            }
        }
    }
}

fn make_translation(components: &[Real; 3]) -> Vec3 {
    Vec3::new(
        components[0] * SCALE_X,
        components[1] * SCALE_Y,
        components[2] * SCALE_Z,
    )
}
