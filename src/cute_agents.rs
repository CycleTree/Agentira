use bevy::prelude::*;
use std::f32::consts::PI;

// === 可愛いエージェントシステム ===

#[derive(Component)]
pub struct CuteChef {
    pub speed: f32,
    pub current_task: Option<String>,
    pub carrying_item: Option<String>,
    pub target_station: Option<Vec3>,
    pub chef_type: ChefPersonality,
    pub animation_timer: f32,
    pub blink_timer: f32,
    pub mood: ChefMood,
}

#[derive(Clone, Debug)]
pub enum ChefPersonality {
    Energetic,    // 元気っ子 - オレンジ系
    Gentle,       // 優しい子 - パステルピンク
    Cool,         // クール系 - 青系
}

#[derive(Clone, Debug)]
pub enum ChefMood {
    Happy,
    Focused,
    Excited,
    Tired,
}

impl Default for CuteChef {
    fn default() -> Self {
        Self {
            speed: 2.5,
            current_task: None,
            carrying_item: None,
            target_station: None,
            chef_type: ChefPersonality::Gentle,
            animation_timer: 0.0,
            blink_timer: 0.0,
            mood: ChefMood::Happy,
        }
    }
}

// === 可愛いシェフスポーン関数 ===
pub fn spawn_cute_chef(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    name: String,
    personality: ChefPersonality,
) {
    // パーソナリティ別カラーパレット
    let colors = get_chef_colors(&personality);
    
    commands.spawn((
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        CuteChef {
            chef_type: personality.clone(),
            ..default()
        },
        Name::new(name),
    )).with_children(|parent| {
        // === 体部分（可愛い樽型） ===
        spawn_cute_body(parent, meshes, materials, &colors);
        
        // === 頭部（大きめで可愛い） ===
        spawn_cute_head(parent, meshes, materials, &colors);
        
        // === 腕（可動域を考慮） ===
        spawn_cute_arms(parent, meshes, materials, &colors);
        
        // === 脚（短足で可愛い） ===
        spawn_cute_legs(parent, meshes, materials, &colors);
        
        // === アクセサリー ===
        spawn_chef_accessories(parent, meshes, materials, &colors, &personality);
    });
}

#[derive(Clone)]
struct ChefColors {
    skin: Color,
    hair: Color,
    uniform_primary: Color,
    uniform_secondary: Color,
    hat: Color,
    eyes: Color,
    accent: Color,
}

fn get_chef_colors(personality: &ChefPersonality) -> ChefColors {
    match personality {
        ChefPersonality::Energetic => ChefColors {
            skin: Color::srgb(0.95, 0.78, 0.65),      // 健康的な肌色
            hair: Color::srgb(0.8, 0.45, 0.2),       // オレンジブラウン
            uniform_primary: Color::srgb(0.95, 0.95, 0.9),   // クリーム白
            uniform_secondary: Color::srgb(1.0, 0.6, 0.2),   // 元気なオレンジ
            hat: Color::srgb(1.0, 1.0, 1.0),         // 純白
            eyes: Color::srgb(0.2, 0.6, 0.9),        // 明るい青
            accent: Color::srgb(0.9, 0.3, 0.1),      // エネルギッシュな赤
        },
        ChefPersonality::Gentle => ChefColors {
            skin: Color::srgb(0.98, 0.85, 0.75),     // 優しい桃色肌
            hair: Color::srgb(0.6, 0.4, 0.3),        // ソフトブラウン
            uniform_primary: Color::srgb(0.98, 0.96, 0.94),  // 温かい白
            uniform_secondary: Color::srgb(0.9, 0.7, 0.8),   // 優しいピンク
            hat: Color::srgb(0.98, 0.95, 0.97),      // ソフト白
            eyes: Color::srgb(0.4, 0.7, 0.4),        // 優しい緑
            accent: Color::srgb(0.8, 0.5, 0.6),      // 温かいピンク
        },
        ChefPersonality::Cool => ChefColors {
            skin: Color::srgb(0.92, 0.82, 0.72),     // クールな肌色
            hair: Color::srgb(0.3, 0.3, 0.4),        // ダークブルーグレー
            uniform_primary: Color::srgb(0.94, 0.96, 0.98),  // クール白
            uniform_secondary: Color::srgb(0.6, 0.8, 0.9),   // クールブルー
            hat: Color::srgb(0.97, 0.98, 1.0),       // 冷色系白
            eyes: Color::srgb(0.3, 0.5, 0.8),        // 深い青
            accent: Color::srgb(0.4, 0.6, 0.8),      // 知的なブルー
        }
    }
}

fn spawn_cute_body(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    // メイン胴体（丸みを帯びた樽型）
    parent.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.35, 0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.uniform_primary,
            metallic: 0.0,
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.6, 0.0),
    ));
    
    // エプロン（前面装飾）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.8, 0.05))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.uniform_secondary,
            metallic: 0.1,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.6, 0.38),
    ));
    
    // エプロンの紐
    for x in [-0.15, 0.15] {
        parent.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.02, 0.4))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: colors.accent,
                ..default()
            })),
            Transform::from_xyz(x, 0.9, 0.0),
        ));
    }
    
    // 胸元のボタン（可愛いディテール）
    for (i, y) in [0.9, 0.7, 0.5].iter().enumerate() {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.04))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.85, 0.8),
                metallic: 0.3,
                perceptual_roughness: 0.2,
                ..default()
            })),
            Transform::from_xyz(0.0, *y, 0.42),
        ));
    }
}

fn spawn_cute_head(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    // メイン頭部（大きめで可愛い比率）
    parent.spawn((
        Mesh3d(meshes.add(Sphere::new(0.32))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.skin,
            metallic: 0.0,
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.3, 0.0),
    ));
    
    // 髪（パーソナリティ別スタイル）
    spawn_cute_hair(parent, meshes, materials, colors);
    
    // 目（大きくて可愛い）
    spawn_cute_eyes(parent, meshes, materials, colors);
    
    // 鼻（小さくて上品）
    parent.spawn((
        Mesh3d(meshes.add(Sphere::new(0.04))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.skin,
            metallic: 0.0,
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.25, 0.28),
    ));
    
    // 口（笑顔）
    parent.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.02, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.4, 0.4),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.15, 0.3)
            .with_rotation(Quat::from_rotation_z(PI / 8)),
    ));
    
    // コック帽（立派で可愛い）
    spawn_cute_chef_hat(parent, meshes, materials, colors);
}

fn spawn_cute_hair(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    let hair_material = materials.add(StandardMaterial {
        base_color: colors.hair,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    // 前髪
    parent.spawn((
        Mesh3d(meshes.add(Sphere::new(0.25))),
        MeshMaterial3d(hair_material.clone()),
        Transform::from_xyz(0.0, 1.45, 0.15),
    ));
    
    // サイドの髪
    for x in [-0.2, 0.2] {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.18))),
            MeshMaterial3d(hair_material.clone()),
            Transform::from_xyz(x, 1.35, 0.05),
        ));
    }
}

fn spawn_cute_eyes(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    // 目の土台（白目）
    for x in [-0.1, 0.1] {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.08))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                metallic: 0.0,
                perceptual_roughness: 0.1,
                ..default()
            })),
            Transform::from_xyz(x, 1.35, 0.25),
        ));
        
        // 虹彩（カラフルで可愛い）
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: colors.eyes,
                metallic: 0.2,
                perceptual_roughness: 0.3,
                ..default()
            })),
            Transform::from_xyz(x, 1.35, 0.28),
        ));
        
        // 瞳孔
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.025))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.15),
                ..default()
            })),
            Transform::from_xyz(x, 1.35, 0.3),
        ));
        
        // ハイライト（生き生きとした表情）
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.015))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                emissive: LinearRgba::rgb(0.5, 0.5, 0.5),
                ..default()
            })),
            Transform::from_xyz(x + 0.02, 1.37, 0.31),
        ));
    }
}

fn spawn_cute_chef_hat(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    let hat_material = materials.add(StandardMaterial {
        base_color: colors.hat,
        metallic: 0.0,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    // 帽子のベース
    parent.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.4, 0.1))),
        MeshMaterial3d(hat_material.clone()),
        Transform::from_xyz(0.0, 1.55, 0.0),
    ));
    
    // メインの帽子部分（膨らんだ形）
    parent.spawn((
        Mesh3d(meshes.add(Sphere::new(0.35))),
        MeshMaterial3d(hat_material.clone()),
        Transform::from_xyz(0.0, 1.8, 0.0)
            .with_scale(Vec3::new(1.0, 1.2, 1.0)),
    ));
    
    // 帽子の先端（ちょこんと曲がって可愛い）
    parent.spawn((
        Mesh3d(meshes.add(Sphere::new(0.08))),
        MeshMaterial3d(hat_material),
        Transform::from_xyz(0.15, 2.1, 0.1),
    ));
}

fn spawn_cute_arms(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    let arm_material = materials.add(StandardMaterial {
        base_color: colors.uniform_primary,
        ..default()
    });
    
    let hand_material = materials.add(StandardMaterial {
        base_color: colors.skin,
        ..default()
    });
    
    // 左右の腕
    for (side, x) in [("left", -0.45), ("right", 0.45)] {
        // 上腕
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.3))),
            MeshMaterial3d(arm_material.clone()),
            Transform::from_xyz(x, 0.9, 0.0)
                .with_rotation(Quat::from_rotation_z(if x < 0.0 { 0.3 } else { -0.3 })),
        ));
        
        // 前腕
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.07, 0.25))),
            MeshMaterial3d(arm_material.clone()),
            Transform::from_xyz(x + if x < 0.0 { -0.15 } else { 0.15 }, 0.5, 0.1)
                .with_rotation(Quat::from_rotation_z(if x < 0.0 { -0.2 } else { 0.2 })),
        ));
        
        // 手
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.09))),
            MeshMaterial3d(hand_material.clone()),
            Transform::from_xyz(x + if x < 0.0 { -0.25 } else { 0.25 }, 0.3, 0.15),
        ));
        
        // 指（簡略化されたかわいい指）
        for i in 0..4 {
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(0.02))),
                MeshMaterial3d(hand_material.clone()),
                Transform::from_xyz(
                    x + if x < 0.0 { -0.32 } else { 0.32 } + (i as f32 - 1.5) * 0.02,
                    0.32,
                    0.18
                ),
            ));
        }
    }
}

fn spawn_cute_legs(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
) {
    let leg_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.2, 0.3), // ダークなパンツ
        ..default()
    });
    
    let shoe_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.1), // 黒い靴
        metallic: 0.2,
        perceptual_roughness: 0.7,
        ..default()
    });
    
    // 短足で可愛い脚
    for x in [-0.12, 0.12] {
        // 太もも
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.25))),
            MeshMaterial3d(leg_material.clone()),
            Transform::from_xyz(x, 0.2, 0.0),
        ));
        
        // すね
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.07, 0.2))),
            MeshMaterial3d(leg_material.clone()),
            Transform::from_xyz(x, -0.1, 0.0),
        ));
        
        // 足（可愛い靴）
        parent.spawn((
            Mesh3d(meshes.add(Capsule3d::new(0.08, 0.15))),
            MeshMaterial3d(shoe_material.clone()),
            Transform::from_xyz(x, -0.25, 0.08)
                .with_rotation(Quat::from_rotation_x(PI / 2)),
        ));
    }
}

fn spawn_chef_accessories(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &ChefColors,
    personality: &ChefPersonality,
) {
    match personality {
        ChefPersonality::Energetic => {
            // エネルギッシュなシェフは調理器具を持っている
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.03, 0.25))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.6, 0.4), // 木の柄
                    ..default()
                })),
                Transform::from_xyz(0.3, 0.4, 0.1),
            ));
            
            // フライ返しの頭部分
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.08, 0.02, 0.12))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.9, 0.9, 0.9), // ステンレス
                    metallic: 0.8,
                    perceptual_roughness: 0.2,
                    ..default()
                })),
                Transform::from_xyz(0.32, 0.55, 0.1),
            ));
        },
        ChefPersonality::Gentle => {
            // 優しいシェフは花のアクセサリー
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(0.06))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.9, 0.7, 0.8), // 優しいピンクの花
                    ..default()
                })),
                Transform::from_xyz(0.25, 1.6, 0.15),
            ));
            
            // 花の中心
            parent.spawn((
                Mesh3d(meshes.add(Sphere::new(0.02))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.9, 0.8, 0.3), // 黄色い中心
                    emissive: LinearRgba::rgb(0.1, 0.1, 0.05),
                    ..default()
                })),
                Transform::from_xyz(0.25, 1.6, 0.18),
            ));
        },
        ChefPersonality::Cool => {
            // クールなシェフはメガネ
            let glass_material = materials.add(StandardMaterial {
                base_color: Color::srgba(0.9, 0.95, 1.0, 0.3), // 透明なガラス
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
            
            let frame_material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.2, 0.3), // ダークフレーム
                metallic: 0.5,
                ..default()
            });
            
            // メガネのレンズ
            for x in [-0.08, 0.08] {
                parent.spawn((
                    Mesh3d(meshes.add(Cylinder::new(0.06, 0.005))),
                    MeshMaterial3d(glass_material.clone()),
                    Transform::from_xyz(x, 1.35, 0.32)
                        .with_rotation(Quat::from_rotation_x(PI / 2)),
                ));
                
                // フレーム
                parent.spawn((
                    Mesh3d(meshes.add(Torus::new(0.06, 0.008))),
                    MeshMaterial3d(frame_material.clone()),
                    Transform::from_xyz(x, 1.35, 0.32)
                        .with_rotation(Quat::from_rotation_x(PI / 2)),
                ));
            }
            
            // ブリッジ
            parent.spawn((
                Mesh3d(meshes.add(Cylinder::new(0.005, 0.04))),
                MeshMaterial3d(frame_material),
                Transform::from_xyz(0.0, 1.35, 0.32)
                    .with_rotation(Quat::from_rotation_z(PI / 2)),
            ));
        }
    }
}

// === アニメーションシステム ===
pub fn animate_cute_chefs(
    time: Res<Time>,
    mut chefs: Query<(&mut CuteChef, &mut Transform, &Children)>,
    mut transforms: Query<&mut Transform, (With<Parent>, Without<CuteChef>)>,
) {
    for (mut chef, mut chef_transform, children) in chefs.iter_mut() {
        chef.animation_timer += time.delta_secs();
        chef.blink_timer += time.delta_secs();
        
        // 歩行アニメーション
        let walk_cycle = (chef.animation_timer * 3.0).sin();
        chef_transform.translation.y += walk_cycle * 0.02;
        
        // 腕の振りアニメーション
        if let Some(task) = &chef.current_task {
            // タスク中は特別なアニメーション
            match task.as_str() {
                "cooking" => {
                    // 調理中は腕を動かす
                    for &child in children.iter() {
                        if let Ok(mut child_transform) = transforms.get_mut(child) {
                            // 簡単な腕振りアニメーション
                            child_transform.rotation = Quat::from_rotation_y(
                                (chef.animation_timer * 4.0).sin() * 0.1
                            );
                        }
                    }
                },
                _ => {}
            }
        }
        
        // まばたき（ランダム）
        if chef.blink_timer > 2.0 && (chef.blink_timer * 7.0).sin() > 0.9 {
            chef.blink_timer = 0.0;
            // 目を一瞬閉じる処理（実装は簡略化）
        }
    }
}