use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Agentira - Prototype".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, move_agent)
        .run();
}

#[derive(Component)]
struct Agent {
    speed: f32,
    direction: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラ
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 床
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // エージェント (ピクセル風のキューブ)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Agent {
            speed: 3.0,
            direction: Vec3::new(1.0, 0.0, 0.5).normalize(),
        },
    ));

    // 光源
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));
}

fn move_agent(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Agent)>,
) {
    for (mut transform, mut agent) in &mut query {
        // 移動
        transform.translation += agent.direction * agent.speed * time.delta_secs();

        // 境界で反射
        if transform.translation.x.abs() > 8.0 {
            agent.direction.x *= -1.0;
        }
        if transform.translation.z.abs() > 8.0 {
            agent.direction.z *= -1.0;
        }
    }
}
