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
        .add_systems(Update, (move_agent, rotate_camera))
        .run();
}

#[derive(Component)]
struct Agent {
    speed: f32,
    direction: Vec3,
}

#[derive(Component)]
struct MainCamera;

// ボクセル風エージェントをスポーン
fn spawn_voxel_agent(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    color: Color,
    speed: f32,
) {
    let body_material = materials.add(color);
    let dark_material = materials.add(Color::srgb(0.1, 0.1, 0.1));
    
    // 親エンティティ（エージェント本体）
    commands.spawn((
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Agent {
            speed,
            direction: Vec3::new(
                (position.x * 0.1).sin(),
                0.0,
                (position.z * 0.1).cos(),
            ).normalize(),
        },
    )).with_children(|parent| {
        // 体 (メイン)
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.6, 0.8, 0.4))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, 0.6, 0.0),
        ));
        
        // 頭
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.0, 1.25, 0.0),
        ));
        
        // 目（左）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
            MeshMaterial3d(dark_material.clone()),
            Transform::from_xyz(-0.12, 1.3, 0.21),
        ));
        
        // 目（右）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
            MeshMaterial3d(dark_material.clone()),
            Transform::from_xyz(0.12, 1.3, 0.21),
        ));
        
        // 左腕
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.6, 0.2))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(-0.5, 0.5, 0.0),
        ));
        
        // 右腕
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.6, 0.2))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.5, 0.5, 0.0),
        ));
        
        // 左脚
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.5, 0.2))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(-0.15, 0.0, 0.0),
        ));
        
        // 右脚
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.5, 0.2))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.15, 0.0, 0.0),
        ));
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラ
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 12.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));

    // 床（グリッド風）
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(24.0, 24.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.25, 0.2))),
    ));
    
    // グリッド線
    for i in -12..=12 {
        let pos = i as f32;
        // X方向の線
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(24.0, 0.02, 0.02))),
            MeshMaterial3d(materials.add(Color::srgba(0.3, 0.4, 0.3, 0.5))),
            Transform::from_xyz(0.0, 0.01, pos),
        ));
        // Z方向の線
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.02, 0.02, 24.0))),
            MeshMaterial3d(materials.add(Color::srgba(0.3, 0.4, 0.3, 0.5))),
            Transform::from_xyz(pos, 0.01, 0.0),
        ));
    }

    // 複数のエージェントをスポーン
    let agent_configs = [
        (Vec3::new(0.0, 0.0, 0.0), Color::srgb(0.9, 0.3, 0.3), 2.5),   // 赤
        (Vec3::new(5.0, 0.0, 3.0), Color::srgb(0.3, 0.9, 0.3), 3.0),   // 緑
        (Vec3::new(-4.0, 0.0, -2.0), Color::srgb(0.3, 0.3, 0.9), 2.0), // 青
        (Vec3::new(3.0, 0.0, -5.0), Color::srgb(0.9, 0.9, 0.3), 3.5),  // 黄
        (Vec3::new(-6.0, 0.0, 4.0), Color::srgb(0.9, 0.3, 0.9), 2.8),  // マゼンタ
    ];
    
    for (pos, color, speed) in agent_configs {
        spawn_voxel_agent(&mut commands, &mut meshes, &mut materials, pos, color, speed);
    }

    // 光源
    commands.spawn((
        DirectionalLight {
            illuminance: 15000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
    
    // 環境光
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
}

fn move_agent(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Agent)>,
) {
    for (mut transform, mut agent) in &mut query {
        // 移動
        transform.translation += agent.direction * agent.speed * time.delta_secs();
        
        // 進行方向を向く
        if agent.direction.length() > 0.01 {
            let target = transform.translation + agent.direction;
            transform.look_at(target, Vec3::Y);
        }

        // 境界で反射
        if transform.translation.x.abs() > 10.0 {
            agent.direction.x *= -1.0;
        }
        if transform.translation.z.abs() > 10.0 {
            agent.direction.z *= -1.0;
        }
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in &mut query {
        // カメラをゆっくり回転
        let angle = time.elapsed_secs() * 0.1;
        let radius = 20.0;
        let height = 12.0;
        transform.translation = Vec3::new(
            angle.sin() * radius,
            height,
            angle.cos() * radius,
        );
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
