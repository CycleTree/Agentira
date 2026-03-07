use bevy::prelude::*;

mod cool_ui_system;
mod ui_effects;
mod cute_agents;
use cool_ui_system::*;
use ui_effects::*;
use cute_agents::*;

// Fish Cake Kitchen - AI Cooking Simulation Game
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fish Cake Kitchen - AI Cooking Simulation".into(),
                resolution: (1200., 800.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(KitchenState::default())
        .add_systems(Startup, (setup_kitchen, setup_cool_ui))
        .add_systems(Update, (
            chef_behavior,
            cooking_system,
            task_system,
            camera_controls,
            update_cool_ui,
            animate_ui_elements,
            update_particle_effects,
            update_floating_text,
            update_ui_glow_effects,
            animate_cute_chefs,
        ))
        .run();
}

// === ゲーム状態 ===

#[derive(Resource, Default)]
struct KitchenState {
    fish_cakes_made: u32,
    current_recipes: Vec<Recipe>,
}

#[derive(Clone, Debug)]
struct Recipe {
    name: String,
    steps: Vec<CookingStep>,
    current_step: usize,
    completed: bool,
}

#[derive(Clone, Debug)]
enum CookingStep {
    GetIngredient(String),
    Chop,
    Fry,
    Bake,
}

// === コンポーネント ===

// 旧Chef構造体 - CuteChefに置き換え済み
/*
#[derive(Component)]
struct Chef {
    speed: f32,
    current_task: Option<CookingStep>,
    carrying_item: Option<String>,
    target_station: Option<Entity>,
}
*/

#[derive(Component)]
struct CookingStation {
    station_type: StationType,
    in_use: bool,
    contents: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
enum StationType {
    Refrigerator,
    Counter,
    Stove,
    Oven,
    Sink,
    ServingArea,
}

#[derive(Component)]
struct KitchenCamera;

#[derive(Component)]
struct Ingredient {
    name: String,
    processed: bool,
}

// === セットアップ ===

fn setup_kitchen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut kitchen_state: ResMut<KitchenState>,
) {
    // アイソメトリックカメラ設定
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        KitchenCamera,
    ));
    
    // ライティング（温かみのある室内光）
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 0.9, 0.8),
            illuminance: 25000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.4, -0.3, 0.0))
    ));
    
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.9, 0.85, 0.8),
        brightness: 0.6,
    });
    
    // キッチンレイアウト作成
    spawn_kitchen_layout(&mut commands, &mut meshes, &mut materials);
    
    // AI料理人スポーン（可愛い3人のシェフ、異なるパーソナリティ）
    let personalities = [
        (ChefPersonality::Energetic, "Chef Akira", Vec3::new(-2.0, 0.5, -3.0)),
        (ChefPersonality::Gentle, "Chef Sakura", Vec3::new(0.0, 0.5, -3.0)),
        (ChefPersonality::Cool, "Chef Rei", Vec3::new(2.0, 0.5, -3.0)),
    ];
    
    for (personality, name, position) in personalities {
        spawn_cute_chef(
            &mut commands, 
            &mut meshes, 
            &mut materials, 
            position,
            name.to_string(),
            personality
        );
    }
    
    // フィッシュケーキレシピの初期化
    kitchen_state.current_recipes.push(Recipe {
        name: "Fish Cakes".to_string(),
        steps: vec![
            CookingStep::GetIngredient("White Belly".to_string()),
            CookingStep::Chop,
            CookingStep::Fry,
            CookingStep::Bake,
        ],
        current_step: 0,
        completed: false,
    });
}

fn spawn_kitchen_layout(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // 床材質
    let floor_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.85, 0.8),
        metallic: 0.0,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    // キッチン床（8x8）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(16.0, 0.2, 16.0))),
        MeshMaterial3d(floor_material),
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));
    
    // 冷蔵庫
    spawn_refrigerator(commands, meshes, materials, Vec3::new(-6.0, 1.0, -6.0));
    
    // カウンター（調理台）
    spawn_counter(commands, meshes, materials, Vec3::new(0.0, 0.5, -4.0));
    
    // コンロ
    spawn_stove(commands, meshes, materials, Vec3::new(4.0, 0.5, -6.0));
    
    // オーブン
    spawn_oven(commands, meshes, materials, Vec3::new(6.0, 0.5, -4.0));
    
    // シンク
    spawn_sink(commands, meshes, materials, Vec3::new(-4.0, 0.5, -4.0));
    
    // 配膳エリア
    spawn_serving_area(commands, meshes, materials, Vec3::new(0.0, 0.5, 4.0));
}

// 旧spawn_chef関数 - spawn_cute_chefに置き換え済み
/*
fn spawn_chef(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    _name: String,
) {
    // 旧コードコメントアウト - 新しいキュートエージェントシステムを使用
}
*/

// === 各キッチン設備のスポーン関数 ===

fn spawn_refrigerator(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let fridge_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 0.95), // 薄いグレー
        metallic: 0.1,
        perceptual_roughness: 0.3,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 2.0, 1.0))),
        MeshMaterial3d(fridge_material),
        Transform::from_translation(position),
        CookingStation {
            station_type: StationType::Refrigerator,
            in_use: false,
            contents: vec!["White Belly".to_string(), "Fish".to_string()],
        },
    ));
}

fn spawn_counter(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let counter_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6), // 木材色
        metallic: 0.0,
        perceptual_roughness: 0.9,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(3.0, 1.0, 1.5))),
        MeshMaterial3d(counter_material),
        Transform::from_translation(position),
        CookingStation {
            station_type: StationType::Counter,
            in_use: false,
            contents: vec![],
        },
    ));
}

fn spawn_stove(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let stove_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3), // 黒
        metallic: 0.8,
        perceptual_roughness: 0.2,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.5))),
        MeshMaterial3d(stove_material),
        Transform::from_translation(position),
        CookingStation {
            station_type: StationType::Stove,
            in_use: false,
            contents: vec![],
        },
    ));
}

fn spawn_oven(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let oven_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.4, 0.4, 0.4), // ダークグレー
        metallic: 0.6,
        perceptual_roughness: 0.3,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.5, 1.5, 1.2))),
        MeshMaterial3d(oven_material),
        Transform::from_translation(position),
        CookingStation {
            station_type: StationType::Oven,
            in_use: false,
            contents: vec![],
        },
    ));
}

fn spawn_sink(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let sink_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.9), // ステンレス
        metallic: 0.9,
        perceptual_roughness: 0.1,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.5))),
        MeshMaterial3d(sink_material),
        Transform::from_translation(position),
        CookingStation {
            station_type: StationType::Sink,
            in_use: false,
            contents: vec![],
        },
    ));
}

fn spawn_serving_area(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let serving_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.8, 0.7), // 明るい木材
        metallic: 0.0,
        perceptual_roughness: 0.8,
        ..default()
    });
    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(4.0, 1.0, 2.0))),
        MeshMaterial3d(serving_material),
        Transform::from_translation(position),
        CookingStation {
            station_type: StationType::ServingArea,
            in_use: false,
            contents: vec![],
        },
    ));
}

// === システム ===

// 簡略化されたChef behavior（CuteChef用）
fn chef_behavior(
    time: Res<Time>,
    mut chefs: Query<(&mut Transform, &mut CuteChef)>,
    stations: Query<(Entity, &Transform, &CookingStation), (Without<CuteChef>,)>,
    kitchen_state: Res<KitchenState>,
) {
    // 可愛いシェフたちのシンプルな行動
    for (mut transform, mut chef) in &mut chefs {
        chef.animation_timer += time.delta_secs();
        
        // 現在のタスクがない場合、新しいタスクを設定
        if chef.current_task.is_none() {
            if let Some(recipe) = kitchen_state.current_recipes.get(0) {
                if !recipe.completed && recipe.current_step < recipe.steps.len() {
                    let step_name = match &recipe.steps[recipe.current_step] {
                        CookingStep::GetIngredient(ingredient) => format!("Getting {}", ingredient),
                        CookingStep::Chop => "Chopping".to_string(),
                        CookingStep::Fry => "Frying".to_string(),
                        CookingStep::Bake => "Baking".to_string(),
                    };
                    chef.current_task = Some(step_name);
                }
            }
        }
        
        // タスクがある場合は対応するステーションに向かう
        if let Some(task) = &chef.current_task {
            let target_station = if task.contains("Getting") {
                StationType::Refrigerator
            } else if task.contains("Chopping") {
                StationType::Counter
            } else if task.contains("Frying") {
                StationType::Stove
            } else if task.contains("Baking") {
                StationType::Oven
            } else {
                StationType::Counter
            };
            
            if let Some((_, station_transform, _)) = stations
                .iter()
                .find(|(_, _, station)| station.station_type == target_station)
            {
                let direction = (station_transform.translation - transform.translation).normalize_or_zero();
                transform.translation += direction * chef.speed * 0.5 * time.delta_secs();
                
                if transform.translation.distance(station_transform.translation) < 2.0 {
                    chef.current_task = None;
                }
            }
        } else {
            // タスクがない時は可愛い巡回動作
            let patrol_movement = (chef.animation_timer * 0.3).sin() * 0.8 * time.delta_secs();
            transform.translation.x += patrol_movement;
            transform.translation.z += (chef.animation_timer * 0.2).cos() * 0.5 * time.delta_secs();
        }
    }
}

fn cooking_system(
    // 料理プロセスの管理
    mut kitchen_state: ResMut<KitchenState>,
    mut commands: Commands,
    chefs: Query<&CuteChef>,
) {
    // すべてのシェフがタスクを完了したら次のステップに進む
    let active_tasks: Vec<_> = chefs.iter().filter_map(|chef| chef.current_task.as_ref()).collect();
    
    if active_tasks.is_empty() {
        let mut completed_recipes = 0;
        
        for recipe in &mut kitchen_state.current_recipes {
            if !recipe.completed {
                recipe.current_step += 1;
                if recipe.current_step >= recipe.steps.len() {
                    recipe.completed = true;
                    completed_recipes += 1;
                    
                    // 完成エフェクト発動！
                    spawn_completion_effect(&mut commands, Vec2::new(400.0, 200.0));
                    spawn_floating_text(
                        &mut commands,
                        "🍳 FISH CAKE COMPLETE! +10 XP".to_string(),
                        Vec2::new(300.0, 150.0),
                        Color::srgba(1.0, 0.8, 0.2, 1.0)
                    );
                }
            }
        }
        
        // 借用を分離して競合を回避
        kitchen_state.fish_cakes_made += completed_recipes;
        
        // 新しいレシピ開始
        if completed_recipes > 0 {
            kitchen_state.current_recipes.clear();
            kitchen_state.current_recipes.push(Recipe {
                name: "Fish Cakes".to_string(),
                steps: vec![
                    CookingStep::GetIngredient("White Belly".to_string()),
                    CookingStep::Chop,
                    CookingStep::Fry,
                    CookingStep::Bake,
                ],
                current_step: 0,
                completed: false,
            });
        }
    }
}

fn task_system(
    // タスクリストのUIとロジック
    kitchen_state: Res<KitchenState>,
) {
    // コンソールに現在のタスク状況を出力（デバッグ用）
    if let Some(recipe) = kitchen_state.current_recipes.get(0) {
        if !recipe.completed {
            println!("Current Recipe: {} - Step {}/{}", 
                recipe.name, 
                recipe.current_step + 1, 
                recipe.steps.len()
            );
            if recipe.current_step < recipe.steps.len() {
                println!("Current Task: {:?}", recipe.steps[recipe.current_step]);
            }
        }
    }
}

fn camera_controls(
    mut camera: Query<&mut Transform, With<KitchenCamera>>,
    time: Res<Time>,
) {
    // アイソメトリックカメラの軽い回転
    for mut transform in &mut camera {
        let rotation_speed = 0.02;
        let angle = time.elapsed_secs() * rotation_speed;
        let distance = 20.0;
        let height = 12.0;
        
        transform.translation = Vec3::new(
            angle.cos() * distance,
            height,
            angle.sin() * distance,
        );
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}