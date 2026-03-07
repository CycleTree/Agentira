use bevy::prelude::*;
use std::f32::consts::PI;

// === ボクセル女の子システム ===

#[derive(Component)]
pub struct VoxelGirl {
    pub speed: f32,
    pub current_task: Option<String>,
    pub carrying_item: Option<String>,
    pub target_station: Option<Vec3>,
    pub girl_type: GirlPersonality,
    pub animation_timer: f32,
    pub blink_timer: f32,
    pub mood: GirlMood,
    pub bob_offset: f32,
}

#[derive(Clone, Debug)]
pub enum GirlPersonality {
    Cheerful,     // 元気っ子 - 茶髪メイド風
    Sweet,        // 甘い子 - ピンク髪
    Cool,         // クール - 青髪
}

#[derive(Clone, Debug)]
pub enum GirlMood {
    Happy,
    Working,
    Excited,
    Focused,
}

impl Default for VoxelGirl {
    fn default() -> Self {
        Self {
            speed: 2.0,
            current_task: None,
            carrying_item: None,
            target_station: None,
            girl_type: GirlPersonality::Cheerful,
            animation_timer: 0.0,
            blink_timer: 0.0,
            mood: GirlMood::Happy,
            bob_offset: 0.0,
        }
    }
}

// === ボクセル女の子スポーン関数 ===
pub fn spawn_voxel_girl(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    name: String,
    personality: GirlPersonality,
) {
    let colors = get_girl_colors(&personality);
    
    commands.spawn((
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        VoxelGirl {
            girl_type: personality.clone(),
            bob_offset: (position.x + position.z) * 1.5, // 位置ベースのオフセット
            ..default()
        },
        Name::new(name),
    )).with_children(|parent| {
        // === 下半身（スカート）===
        spawn_voxel_lower_body(parent, meshes, materials, &colors);
        
        // === 上半身（胴体）===
        spawn_voxel_upper_body(parent, meshes, materials, &colors);
        
        // === 頭部（ボクセル風）===
        spawn_voxel_head(parent, meshes, materials, &colors);
        
        // === 腕（ボクセル関節）===
        spawn_voxel_arms(parent, meshes, materials, &colors);
        
        // === アクセサリー（パーソナリティ別）===
        spawn_voxel_accessories(parent, meshes, materials, &colors, &personality);
    });
}

#[derive(Clone)]
struct GirlColors {
    skin: Color,
    hair: Color,
    outfit_primary: Color,
    outfit_secondary: Color,
    outfit_accent: Color,
    eyes: Color,
    special: Color,
}

fn get_girl_colors(personality: &GirlPersonality) -> GirlColors {
    match personality {
        GirlPersonality::Cheerful => GirlColors {
            skin: Color::srgb(0.98, 0.85, 0.75),      // 健康的な肌
            hair: Color::srgb(0.7, 0.4, 0.2),        // 茶髪
            outfit_primary: Color::srgb(0.95, 0.95, 0.9),   // 白いメイド服
            outfit_secondary: Color::srgb(0.2, 0.2, 0.3),   // 黒いエプロン
            outfit_accent: Color::srgb(1.0, 0.6, 0.2),      // オレンジリボン
            eyes: Color::srgb(0.2, 0.6, 0.9),        // 明るい青い目
            special: Color::srgb(0.9, 0.8, 0.1),     // 金色のアクセント
        },
        GirlPersonality::Sweet => GirlColors {
            skin: Color::srgb(0.99, 0.88, 0.78),     // 優しい桃色肌
            hair: Color::srgb(0.9, 0.3, 0.7),        // ピンク髪
            outfit_primary: Color::srgb(0.98, 0.9, 0.95),   // 薄ピンク
            outfit_secondary: Color::srgb(0.8, 0.5, 0.7),   // 濃いピンク
            outfit_accent: Color::srgb(0.9, 0.7, 0.8),      // ローズピンク
            eyes: Color::srgb(0.8, 0.4, 0.6),        // ピンクの目
            special: Color::srgb(0.9, 0.2, 0.5),     // 鮮やかなピンク
        },
        GirlPersonality::Cool => GirlColors {
            skin: Color::srgb(0.94, 0.82, 0.72),     // クールな肌色
            hair: Color::srgb(0.3, 0.5, 0.8),        // 青髪
            outfit_primary: Color::srgb(0.9, 0.95, 0.98),   // 冷色系白
            outfit_secondary: Color::srgb(0.4, 0.6, 0.8),   // クールブルー
            outfit_accent: Color::srgb(0.6, 0.8, 0.9),      // 明るいブルー
            eyes: Color::srgb(0.3, 0.5, 0.8),        // 深い青い目
            special: Color::srgb(0.2, 0.7, 0.9),     // アクアブルー
        }
    }
}

fn spawn_voxel_lower_body(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
) {
    // スカート（ボクセル風フレアスカート）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.8, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.outfit_primary,
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.3, 0.0),
    ));
    
    // スカートの装飾ライン
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.1, 0.1, 1.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.outfit_accent,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    
    // 脚（ボクセル）
    for x in [-0.2, 0.2] {
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.16, 0.6, 0.16))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: colors.skin,
                metallic: 0.0,
                perceptual_roughness: 0.9,
                ..default()
            })),
            Transform::from_xyz(x, -0.1, 0.0),
        ));
        
        // 靴（可愛いボクセル靴）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.1, 0.3))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: colors.outfit_secondary,
                metallic: 0.2,
                perceptual_roughness: 0.6,
                ..default()
            })),
            Transform::from_xyz(x, -0.45, 0.05),
        ));
    }
}

fn spawn_voxel_upper_body(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
) {
    // メイン胴体
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.9, 0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.outfit_primary,
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.1, 0.0),
    ));
    
    // エプロン（前面装飾）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.6, 0.8, 0.05))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.outfit_secondary,
            metallic: 0.0,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.1, 0.28),
    ));
    
    // 胸元のリボン（ボクセル風）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 0.15, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.outfit_accent,
            metallic: 0.1,
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.4, 0.32),
    ));
    
    // リボンの中心
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.special,
            metallic: 0.3,
            perceptual_roughness: 0.5,
            emissive: LinearRgba::rgb(0.1, 0.1, 0.05),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.4, 0.37),
    ));
}

fn spawn_voxel_head(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
) {
    // メイン頭部（ボクセル）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: colors.skin,
            metallic: 0.0,
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.8, 0.0),
    ));
    
    // 髪型（ボクセル風）
    spawn_voxel_hair(parent, meshes, materials, colors);
    
    // 目（ボクセル風ピクセルアート）
    spawn_voxel_eyes(parent, meshes, materials, colors);
    
    // 鼻（小さなボクセル）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.08, 0.06, 0.08))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(
                colors.skin.to_linear().red + 0.05,
                colors.skin.to_linear().green,
                colors.skin.to_linear().blue,
            ),
            metallic: 0.0,
            perceptual_roughness: 0.95,
            ..default()
        })),
        Transform::from_xyz(0.0, 1.75, 0.28),
    ));
    
    // 口（小さなボクセル）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.15, 0.06, 0.05))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.4, 0.4),
            ..default()
        })),
        Transform::from_xyz(0.0, 1.65, 0.29),
    ));
}

fn spawn_voxel_hair(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
) {
    let hair_material = materials.add(StandardMaterial {
        base_color: colors.hair,
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    // ベース髪（頭を覆う）
    parent.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.65, 0.4, 0.65))),
        MeshMaterial3d(hair_material.clone()),
        Transform::from_xyz(0.0, 1.95, 0.0),
    ));
    
    // 前髪（ボクセル風）
    for x in [-0.15, 0.0, 0.15] {
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, 0.25, 0.12))),
            MeshMaterial3d(hair_material.clone()),
            Transform::from_xyz(x, 1.9, 0.25),
        ));
    }
    
    // サイドの髪（ツインテール風）
    for x in [-0.4, 0.4] {
        // サイドヘア上部
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.3, 0.2))),
            MeshMaterial3d(hair_material.clone()),
            Transform::from_xyz(x, 1.9, 0.0),
        ));
        
        // ツインテール
        for y in [0.0, -0.2, -0.4] {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.15, 0.15, 0.15))),
                MeshMaterial3d(hair_material.clone()),
                Transform::from_xyz(x * 1.2, 1.5 + y, -0.1),
            ));
        }
    }
}

fn spawn_voxel_eyes(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
) {
    // 目の土台（白）
    for x in [-0.12, 0.12] {
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, 0.15, 0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                metallic: 0.0,
                perceptual_roughness: 0.1,
                ..default()
            })),
            Transform::from_xyz(x, 1.82, 0.27),
        ));
        
        // 虹彩（カラー）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.08, 0.1, 0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: colors.eyes,
                metallic: 0.2,
                perceptual_roughness: 0.3,
                ..default()
            })),
            Transform::from_xyz(x, 1.82, 0.29),
        ));
        
        // 瞳孔（黒）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.05, 0.06, 0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.15),
                ..default()
            })),
            Transform::from_xyz(x, 1.82, 0.31),
        ));
        
        // ハイライト（発光）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.03, 0.04, 0.05))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(1.0, 1.0, 1.0),
                emissive: LinearRgba::rgb(0.8, 0.8, 0.8),
                ..default()
            })),
            Transform::from_xyz(x + 0.02, 1.85, 0.32),
        ));
    }
}

fn spawn_voxel_arms(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
) {
    let sleeve_material = materials.add(StandardMaterial {
        base_color: colors.outfit_primary,
        ..default()
    });
    
    let hand_material = materials.add(StandardMaterial {
        base_color: colors.skin,
        ..default()
    });
    
    // 左右の腕
    for x in [-0.5, 0.5] {
        // 肩
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
            MeshMaterial3d(sleeve_material.clone()),
            Transform::from_xyz(x, 1.4, 0.0),
        ));
        
        // 上腕（袖）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.18, 0.4, 0.18))),
            MeshMaterial3d(sleeve_material.clone()),
            Transform::from_xyz(x, 1.0, 0.0),
        ));
        
        // 前腕（肌）
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.15, 0.35, 0.15))),
            MeshMaterial3d(hand_material.clone()),
            Transform::from_xyz(x, 0.6, 0.0),
        ));
        
        // 手
        parent.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, 0.12, 0.12))),
            MeshMaterial3d(hand_material.clone()),
            Transform::from_xyz(x, 0.35, 0.0),
        ));
    }
}

fn spawn_voxel_accessories(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    colors: &GirlColors,
    personality: &GirlPersonality,
) {
    match personality {
        GirlPersonality::Cheerful => {
            // メイドのヘッドドレス
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.5, 0.1, 0.3))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(1.0, 1.0, 1.0),
                    metallic: 0.0,
                    perceptual_roughness: 0.8,
                    ..default()
                })),
                Transform::from_xyz(0.0, 2.1, 0.0),
            ));
            
            // フリル装飾
            for x in [-0.2, 0.2] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.08, 0.08, 0.08))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: colors.outfit_accent,
                        ..default()
                    })),
                    Transform::from_xyz(x, 2.12, 0.1),
                ));
            }
        },
        
        GirlPersonality::Sweet => {
            // ピンクのリボン
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.4, 0.15, 0.1))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: colors.special,
                    metallic: 0.1,
                    perceptual_roughness: 0.7,
                    ..default()
                })),
                Transform::from_xyz(0.0, 2.0, 0.2),
            ));
            
            // リボンの中心
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.12, 0.12, 0.12))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: colors.hair,
                    emissive: LinearRgba::rgb(0.1, 0.05, 0.1),
                    ..default()
                })),
                Transform::from_xyz(0.0, 2.0, 0.25),
            ));
        },
        
        GirlPersonality::Cool => {
            // クールなヘアピン
            for x in [-0.15, 0.15] {
                parent.spawn((
                    Mesh3d(meshes.add(Cuboid::new(0.06, 0.1, 0.03))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: colors.special,
                        metallic: 0.7,
                        perceptual_roughness: 0.2,
                        ..default()
                    })),
                    Transform::from_xyz(x, 1.95, 0.25),
                ));
            }
        }
    }
}

// === アニメーションシステム ===
pub fn animate_voxel_girls(
    time: Res<Time>,
    mut girls: Query<(&mut VoxelGirl, &mut Transform, &Children)>,
    mut child_transforms: Query<&mut Transform, (With<Parent>, Without<VoxelGirl>)>,
) {
    for (mut girl, mut girl_transform, children) in girls.iter_mut() {
        girl.animation_timer += time.delta_secs();
        girl.blink_timer += time.delta_secs();
        
        // 可愛いボブアニメーション
        let bob = ((girl.animation_timer + girl.bob_offset) * 2.0).sin() * 0.05;
        girl_transform.translation.y += bob * time.delta_secs();
        
        // 微妙な左右スウェイ
        let sway = ((girl.animation_timer + girl.bob_offset) * 1.5).cos() * 0.02;
        girl_transform.translation.x += sway * time.delta_secs();
        
        // 腕の可愛い振り
        if let Some(task) = &girl.current_task {
            // タスク中は特別なアニメーション
            let arm_swing = (girl.animation_timer * 3.0).sin() * 0.1;
            
            for &child in children.iter() {
                if let Ok(mut child_transform) = child_transforms.get_mut(child) {
                    // 腕の軽い振りアニメーション
                    if child_transform.translation.x.abs() > 0.4 { // 腕の位置を検出
                        child_transform.rotation = Quat::from_rotation_z(
                            arm_swing * if child_transform.translation.x > 0.0 { 1.0 } else { -1.0 }
                        );
                    }
                }
            }
        }
        
        // まばたき（瞬間的）
        if girl.blink_timer > 2.0 + (girl.bob_offset * 0.5).sin().abs() * 3.0 {
            girl.blink_timer = 0.0;
            // 瞬間的にまばたき（目のサイズを小さく）
            for &child in children.iter() {
                if let Ok(mut child_transform) = child_transforms.get_mut(child) {
                    // 目の位置を検出してまばたき効果
                    if child_transform.translation.y > 1.8 && child_transform.translation.z > 0.25 {
                        child_transform.scale.y = 0.1; // 瞬間的に縮める
                    }
                }
            }
        } else if girl.blink_timer > 0.1 {
            // まばたき終了 - 元に戻す
            for &child in children.iter() {
                if let Ok(mut child_transform) = child_transforms.get_mut(child) {
                    if child_transform.scale.y < 0.9 {
                        child_transform.scale.y = 1.0;
                    }
                }
            }
        }
    }
}