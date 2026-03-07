use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minecraft Woodcutting - Multiplayer AI".into(),
                resolution: (1000., 750.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WoodcuttingState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            woodcutter_behavior,
            animate_trees,
            rotate_camera,
        ))
        .run();
}

// === ゲーム状態 ===

#[derive(Resource, Default)]
struct WoodcuttingState {
    wood_collected: u32,
    trees_cut: u32,
}

// === コンポーネント ===

#[derive(Component)]
struct Woodcutter {
    speed: f32,
    carrying_wood: bool,
    target_tree: Option<Entity>,
    target_chest: Option<Entity>,
}

#[derive(Component)]
struct Tree {
    health: f32,
    max_health: f32,
    being_cut: bool,
}

#[derive(Component)]
struct Chest {
    wood_count: u32,
    max_capacity: u32,
}

#[derive(Component)]
struct Bobbing { 
    offset: f32, 
    speed: f32 
}

#[derive(Component)]
struct MainCamera;

// === スポーン関数 ===

fn spawn_woodcutter(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let body_mat = materials.add(StandardMaterial { 
        base_color: Color::srgb(0.6, 0.4, 0.2),  // 茶色の服
        perceptual_roughness: 0.8,
        ..default() 
    });
    
    let skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.7, 0.6),  // 肌色
        perceptual_roughness: 0.9,
        ..default()
    });
    
    commands.spawn((
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
        Woodcutter {
            speed: 3.0,
            carrying_wood: false,
            target_tree: None,
            target_chest: None,
        },
    )).with_children(|p| {
        // 体
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.5, 0.7, 0.3))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.0, 0.5, 0.0)));
        // 頭
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.4, 0.4, 0.4))), MeshMaterial3d(skin_mat.clone()), Transform::from_xyz(0.0, 1.0, 0.0)));
        // 目
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.08, 0.04, 0.02))), MeshMaterial3d(materials.add(Color::BLACK)), Transform::from_xyz(-0.06, 1.05, 0.15)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.08, 0.04, 0.02))), MeshMaterial3d(materials.add(Color::BLACK)), Transform::from_xyz(0.06, 1.05, 0.15)));
        // 腕
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.15, 0.4, 0.15))), MeshMaterial3d(skin_mat.clone()), Transform::from_xyz(-0.35, 0.4, 0.0)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.15, 0.4, 0.15))), MeshMaterial3d(skin_mat.clone()), Transform::from_xyz(0.35, 0.4, 0.0)));
        // 脚
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.18, 0.4, 0.18))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(-0.12, 0.0, 0.0)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.18, 0.4, 0.18))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.12, 0.0, 0.0)));
        
        // 斧 (木こり道具)
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.06, 0.6, 0.06))), MeshMaterial3d(materials.add(Color::srgb(0.4, 0.2, 0.1))), Transform::from_xyz(0.5, 0.5, 0.0))); // 柄
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.2, 0.15, 0.08))), MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.7))), Transform::from_xyz(0.6, 0.8, 0.0))); // 刃
    });
}

fn spawn_chest(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let chest_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.2, 0.1),  // 茶色の木材
        perceptual_roughness: 0.8,
        ..default()
    });
    
    let metal_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),  // 金属の金具
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 0.8, 0.8))),
        MeshMaterial3d(chest_material.clone()),
        Transform::from_translation(position),
        Chest {
            wood_count: 0,
            max_capacity: 20,
        },
    )).with_children(|p| {
        // 金具装飾
        p.spawn((Mesh3d(meshes.add(Cuboid::new(1.3, 0.1, 0.1))), MeshMaterial3d(metal_material.clone()), Transform::from_xyz(0.0, 0.2, 0.0)));
        p.spawn((Mesh3d(meshes.add(Cuboid::new(1.3, 0.1, 0.1))), MeshMaterial3d(metal_material.clone()), Transform::from_xyz(0.0, -0.2, 0.0)));
    });
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

    // シンプルなボクセル地形生成
    spawn_minecraft_terrain(&mut commands, &mut meshes, &mut materials);
    
    // 木を配置
    spawn_trees(&mut commands, &mut meshes, &mut materials);

    // 木こりエージェント (5人)
    for i in 0..5 {
        let angle = i as f32 * 1.256; // 72度ずつ
        spawn_woodcutter(&mut commands, &mut meshes, &mut materials, Vec3::new(angle.cos() * 3.0, 2.0, angle.sin() * 3.0));
    }

    // チェスト (2つ)
    spawn_chest(&mut commands, &mut meshes, &mut materials, Vec3::new(-8.0, 2.0, -8.0));
    spawn_chest(&mut commands, &mut meshes, &mut materials, Vec3::new(8.0, 2.0, 8.0));

    // Minecraft風太陽光照明
    commands.spawn((
        DirectionalLight { 
            illuminance: 12000.0, 
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true, 
            ..default() 
        }, 
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.6, 0.3, 0.0))
    ));
    
    commands.insert_resource(AmbientLight { 
        color: Color::srgb(0.6, 0.8, 1.0), 
        brightness: 800.0 
    });
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
    
    // シンプルな平坦地形 (16x16エリア)
    for x in -8..=8 {
        for z in -8..=8 {
            // 地下ブロック (土)
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(dirt_material.clone()),
                Transform::from_xyz(x as f32, 0.0, z as f32),
            ));
            
            // 表面ブロック (草)
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                MeshMaterial3d(grass_top_material.clone()),
                Transform::from_xyz(x as f32, 1.0, z as f32),
            ));
        }
    }
}

fn spawn_trees(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 木の幹マテリアル
    let trunk_material = materials.add(StandardMaterial {
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
    
    // 木を配置 (8本)
    let positions = [
        Vec3::new(4.0, 2.0, 4.0),
        Vec3::new(-4.0, 2.0, 4.0),
        Vec3::new(4.0, 2.0, -4.0),
        Vec3::new(-4.0, 2.0, -4.0),
        Vec3::new(0.0, 2.0, 6.0),
        Vec3::new(0.0, 2.0, -6.0),
        Vec3::new(6.0, 2.0, 0.0),
        Vec3::new(-6.0, 2.0, 0.0),
    ];
    
    for position in positions {
        spawn_tree(commands, meshes, materials, position, &trunk_material, &leaves_material);
    }
}

fn spawn_tree(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    trunk_material: &Handle<StandardMaterial>,
    leaves_material: &Handle<StandardMaterial>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 3.0, 0.5))),
        MeshMaterial3d(trunk_material.clone()),
        Transform::from_translation(position),
        Tree {
            health: 100.0,
            max_health: 100.0,
            being_cut: false,
        },
    )).with_children(|p| {
        // 葉っぱ (球状)
        p.spawn((
            Mesh3d(meshes.add(Sphere::new(2.0))),
            MeshMaterial3d(leaves_material.clone()),
            Transform::from_xyz(0.0, 2.0, 0.0),
        ));
    });
}

// === 行動システム ===

fn woodcutter_behavior(
    time: Res<Time>,
    mut state: ResMut<WoodcuttingState>,
    mut woodcutters: Query<(&mut Transform, &mut Woodcutter)>,
    mut trees: Query<(Entity, &Transform, &mut Tree), (With<Tree>, Without<Woodcutter>)>,
    mut chests: Query<(Entity, &Transform, &mut Chest), (With<Chest>, Without<Woodcutter>, Without<Tree>)>,
    mut commands: Commands,
) {
    for (mut transform, mut woodcutter) in &mut woodcutters {
        if woodcutter.carrying_wood {
            // 木材を運搬中 → 最寄りのチェストへ
            if let Some((chest_entity, chest_transform, _)) = chests.iter().min_by(|a, b| {
                transform.translation.distance(a.1.translation).partial_cmp(&transform.translation.distance(b.1.translation)).unwrap()
            }) {
                let dir = (chest_transform.translation - transform.translation).normalize_or_zero();
                transform.translation += dir * woodcutter.speed * time.delta_secs();
                if transform.translation.distance(chest_transform.translation) < 1.5 {
                    woodcutter.carrying_wood = false;
                    state.wood_collected += 1;
                    
                    // チェストの中身を更新
                    if let Ok((_, _, mut chest)) = chests.get_mut(chest_entity) {
                        if chest.wood_count < chest.max_capacity {
                            chest.wood_count += 1;
                        }
                    }
                }
            }
        } else {
            // 木を探して切る
            if let Some((tree_entity, tree_transform, mut tree)) = trees.iter_mut().min_by(|a, b| {
                transform.translation.distance(a.1.translation).partial_cmp(&transform.translation.distance(b.1.translation)).unwrap()
            }) {
                let distance = transform.translation.distance(tree_transform.translation);
                
                if distance > 2.0 {
                    // 木に近づく
                    let dir = (tree_transform.translation - transform.translation).normalize_or_zero();
                    transform.translation += dir * woodcutter.speed * time.delta_secs();
                } else {
                    // 木を切る
                    tree.health -= 30.0 * time.delta_secs(); // 1秒で30ダメージ
                    
                    if tree.health <= 0.0 {
                        // 木が切れた
                        commands.entity(tree_entity).despawn_recursive();
                        woodcutter.carrying_wood = true;
                        state.trees_cut += 1;
                    }
                }
            }
        }
        
        // 境界制限
        transform.translation.x = transform.translation.x.clamp(-15.0, 15.0);
        transform.translation.z = transform.translation.z.clamp(-15.0, 15.0);
        transform.translation.y = 2.5; // 地面より少し上の固定高さ
        
        // 移動方向を向く
        if woodcutter.carrying_wood {
            if let Some((_, chest_transform, _)) = chests.iter().min_by(|a, b| {
                transform.translation.distance(a.1.translation).partial_cmp(&transform.translation.distance(b.1.translation)).unwrap()
            }) {
                let look_dir = (chest_transform.translation - transform.translation).normalize_or_zero();
                if look_dir.length() > 0.01 {
                    transform.look_at(transform.translation + look_dir, Vec3::Y);
                }
            }
        }
    }
}

fn animate_trees(
    time: Res<Time>,
    mut trees: Query<&mut Transform, With<Tree>>,
) {
    for mut transform in &mut trees {
        // 木の軽い揺れアニメーション
        let sway = (time.elapsed_secs() + transform.translation.x + transform.translation.z).sin() * 0.02;
        transform.rotation = Quat::from_rotation_z(sway);
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    for mut transform in &mut query {
        let t = time.elapsed_secs();
        
        // シンプルな軌道
        let angle = t * 0.05;
        let radius = 20.0;
        let height = 15.0;
        
        transform.translation = Vec3::new(angle.sin() * radius, height, angle.cos() * radius);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}