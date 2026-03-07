use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Agentira - Factory Simulation".into(),
                resolution: (1000., 750.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FactoryState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            collector_behavior,
            builder_behavior,
            worker_behavior,
            building_production,
            spawn_resources,
            animate_resources,
            rotate_camera,
        ))
        .run();
}

// === リソース・状態 ===

#[derive(Resource, Default)]
struct FactoryState {
    iron_collected: u32,
    products_made: u32,
    spawn_timer: f32,
}

// === コンポーネント ===

#[derive(Component, Clone, Copy, PartialEq)]
enum AgentRole {
    Collector,  // リソース収集
    Builder,    // 建設
    Worker,     // 生産
}

#[derive(Component)]
struct Agent {
    role: AgentRole,
    speed: f32,
    carrying: Option<ResourceType>,
    target: Option<Vec3>,
}

#[derive(Component, Clone, Copy)]
enum ResourceType {
    Iron,
    Product,
}

#[derive(Component)]
struct ResourceNode {
    resource_type: ResourceType,
}

#[derive(Component)]
struct Building {
    building_type: BuildingType,
    progress: f32,
    producing: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum BuildingType {
    Factory,
    Storage,
}

#[derive(Component)]
struct Bobbing { offset: f32, speed: f32 }

#[derive(Component)]
struct MainCamera;

// === スポーン関数 ===

fn spawn_agent(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    role: AgentRole,
) {
    let (color, emissive) = match role {
        AgentRole::Collector => (Color::srgb(0.2, 0.8, 0.3), LinearRgba::new(0.1, 0.4, 0.15, 1.0)),
        AgentRole::Builder => (Color::srgb(0.9, 0.6, 0.2), LinearRgba::new(0.5, 0.3, 0.1, 1.0)),
        AgentRole::Worker => (Color::srgb(0.3, 0.5, 0.9), LinearRgba::new(0.15, 0.25, 0.5, 1.0)),
    };
    
    let body_mat = materials.add(StandardMaterial { base_color: color, emissive, ..default() });
    let eye_mat = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::new(1.0, 1.0, 1.0, 1.0),
        ..default()
    });
    
    commands.spawn((
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Agent {
            role,
            speed: match role {
                AgentRole::Collector => 5.0,
                AgentRole::Builder => 3.5,
                AgentRole::Worker => 4.0,
            },
            carrying: None,
            target: None,
        },
    )).with_children(|p| {
        // 体
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.5, 0.7, 0.35))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.0, 0.55, 0.0)));
        // 頭
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.0, 1.1, 0.0)));
        // 目
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.1, 0.06, 0.03))), MeshMaterial3d(eye_mat.clone()), Transform::from_xyz(-0.08, 1.12, 0.18)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.1, 0.06, 0.03))), MeshMaterial3d(eye_mat.clone()), Transform::from_xyz(0.08, 1.12, 0.18)));
        // 腕
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.12, 0.4, 0.12))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(-0.38, 0.5, 0.0)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.12, 0.4, 0.12))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.38, 0.5, 0.0)));
        // 脚
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.14, 0.35, 0.14))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(-0.12, 0.0, 0.0)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.14, 0.35, 0.14))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.12, 0.0, 0.0)));
        
        // 役割別装飾
        match role {
            AgentRole::Collector => {
                // バックパック
                p.spawn((Mesh3d(meshes.add(Cuboid::new(0.3, 0.35, 0.2))), MeshMaterial3d(materials.add(Color::srgb(0.4, 0.3, 0.2))), Transform::from_xyz(0.0, 0.6, -0.25)));
            }
            AgentRole::Builder => {
                // ヘルメット
                p.spawn((Mesh3d(meshes.add(Cuboid::new(0.45, 0.15, 0.45))), MeshMaterial3d(materials.add(Color::srgb(1.0, 0.8, 0.0))), Transform::from_xyz(0.0, 1.35, 0.0)));
            }
            AgentRole::Worker => {
                // ツール
                p.spawn((Mesh3d(meshes.add(Cuboid::new(0.08, 0.5, 0.08))), MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))), Transform::from_xyz(0.5, 0.6, 0.0)));
            }
        }
    });
}

fn spawn_resource_node(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.6, 0.7),
            emissive: LinearRgba::new(0.3, 0.3, 0.4, 1.0),
            metallic: 0.9,
            ..default()
        })),
        Transform::from_translation(position + Vec3::Y * 0.4),
        ResourceNode { resource_type: ResourceType::Iron },
        Bobbing { offset: position.x + position.z, speed: 2.0 },
    ));
}

fn spawn_building(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    building_type: BuildingType,
) {
    let (size, color, emissive) = match building_type {
        BuildingType::Factory => (
            Vec3::new(2.0, 1.5, 2.0),
            Color::srgb(0.5, 0.4, 0.3),
            LinearRgba::new(0.2, 0.15, 0.1, 1.0),
        ),
        BuildingType::Storage => (
            Vec3::new(1.5, 1.0, 1.5),
            Color::srgb(0.3, 0.4, 0.5),
            LinearRgba::new(0.1, 0.15, 0.2, 1.0),
        ),
    };
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(materials.add(StandardMaterial { base_color: color, emissive, ..default() })),
        Transform::from_translation(position + Vec3::Y * size.y / 2.0),
        Building { building_type, progress: 100.0, producing: false },
    ));
}

// === セットアップ ===

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // カメラ
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 18.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));

    // 床
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(32.0, 32.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.12, 0.15, 0.1))),
    ));
    
    // グリッド
    for i in -16..=16 {
        let alpha = if i % 4 == 0 { 0.3 } else { 0.1 };
        let pos = i as f32;
        commands.spawn((Mesh3d(meshes.add(Cuboid::new(32.0, 0.01, 0.02))), MeshMaterial3d(materials.add(Color::srgba(0.4, 0.5, 0.3, alpha))), Transform::from_xyz(0.0, 0.01, pos)));
        commands.spawn((Mesh3d(meshes.add(Cuboid::new(0.02, 0.01, 32.0))), MeshMaterial3d(materials.add(Color::srgba(0.4, 0.5, 0.3, alpha))), Transform::from_xyz(pos, 0.01, 0.0)));
    }

    // エージェント
    // Collectors (緑)
    for i in 0..3 {
        let angle = i as f32 * 2.1;
        spawn_agent(&mut commands, &mut meshes, &mut materials, Vec3::new(angle.cos() * 3.0, 0.0, angle.sin() * 3.0), AgentRole::Collector);
    }
    // Builders (オレンジ)
    for i in 0..2 {
        let angle = i as f32 * 3.14 + 1.0;
        spawn_agent(&mut commands, &mut meshes, &mut materials, Vec3::new(angle.cos() * 4.0, 0.0, angle.sin() * 4.0), AgentRole::Builder);
    }
    // Workers (青)
    for i in 0..2 {
        let angle = i as f32 * 3.14 + 0.5;
        spawn_agent(&mut commands, &mut meshes, &mut materials, Vec3::new(angle.cos() * 2.0, 0.0, angle.sin() * 2.0), AgentRole::Worker);
    }

    // 初期リソースノード
    for x in [-10, -6, 6, 10] {
        for z in [-10, -6, 6, 10] {
            if (x + z) % 8 == 0 {
                spawn_resource_node(&mut commands, &mut meshes, &mut materials, Vec3::new(x as f32, 0.0, z as f32));
            }
        }
    }

    // 初期建物
    spawn_building(&mut commands, &mut meshes, &mut materials, Vec3::new(0.0, 0.0, -6.0), BuildingType::Factory);
    spawn_building(&mut commands, &mut meshes, &mut materials, Vec3::new(5.0, 0.0, 0.0), BuildingType::Storage);

    // 光源
    commands.spawn((DirectionalLight { illuminance: 10000.0, shadows_enabled: true, ..default() }, Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, 0.5, 0.0))));
    commands.insert_resource(AmbientLight { color: Color::srgb(0.5, 0.55, 0.6), brightness: 350.0 });
}

// === 行動システム ===

fn collector_behavior(
    time: Res<Time>,
    mut state: ResMut<FactoryState>,
    resources: Query<(Entity, &Transform), With<ResourceNode>>,
    buildings: Query<&Transform, With<Building>>,
    mut agents: Query<(&mut Transform, &mut Agent), Without<ResourceNode>>,
    mut commands: Commands,
) {
    let resource_positions: Vec<(Entity, Vec3)> = resources.iter().map(|(e, t)| (e, t.translation)).collect();
    let storage_pos = buildings.iter().next().map(|t| t.translation);
    
    for (mut transform, mut agent) in &mut agents {
        if agent.role != AgentRole::Collector { continue; }
        
        if agent.carrying.is_some() {
            // 運搬中 → ストレージへ
            if let Some(storage) = storage_pos {
                let dir = (storage - transform.translation).normalize_or_zero();
                let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                transform.translation += dir_xz * agent.speed * time.delta_secs();
                if dir_xz.length() > 0.01 { transform.look_at(transform.translation + dir_xz, Vec3::Y); }
                
                if transform.translation.distance(storage) < 2.0 {
                    agent.carrying = None;
                    state.iron_collected += 1;
                }
            }
        } else {
            // 収集 → 最寄りリソースへ
            if let Some((entity, res_pos)) = resource_positions.iter().min_by(|a, b| {
                transform.translation.distance(a.1).partial_cmp(&transform.translation.distance(b.1)).unwrap()
            }) {
                let dir = (*res_pos - transform.translation).normalize_or_zero();
                let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                transform.translation += dir_xz * agent.speed * time.delta_secs();
                if dir_xz.length() > 0.01 { transform.look_at(transform.translation + dir_xz, Vec3::Y); }
                
                if transform.translation.distance(*res_pos) < 0.8 {
                    agent.carrying = Some(ResourceType::Iron);
                    commands.entity(*entity).despawn();
                }
            }
        }
        
        transform.translation.x = transform.translation.x.clamp(-14.0, 14.0);
        transform.translation.z = transform.translation.z.clamp(-14.0, 14.0);
    }
}

fn builder_behavior(
    time: Res<Time>,
    mut agents: Query<(&mut Transform, &Agent)>,
) {
    for (mut transform, agent) in &mut agents {
        if agent.role != AgentRole::Builder { continue; }
        
        // ランダム巡回
        let t = time.elapsed_secs();
        let target = Vec3::new((t * 0.5).sin() * 8.0, 0.0, (t * 0.3).cos() * 8.0);
        let dir = (target - transform.translation).normalize_or_zero();
        let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
        transform.translation += dir_xz * agent.speed * time.delta_secs() * 0.5;
        if dir_xz.length() > 0.01 { transform.look_at(transform.translation + dir_xz, Vec3::Y); }
        
        transform.translation.x = transform.translation.x.clamp(-14.0, 14.0);
        transform.translation.z = transform.translation.z.clamp(-14.0, 14.0);
    }
}

fn worker_behavior(
    time: Res<Time>,
    buildings: Query<&Transform, With<Building>>,
    mut agents: Query<(&mut Transform, &Agent), Without<Building>>,
) {
    let factory_pos = buildings.iter().next().map(|t| t.translation);
    
    for (mut transform, agent) in &mut agents {
        if agent.role != AgentRole::Worker { continue; }
        
        if let Some(factory) = factory_pos {
            let dist = transform.translation.distance(factory);
            if dist > 3.0 {
                let dir = (factory - transform.translation).normalize_or_zero();
                let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                transform.translation += dir_xz * agent.speed * time.delta_secs();
                if dir_xz.length() > 0.01 { transform.look_at(transform.translation + dir_xz, Vec3::Y); }
            } else {
                // 工場周辺で作業
                let t = time.elapsed_secs();
                let orbit = Vec3::new((t * 1.5).sin() * 2.5, 0.0, (t * 1.5).cos() * 2.5);
                let target = factory + orbit;
                let dir = (target - transform.translation).normalize_or_zero();
                transform.translation += dir * agent.speed * time.delta_secs() * 0.3;
            }
        }
    }
}

fn building_production(
    time: Res<Time>,
    mut state: ResMut<FactoryState>,
    mut buildings: Query<&mut Building>,
) {
    for mut building in &mut buildings {
        if building.building_type == BuildingType::Factory && state.iron_collected > 0 {
            building.producing = true;
            building.progress += time.delta_secs() * 20.0;
            if building.progress >= 100.0 {
                building.progress = 0.0;
                state.iron_collected = state.iron_collected.saturating_sub(1);
                state.products_made += 1;
            }
        }
    }
}

fn spawn_resources(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<FactoryState>,
    time: Res<Time>,
    resources: Query<&ResourceNode>,
) {
    state.spawn_timer += time.delta_secs();
    
    if state.spawn_timer > 2.0 && resources.iter().count() < 12 {
        state.spawn_timer = 0.0;
        let x = (time.elapsed_secs() * 3.7).sin() * 11.0;
        let z = (time.elapsed_secs() * 2.3).cos() * 11.0;
        spawn_resource_node(&mut commands, &mut meshes, &mut materials, Vec3::new(x, 0.0, z));
    }
}

fn animate_resources(
    time: Res<Time>,
    mut resources: Query<(&mut Transform, &Bobbing), With<ResourceNode>>,
) {
    for (mut transform, bobbing) in &mut resources {
        transform.translation.y = 0.4 + ((time.elapsed_secs() * bobbing.speed + bobbing.offset).sin() * 0.15);
        transform.rotate_y(time.delta_secs() * 1.5);
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in &mut query {
        let angle = time.elapsed_secs() * 0.05;
        let radius = 28.0;
        let height = 20.0;
        transform.translation = Vec3::new(angle.sin() * radius, height, angle.cos() * radius);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}
