use bevy::prelude::*;

// in this video, we will be creating a simple 2d gizmo system
// and learn to how to insert resources into the system

#[derive(Resource, Debug)]
struct Circle2DResource {
    center: Vec2,
    radius: f32,
    color: Color,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system, update_config))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(TextBundle::from_section(
        "", // insert text you like
        TextStyle {
            font_size: 30.,
            color: Color::WHITE,
            ..Default::default()
        },
    ));
    commands.insert_resource(Circle2DResource {
        center: Vec2 { x: 0.0, y: 0.0 },
        radius: 20.0,
        color: Color::BLUE,
    });
}

fn system(mut gizmos: Gizmos) {
    //gizmos.circle_2d(Vec2 { x: 0.0, y: 0.0 }, 10.0, Color::WHITE);
}

fn update_config(
    mut gizmos: Gizmos,
    mut config: ResMut<GizmoConfig>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut circle_2: ResMut<Circle2DResource>,
) {
    if keyboard.pressed(KeyCode::Left) {
        gizmos.circle_2d(Vec2 { x: 0.0, y: 0.0 }, 50.0, Color::RED);
    }

    if keyboard.pressed(KeyCode::Right) {
        gizmos.circle_2d(Vec2 { x: 0.0, y: 0.0 }, 60.0, Color::GREEN);
    }

    if keyboard.pressed(KeyCode::Up) {
        gizmos.circle_2d(Vec2 { x: 0.0, y: 0.0 }, 70.0, Color::BLUE);
    }

    if keyboard.pressed(KeyCode::Down) {
        // when pressed down, circle goes down

        gizmos.circle_2d(circle_2.center, circle_2.radius, circle_2.color);
    }
    circle_2.radius += time.delta_seconds() * 100.0;
    // choice random color
    for _ in 0..10 {
        gizmos.circle_2d(
            Vec2 { x: 0.0, y: 0.0 },
            // random but between 0 and 100
            rand::random::<f32>() * 1000.0 + circle_2.radius,
            Color::rgba(
                rand::random::<f32>(),
                rand::random::<f32>(),
                rand::random::<f32>(),
                // rand between 0,1
                rand::random::<f32>(),
            ),
        );
    }
}
