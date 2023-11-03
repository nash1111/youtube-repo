use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).add_systems(Update, (system, update_config)).run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        ..Default::default()
    })
    ;
}

fn system(mut gizmos: Gizmos) {
    gizmos.linestrip_gradient_2d([
        (Vec2::Y*300., Color::BLUE),
        (Vec2::new(-255.,-155.), Color::RED),
        (Vec2::new(255.,-155.), Color::GREEN),
        (Vec2::Y*300., Color::BLUE),
    ])
}

fn update_config(mut config: ResMut<GizmoConfig>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    if keyboard.pressed(KeyCode::Right) {
        config.line_width += 300. * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        config.line_width -= 300. * time.delta_seconds();
    }
}