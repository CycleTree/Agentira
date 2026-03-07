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
            agent_behavior,
            building_production,
            spawn_resources,
            animate_resources,
            update_trails,
            update_trail_particles,
            update_effect_emitters,
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

#[derive(Component)]
struct Trail {
    positions: Vec<(Vec3, f32)>, // (位置, 寿命)
    max_length: usize,
    spawn_timer: f32,
}

impl Default for Trail {
    fn default() -> Self {
        Self {
            positions: Vec::new(),
            max_length: 15,
            spawn_timer: 0.0,
        }
    }
}

#[derive(Component)]
struct TrailParticle {
    lifetime: f32,
    max_lifetime: f32,
}

#[derive(Component)]
struct EffectEmitter {
    timer: f32,
    interval: f32,
}

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
        Trail::default(),
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
    // メインリソース球体 - 強力な発光
    let main_entity = commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.9, 1.0),
            emissive: LinearRgba::new(0.6, 0.8, 1.2, 1.0),
            metallic: 0.95,
            perceptual_roughness: 0.05,
            ..default()
        })),
        Transform::from_translation(position + Vec3::Y * 0.4),
        ResourceNode { resource_type: ResourceType::Iron },
        Bobbing { offset: position.x + position.z, speed: 2.0 },
        EffectEmitter { timer: 0.0, interval: 0.1 },
    )).id();

    // 外側グロー効果
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.5, 1.0, 0.3),
            emissive: LinearRgba::new(0.0, 0.4, 0.8, 0.5),
            alpha_mode: bevy::prelude::AlphaMode::Blend,
            ..default()
        })),
        Transform::from_translation(position + Vec3::Y * 0.4),
        Bobbing { offset: position.x + position.z + 1.0, speed: 1.5 },
    ));
}

fn spawn_building(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    building_type: BuildingType,
) {
    let (blocks, base_color) = match building_type {
        BuildingType::Factory => (
            vec![(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1),  // 基部 2x2
                 (0, 1, 0), (1, 1, 0), (0, 1, 1), (1, 1, 1),  // 2層目
                 (0, 2, 0), (1, 2, 1)],                        // 煙突
            Color::srgb(0.6, 0.3, 0.2), // レンガ色
        ),
        BuildingType::Storage => (
            vec![(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1),  // 基部 2x2
                 (0, 1, 0), (1, 1, 1)],                        // 上部
            Color::srgb(0.4, 0.4, 0.4), // 石色
        ),
    };
    
    let material = materials.add(StandardMaterial { 
        base_color,
        perceptual_roughness: 0.8,
        ..default() 
    });
    
    // ボクセルブロックを配置
    for (dx, dy, dz) in blocks {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(position + Vec3::new(dx as f32, dy as f32 + 1.0, dz as f32)),
            Building { building_type, progress: 100.0, producing: false },
        ));
    }
    
    // 工場の場合は煙突に煙エフェクト
    if building_type == BuildingType::Factory {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.3))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.8, 0.8, 0.8, 0.6),
                alpha_mode: bevy::prelude::AlphaMode::Blend,
                unlit: true,
                ..default()
            })),
            Transform::from_translation(position + Vec3::new(0.5, 4.0, 0.5)),
            Bobbing { offset: 0.0, speed: 2.0 },
        ));
    }
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

    // Minecraft風ボクセル地形生成
    spawn_minecraft_terrain(&mut commands, &mut meshes, &mut materials);
    
    // 自然オブジェクト生成
    spawn_minecraft_objects(&mut commands, &mut meshes, &mut materials);

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

    // Minecraft風太陽光照明
    // 太陽光 - 温かい黄色系
    commands.spawn((
        DirectionalLight { 
            illuminance: 12000.0, 
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true, 
            ..default() 
        }, 
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.3, 0.0))
    ));
    
    // 明るいアンビエント - 空の色
    commands.insert_resource(AmbientLight { 
        color: Color::srgb(0.6, 0.8, 1.0), 
        brightness: 800.0 
    });
}

// === 行動システム ===

fn agent_behavior(
    time: Res<Time>,
    mut state: ResMut<FactoryState>,
    resources: Query<(Entity, &Transform), With<ResourceNode>>,
    buildings: Query<&Transform, With<Building>>,
    mut agents: Query<(&mut Transform, &mut Agent), (Without<ResourceNode>, Without<Building>)>,
    mut commands: Commands,
) {
    let resource_positions: Vec<(Entity, Vec3)> = resources.iter().map(|(e, t)| (e, t.translation)).collect();
    let storage_pos = buildings.iter().find(|t| t.translation.distance(Vec3::new(5.0, 0.5, 0.0)) < 0.1).map(|t| t.translation);
    let factory_pos = buildings.iter().find(|t| t.translation.distance(Vec3::new(0.0, 0.75, -6.0)) < 0.1).map(|t| t.translation);
    
    for (mut transform, mut agent) in &mut agents {
        match agent.role {
            AgentRole::Collector => {
                if agent.carrying.is_some() {
                    // 運搬中 → ストレージへ
                    if let Some(storage) = storage_pos {
                        let dir = (storage - transform.translation).normalize_or_zero();
                        let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                        transform.translation += dir_xz * agent.speed * time.delta_secs();
                        if dir_xz.length() > 0.01 { let target = transform.translation + dir_xz; transform.look_at(target, Vec3::Y); }
                        
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
                        if dir_xz.length() > 0.01 { let target = transform.translation + dir_xz; transform.look_at(target, Vec3::Y); }
                        
                        if transform.translation.distance(*res_pos) < 0.8 {
                            agent.carrying = Some(ResourceType::Iron);
                            commands.entity(*entity).despawn();
                        }
                    }
                }
            },
            
            AgentRole::Builder => {
                // ランダム巡回
                let t = time.elapsed_secs();
                let target = Vec3::new((t * 0.5).sin() * 8.0, 0.0, (t * 0.3).cos() * 8.0);
                let dir = (target - transform.translation).normalize_or_zero();
                let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                transform.translation += dir_xz * agent.speed * time.delta_secs() * 0.5;
                if dir_xz.length() > 0.01 { let target = transform.translation + dir_xz; transform.look_at(target, Vec3::Y); }
            },
            
            AgentRole::Worker => {
                if let Some(factory) = factory_pos {
                    let dist = transform.translation.distance(factory);
                    if dist > 3.0 {
                        let dir = (factory - transform.translation).normalize_or_zero();
                        let dir_xz = Vec3::new(dir.x, 0.0, dir.z).normalize_or_zero();
                        transform.translation += dir_xz * agent.speed * time.delta_secs();
                        if dir_xz.length() > 0.01 { let target = transform.translation + dir_xz; transform.look_at(target, Vec3::Y); }
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
        
        // 境界制限 (Minecraft地形内)
        transform.translation.x = transform.translation.x.clamp(-15.0, 15.0);
        transform.translation.z = transform.translation.z.clamp(-15.0, 15.0);
        
        // Y座標を地形に合わせて調整
        let grid_x = transform.translation.x.round() as i32;
        let grid_z = transform.translation.z.round() as i32;
        let height = ((grid_x as f32 * 0.1).sin() * (grid_z as f32 * 0.15).cos() * 2.0 + 
                     (grid_x as f32 * 0.05).cos() * (grid_z as f32 * 0.08).sin() * 1.0).round() as i32;
        let ground_y = 0.max(height.min(3)) + 1;
        transform.translation.y = ground_y as f32 + 0.5; // 地面より少し上
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

// Minecraft風地形生成関数
fn spawn_minecraft_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 草ブロック用マテリアル
    let grass_top_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.8, 0.3),
        perceptual_roughness: 0.8,
        ..default()
    });
    
    let dirt_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.3, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });
    
    // ボクセル地形生成 (32x32エリア)
    for x in -16..=16 {
        for z in -16..=16 {
            // 高さマップ生成 (シンプルなノイズ)
            let height = ((x as f32 * 0.1).sin() * (z as f32 * 0.15).cos() * 2.0 + 
                         (x as f32 * 0.05).cos() * (z as f32 * 0.08).sin() * 1.0).round() as i32;
            let base_height = 0.max(height.min(3));
            
            // 地下ブロック (土)
            for y in 0..base_height {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(dirt_material.clone()),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                ));
            }
            
            // 表面ブロック (草)
            if base_height >= 0 {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(grass_top_material.clone()),
                    Transform::from_xyz(x as f32, base_height as f32, z as f32),
                ));
            }
        }
    }
}

fn spawn_minecraft_objects(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 木の幹マテリアル
    let wood_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.2, 0.1),
        perceptual_roughness: 0.9,
        ..default()
    });
    
    // 葉っぱマテリアル
    let leaves_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.2),
        perceptual_roughness: 0.8,
        ..default()
    });
    
    // 石マテリアル
    let stone_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.5),
        perceptual_roughness: 0.7,
        ..default()
    });
    
    // 水マテリアル
    let water_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.2, 0.4, 0.8, 0.7),
        alpha_mode: bevy::prelude::AlphaMode::Blend,
        perceptual_roughness: 0.1,
        ..default()
    });
    
    // 木を生成 (数本)
    for i in 0..8 {
        let angle = i as f32 * 0.785; // 45度ずつ
        let x = (angle.cos() * 8.0) as i32;
        let z = (angle.sin() * 8.0) as i32;
        
        // 地面の高さを計算
        let ground_height = ((x as f32 * 0.1).sin() * (z as f32 * 0.15).cos() * 2.0 + 
                           (x as f32 * 0.05).cos() * (z as f32 * 0.08).sin() * 1.0).round() as i32;
        let base_y = 0.max(ground_height.min(3)) + 1;
        
        // 木の幹 (3-5ブロック高)
        let tree_height = 3 + (i % 3);
        for y in 0..tree_height {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(wood_material.clone()),
                Transform::from_xyz(x as f32, (base_y + y) as f32, z as f32),
            ));
        }
        
        // 葉っぱ (3x3x2の立方体)
        let leaves_y = base_y + tree_height;
        for dx in -1..=1 {
            for dz in -1..=1 {
                for dy in 0..=1 {
                    if !(dx == 0 && dz == 0 && dy == 0) { // 幹の位置は除く
                        commands.spawn((
                            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                            MeshMaterial3d(leaves_material.clone()),
                            Transform::from_xyz((x + dx) as f32, (leaves_y + dy) as f32, (z + dz) as f32),
                        ));
                    }
                }
            }
        }
    }
    
    // 石の塊を生成 (数か所)
    for i in 0..5 {
        let angle = i as f32 * 1.256; // 72度ずつ
        let x = (angle.cos() * 12.0) as i32;
        let z = (angle.sin() * 12.0) as i32;
        
        let ground_height = ((x as f32 * 0.1).sin() * (z as f32 * 0.15).cos() * 2.0).round() as i32;
        let base_y = 0.max(ground_height.min(3)) + 1;
        
        // 2x2の石ブロック
        for dx in 0..2 {
            for dz in 0..2 {
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                    MeshMaterial3d(stone_material.clone()),
                    Transform::from_xyz((x + dx) as f32, base_y as f32, (z + dz) as f32),
                ));
            }
        }
    }
    
    // 水の池を生成 (中央付近)
    for x in -2..=2 {
        for z in -2..=2 {
            if x * x + z * z <= 4 { // 円形の池
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 1.0))),
                    MeshMaterial3d(water_material.clone()),
                    Transform::from_xyz(x as f32, 0.75, z as f32), // 地面より少し高い
                ));
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

fn update_trails(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut agents: Query<(&Transform, &mut Trail, &Agent)>,
) {
    for (transform, mut trail, agent) in &mut agents {
        trail.spawn_timer += time.delta_secs();
        
        // 軌跡ポイント追加
        if trail.spawn_timer >= 0.05 {
            trail.spawn_timer = 0.0;
            trail.positions.push((transform.translation, 1.0));
            
            // 最大長制限
            if trail.positions.len() > trail.max_length {
                trail.positions.remove(0);
            }
        }
        
        // 寿命更新
        for (_, lifetime) in &mut trail.positions {
            *lifetime -= time.delta_secs() * 2.0;
        }
        trail.positions.retain(|(_, lifetime)| *lifetime > 0.0);
        
        // トレイルパーティクル生成
        for (i, (pos, lifetime)) in trail.positions.iter().enumerate() {
            if i % 3 == 0 {  // 間引き
                let alpha = *lifetime * 0.5;
                let scale = 0.1 + alpha * 0.15;
                
                let color = match agent.role {
                    AgentRole::Collector => Color::srgb(0.0, 1.0, 0.5),
                    AgentRole::Builder => Color::srgb(1.0, 0.6, 0.0),
                    AgentRole::Worker => Color::srgb(0.2, 0.4, 1.0),
                };
                
                let linear: LinearRgba = color.into();
                let base_color = Color::srgba(linear.red, linear.green, linear.blue, alpha);
                let emissive_color = LinearRgba::new(linear.red, linear.green, linear.blue, 0.8);
                
                commands.spawn((
                    Mesh3d(meshes.add(Sphere::new(scale))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color,
                        emissive: emissive_color,
                        alpha_mode: bevy::prelude::AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    })),
                    Transform::from_translation(*pos + Vec3::Y * 0.3),
                    TrailParticle { lifetime: *lifetime, max_lifetime: 1.0 },
                ));
            }
        }
    }
}

fn update_trail_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut TrailParticle, &mut Transform)>,
) {
    for (entity, mut particle, mut transform) in &mut particles {
        particle.lifetime -= time.delta_secs() * 2.0;
        
        // フェードアウト
        transform.scale *= 0.95;
        
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_effect_emitters(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut emitters: Query<(&Transform, &mut EffectEmitter), With<ResourceNode>>,
) {
    for (transform, mut emitter) in &mut emitters {
        emitter.timer += time.delta_secs();
        
        if emitter.timer >= emitter.interval {
            emitter.timer = 0.0;
            
            // スパークエフェクト
            let angle = time.elapsed_secs() * 3.0;
            let radius = 0.8;
            let spark_pos = transform.translation + Vec3::new(
                angle.cos() * radius,
                (angle * 2.0).sin() * 0.2,
                angle.sin() * radius,
            );
            
            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.05))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgba(1.0, 1.0, 0.8, 0.8),
                    emissive: LinearRgba::new(0.8, 0.8, 1.0, 1.0),
                    alpha_mode: bevy::prelude::AlphaMode::Blend,
                    unlit: true,
                    ..default()
                })),
                Transform::from_translation(spark_pos),
                TrailParticle { lifetime: 0.5, max_lifetime: 0.5 },
            ));
        }
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in &mut query {
        let t = time.elapsed_secs();
        
        // 複雑な軌道パターン
        let angle = t * 0.08;
        let height_wave = (t * 0.3).sin() * 5.0;
        let radius_wave = (t * 0.15).cos() * 5.0;
        
        let radius = 25.0 + radius_wave;
        let height = 18.0 + height_wave;
        
        // 螺旋的カメラ動作
        let x = angle.sin() * radius + (t * 0.2).cos() * 3.0;
        let y = height + (t * 0.25).sin() * 2.0;
        let z = angle.cos() * radius + (t * 0.1).sin() * 3.0;
        
        transform.translation = Vec3::new(x, y, z);
        
        // 注視点も少し動的に
        let look_target = Vec3::new(
            (t * 0.1).sin() * 2.0,
            (t * 0.2).cos() * 1.0,
            0.0
        );
        transform.look_at(look_target, Vec3::Y);
    }
}
