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
            tree_respawn_system,
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

#[derive(Component)]
struct TreeRespawnTimer {
    position: Vec3,
    respawn_time: f32,
    elapsed: f32,
}

// === スポーン関数 ===

fn spawn_woodcutter(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let body_mat = materials.add(StandardMaterial { 
        base_color: Color::srgb(0.6, 0.4, 0.2),  // 茶色の服
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default() 
    });
    
    let skin_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.7, 0.6),  // 肌色
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
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
        // 軽量化: パーツを大幅削減（マインクラフト風シンプル）
        // 体（統合）
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.6, 1.2, 0.4))), MeshMaterial3d(body_mat.clone()), Transform::from_xyz(0.0, 0.6, 0.0)));
        // 頭
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))), MeshMaterial3d(skin_mat.clone()), Transform::from_xyz(0.0, 1.3, 0.0)));
        // 斧（統合）
        p.spawn((Mesh3d(meshes.add(Cuboid::new(0.08, 0.8, 0.08))), MeshMaterial3d(materials.add(Color::srgb(0.4, 0.2, 0.1))), Transform::from_xyz(0.5, 0.8, 0.0)));
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
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });
    
    let metal_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),  // 金属の金具
        metallic: 0.0,  // 軽量化: 金属反射なし
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });
    
    // 軽量化: チェストをシンプルな単一立方体に
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 0.8, 0.8))),
        MeshMaterial3d(chest_material.clone()),
        Transform::from_translation(position),
        Chest {
            wood_count: 0,
            max_capacity: 20,
        },
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
    // 草ブロック用マテリアル（軽量化）
    let grass_top_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.8, 0.3),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });
    
    let dirt_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.3, 0.2),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
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
    _materials: &mut ResMut<Assets<StandardMaterial>>,
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
        // 葉っぱ (マインクラフト風立方体)
        p.spawn((
            Mesh3d(meshes.add(Cuboid::new(3.0, 3.0, 3.0))),
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
    chests: Query<(Entity, &Transform), (With<Chest>, Without<Woodcutter>, Without<Tree>)>,
    mut chest_storage: Query<&mut Chest>,
    mut commands: Commands,
) {
    for (mut transform, mut woodcutter) in &mut woodcutters {
        if woodcutter.carrying_wood {
            // 木材を運搬中 → 最寄りのチェストへ
            if let Some((chest_entity, chest_transform)) = chests.iter().min_by(|a, b| {
                transform.translation.distance(a.1.translation).partial_cmp(&transform.translation.distance(b.1.translation)).unwrap()
            }) {
                let dir = (chest_transform.translation - transform.translation).normalize_or_zero();
                transform.translation += dir * woodcutter.speed * time.delta_secs();
                if transform.translation.distance(chest_transform.translation) < 1.5 {
                    woodcutter.carrying_wood = false;
                    state.wood_collected += 1;
                    
                    // チェストの中身を更新
                    if let Ok(mut chest) = chest_storage.get_mut(chest_entity) {
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
                        // 木が切れた - リスポーンタイマーを作成
                        let tree_position = tree_transform.translation;
                        commands.spawn(TreeRespawnTimer {
                            position: tree_position,
                            respawn_time: 15.0, // 15秒後にリスポーン
                            elapsed: 0.0,
                        });
                        
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
            if let Some((_, chest_transform)) = chests.iter().min_by(|a, b| {
                transform.translation.distance(a.1.translation).partial_cmp(&transform.translation.distance(b.1.translation)).unwrap()
            }) {
                let look_dir = (chest_transform.translation - transform.translation).normalize_or_zero();
                if look_dir.length() > 0.01 {
                    let target_pos = transform.translation + look_dir;
                    transform.look_at(target_pos, Vec3::Y);
                }
            }
        }
    }
}

fn animate_trees(
    time: Res<Time>,
    mut trees: Query<&mut Transform, With<Tree>>,
) {
    // 軽量化: アニメーション頻度を下げる（0.5秒間隔）
    if (time.elapsed_secs() * 2.0) % 1.0 < 0.1 {
        for mut transform in &mut trees {
            // より軽い計算
            let sway = (time.elapsed_secs() * 0.5).sin() * 0.01;
            transform.rotation = Quat::from_rotation_z(sway);
        }
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    // 軽量化: カメラ更新頻度を下げる
    if (time.elapsed_secs() * 10.0) % 1.0 < 0.1 {
        for mut transform in &mut query {
            let t = time.elapsed_secs();
            
            // より軽い計算
            let angle = t * 0.03; // 回転速度を下げる
            let radius = 20.0;
            let height = 15.0;
            
            transform.translation = Vec3::new(angle.sin() * radius, height, angle.cos() * radius);
            transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }
}

fn tree_respawn_system(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut respawn_timers: Query<(Entity, &mut TreeRespawnTimer)>,
) {
    // 既存のマテリアルを再作成（簡略化）
    let trunk_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.2, 0.1),
        metallic: 0.0,      // 軽量化: 金属反射なし
        perceptual_roughness: 1.0,  // 軽量化: 完全マット
        reflectance: 0.0,   // 軽量化: 反射なし
        ..default()
    });
    
    let leaves_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.6, 0.2),
        metallic: 0.0,
        perceptual_roughness: 1.0,
        reflectance: 0.0,
        ..default()
    });
    
    for (entity, mut timer) in &mut respawn_timers {
        timer.elapsed += time.delta_secs();
        
        if timer.elapsed >= timer.respawn_time {
            // 新しい木を生成
            spawn_tree(
                &mut commands,
                &mut meshes,
                &mut materials,
                timer.position,
                &trunk_material,
                &leaves_material,
            );
            
            // タイマーエンティティを削除
            commands.entity(entity).despawn();
        }
    }
}