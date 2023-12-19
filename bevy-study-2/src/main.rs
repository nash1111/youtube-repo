// original source:
// https://bevyengine.org/examples/3D%20Rendering/3d-scene/
// in this video, modify and play with this code:


use bevy::{prelude::*, input::keyboard, gizmos};

#[derive(Resource, Debug)]
struct Cubesize {
    size: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system, update))
        .run();
}

fn system(mut gizmos: Gizmos){

}

fn update(keyboard: Res<Input<KeyCode>>, mut cubesize: ResMut<Cubesize>, gizmos: Gizmos,mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,mut materials: ResMut<Assets<StandardMaterial>>) {
    println!("update");
    if keyboard.pressed(KeyCode::Up) {
        cubesize.size += 0.1;
    }
    if keyboard.pressed(KeyCode::Down) {
        cubesize.size -= 0.1;
    }
    println!("cubesize: {}", cubesize.size);
    let color = 
    // generate color random
    Color::rgb(
        rand::random::<f32>(),
        rand::random::<f32>(),
        rand::random::<f32>(),
    );
    let random_angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: cubesize.size })),
        material: materials.add(color.into()),
        transform: Transform::from_rotation(Quat::from_rotation_arc(Vec3::X, Vec3::Y)),
        ..default()
    });
    //let random_angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: cubesize.size })),
    //    material: materials.add(color.into()),
    //    transform: Transform::from_rotation(Quat::from_rotation_y(random_angle)),
    //    ..default()
    //});
    //let random_angle = rand::random::<f32>() * 2.0 * std::f32::consts::PI;
    //commands.spawn(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: cubesize.size })),
    //    material: materials.add(color.into()),
    //    transform: Transform::from_rotation(Quat::from_rotation_z(random_angle)),
    //    ..default()
    //});
    cubesize.size += 0.01;

}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // default cube size
    commands.insert_resource(Cubesize { size: 1.0 })
}