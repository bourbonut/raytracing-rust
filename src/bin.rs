mod bevy_utils;
mod utils;

use bevy::prelude::*;
// use bevy_utils::resources::BevyCamera;
// use bevy_utils::resources::BevyLight;
// use bevy_utils::raytracing::compute_rt;
use bevy_utils::resources::Materials;
use bevy_utils::resources::Meshes;
// use bevy_utils::spawns::*;
use raytracing::run_lib;

fn main() {
    run_lib();
    // App::new()
    //     .insert_resource(WindowDescriptor {
    //         title: "Raytracing app".to_string(),
    //         width: 500.0,
    //         height: 300.0,
    //         ..Default::default()
    //     })
    //     .add_plugins(DefaultPlugins)
    //     .add_startup_system(setup)
    //     .add_startup_stage("camera_spawn_stage", SystemStage::single(camera_spawn))
    //     .add_startup_stage("light_spawn_stage", SystemStage::single(light_spawn))
    //     .add_startup_stage("plane_spawn_stage", SystemStage::single(plane_spawn))
    //     .add_startup_stage("sphere_spawn_stage", SystemStage::single(sphere_spawn))
    //     .add_system(compute_rt)
    //     .run();
}
/// set up a simple 3D scene
fn _setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //We add the ressources "Materials" type we made so bevy knows them
    commands.insert_resource(Materials {
        sphere_material_red: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
        sphere_material_green: materials.add(Color::rgb(0.1, 0.8, 0.1).into()),
        sphere_material_blue: materials.add(Color::rgb(0.1, 0.1, 0.8).into()),
        plane_material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    });

    //Same for the meshings
    commands.insert_resource(Meshes {
        sphere_mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
        plane_mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    });
}
