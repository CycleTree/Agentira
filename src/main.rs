use bevy::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Agentira - AI Chase Game".into(),
                resolution: (900., 700.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            chase_behavior,
            collect_resources,
            spawn_resources,
            animate_resources,
            update_score_display,
            rotate_camera,
        ))
        .run();
}

#[derive(Resource, Default)]
struct GameState {
    scores: [u32; 5],
    spawn_timer: f32,
}

#[derive(Component)]
struct Agent {
    id: usize,
    speed: f32,
    target: Option<Vec3>,
}

#[derive(Component)]
struct Resource {
    value: u32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Bobbing {
    offset: f32,
    speed: f32,
}

fn spawn_voxel_agent(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    color: Color,
    speed: f32,
    id: usize,
) {
    let body_material = materials.add(StandardMaterial {
        base_color: color,
        emissive: color.into(),
        ..default()
    });
    let dark_material = materials.add(Color::srgb(0.05, 0.05, 0.05));
    let eye_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: (Color::WHITE * 2.0).into(),
        ..default()
    });
    
    commands.spawn((
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Agent {
            id,
            speed,
            target: None,
        },
    )).with_children(|parent| {
        // 体
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
        
        // アンテナ
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.05, 0.3, 0.05))),
            MeshMaterial3d(dark_material.clone()),
            Transform::from_xyz(0.0, 1.65, 0.0),
        ));
        
        // アンテナ先端（光る）
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.08))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(0.0, 1.85, 0.0),
        ));
        
        // 目（左）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, 0.08, 0.05))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(-0.12, 1.28, 0.23),
        ));
        
        // 目（右）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, 0.08, 0.05))),
            MeshMaterial3d(eye_material.clone()),
            Transform::from_xyz(0.12, 1.28, 0.23),
        ));
        
        // 腕
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.15, 0.5, 0.15))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(-0.45, 0.5, 0.0),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.15, 0.5, 0.15))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.45, 0.5, 0.0),
        ));
        
        // 脚
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.18, 0.4, 0.18))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(-0.15, 0.0, 0.0),
        ));
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.18, 0.4, 0.18))),
            MeshMaterial3d(body_material.clone()),
            Transform::from_xyz(0.15, 0.0, 0.0),
        ));
    });
}

fn spawn_resource(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let colors = [
        Color::srgb(1.0, 0.8, 0.2),  // 金
        Color::srgb(0.2, 1.0, 0.8),  // シアン
        Color::srgb(1.0, 0.4, 0.8),  // ピンク
    ];
    let color = colors[(position.x.abs() as usize + position.z.abs() as usize) % colors.len()];
    
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            emissive: (color * 3.0).into(),
            ..default()
        })),
        Transform::from_translation(position + Vec3::Y * 0.5),
        Resource { value: 10 },
        Bobbing {
            offset: position.x + position.z,
            speed: 3.0,
        },
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラ
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 22.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));

    // 床
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(28.0, 28.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.08, 0.12, 0.15),
            ..default()
        })),
    ));
    
    // グリッド
    for i in -14..=14 {
        let alpha = if i % 5 == 0 { 0.4 } else { 0.15 };
        let pos = i as f32;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(28.0, 0.01, 0.02))),
            MeshMaterial3d(materials.add(Color::srgba(0.3, 0.5, 0.6, alpha))),
            Transform::from_xyz(0.0, 0.01, pos),
        ));
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.02, 0.01, 28.0))),
            MeshMaterial3d(materials.add(Color::srgba(0.3, 0.5, 0.6, alpha))),
            Transform::from_xyz(pos, 0.01, 0.0),
        ));
    }

    // エージェント
    let configs = [
        (Vec3::new(0.0, 0.0, 0.0), Color::srgb(1.0, 0.3, 0.3), 4.0),
        (Vec3::new(6.0, 0.0, 4.0), Color::srgb(0.3, 1.0, 0.4), 4.5),
        (Vec3::new(-5.0, 0.0, -3.0), Color::srgb(0.3, 0.4, 1.0), 3.8),
        (Vec3::new(4.0, 0.0, -6.0), Color::srgb(1.0, 1.0, 0.3), 5.0),
        (Vec3::new(-7.0, 0.0, 5.0), Color::srgb(1.0, 0.3, 1.0), 4.2),
    ];
    
    for (i, (pos, color, speed)) in configs.iter().enumerate() {
        spawn_voxel_agent(&mut commands, &mut meshes, &mut materials, *pos, *color, *speed, i);
    }

    // 初期リソース
    for x in [-8, -4, 0, 4, 8] {
        for z in [-8, -4, 0, 4, 8] {
            if (x + z) % 4 == 0 {
                spawn_resource(&mut commands, &mut meshes, &mut materials, Vec3::new(x as f32, 0.0, z as f32));
            }
        }
    }

    // 光源
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.5, 0.0)),
    ));
    
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.4, 0.5, 0.7),
        brightness: 300.0,
    });
}

fn chase_behavior(
    time: Res<Time>,
    resources: Query<&Transform, (With<Resource>, Without<Agent>)>,
    mut agents: Query<(&mut Transform, &mut Agent)>,
) {
    let resource_positions: Vec<Vec3> = resources.iter().map(|t| t.translation).collect();
    
    for (mut transform, mut agent) in &mut agents {
        // 最も近いリソースを探す
        let mut closest: Option<Vec3> = None;
        let mut closest_dist = f32::MAX;
        
        for &res_pos in &resource_positions {
            let dist = transform.translation.distance(res_pos);
            if dist < closest_dist {
                closest_dist = dist;
                closest = Some(res_pos);
            }
        }
        
        agent.target = closest;
        
        // ターゲットに向かって移動
        if let Some(target) = agent.target {
            let direction = (target - transform.translation).normalize_or_zero();
            let direction_xz = Vec3::new(direction.x, 0.0, direction.z).normalize_or_zero();
            
            transform.translation += direction_xz * agent.speed * time.delta_secs();
            
            if direction_xz.length() > 0.01 {
                let target_pos = transform.translation + direction_xz;
                transform.look_at(target_pos, Vec3::Y);
            }
        }
        
        // 境界制限
        transform.translation.x = transform.translation.x.clamp(-12.0, 12.0);
        transform.translation.z = transform.translation.z.clamp(-12.0, 12.0);
    }
}

fn collect_resources(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    agents: Query<(&Transform, &Agent)>,
    resources: Query<(Entity, &Transform, &Resource)>,
) {
    for (agent_transform, agent) in &agents {
        for (entity, res_transform, resource) in &resources {
            let dist = agent_transform.translation.distance(res_transform.translation);
            if dist < 0.8 {
                game_state.scores[agent.id] += resource.value;
                commands.entity(entity).despawn();
            }
        }
    }
}

fn spawn_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    resources: Query<&Resource>,
) {
    game_state.spawn_timer += time.delta_secs();
    
    if game_state.spawn_timer > 1.5 && resources.iter().count() < 15 {
        game_state.spawn_timer = 0.0;
        let x = (time.elapsed_secs() * 7.3).sin() * 10.0;
        let z = (time.elapsed_secs() * 5.7).cos() * 10.0;
        spawn_resource(&mut commands, &mut meshes, &mut materials, Vec3::new(x, 0.0, z));
    }
}

fn animate_resources(
    time: Res<Time>,
    mut resources: Query<(&mut Transform, &Bobbing), With<Resource>>,
) {
    for (mut transform, bobbing) in &mut resources {
        transform.translation.y = 0.5 + ((time.elapsed_secs() * bobbing.speed + bobbing.offset).sin() * 0.2);
        transform.rotate_y(time.delta_secs() * 2.0);
    }
}

fn update_score_display(
    game_state: Res<GameState>,
) {
    // スコア表示（デバッグ用、後でUIに変更）
    // println!("Scores: {:?}", game_state.scores);
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in &mut query {
        let angle = time.elapsed_secs() * 0.08;
        let radius = 24.0;
        let height = 16.0;
        transform.translation = Vec3::new(
            angle.sin() * radius,
            height,
            angle.cos() * radius,
        );
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
