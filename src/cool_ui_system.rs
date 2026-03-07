use bevy::prelude::*;

#[derive(Component)]
pub struct CoolTaskListUI;

#[derive(Component)]
pub struct TaskProgressBar;

#[derive(Component)]
pub struct TaskIcon;

#[derive(Component)]
pub struct CompletionCounter;

#[derive(Component)]
pub struct ChefStatusPanel;

#[derive(Component)]
pub struct GlowEffect;

#[derive(Component)]
pub struct AnimatedElement {
    pub start_time: f32,
    pub animation_type: AnimationType,
}

#[derive(Clone)]
pub enum AnimationType {
    Pulse,
    Slide,
    Glow,
    Rotate,
}

pub fn setup_cool_ui(mut commands: Commands, time: Res<Time>) {
    // メインUIコンテナ（グラデーション背景）
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(30.0),
            top: Val::Px(30.0),
            width: Val::Px(400.0),
            height: Val::Px(280.0),
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(3.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.06, 0.12, 0.98)), // より濃いブルーグレー
        BorderColor(Color::srgba(0.1, 0.7, 1.0, 0.9)), // より明るいネオンブルー
        CoolTaskListUI,
        AnimatedElement {
            start_time: time.elapsed_secs(),
            animation_type: AnimationType::Glow,
        },
    )).with_children(|parent| {
        // ヘッダー部分
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                margin: UiRect::bottom(Val::Px(15.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.2, 0.4, 0.3)),
            BorderColor(Color::srgba(0.4, 0.7, 1.0, 0.6)),
        )).with_children(|header| {
            // タイトル
            header.spawn((
                Text::new("◤🍳 FISH CAKE KITCHEN ◥"),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::srgba(0.1, 0.9, 1.0, 1.0)), // シアンブルー
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                AnimatedElement {
                    start_time: time.elapsed_secs(),
                    animation_type: AnimationType::Pulse,
                },
            ));
            
            // 完成数カウンター
            header.spawn((
                Text::new("COMPLETED: 0"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgba(0.3, 0.9, 0.3, 1.0)), // 明るいグリーン
                CompletionCounter,
            ));
        });
        
        // タスクリスト部分
        spawn_task_list(parent, &time);
        
        // シェフステータス部分
        spawn_chef_status_panel(parent, &time);
    });
    
    // サイドパネル（右上）
    spawn_side_panel(&mut commands, &time);
}

fn spawn_task_list(parent: &mut ChildBuilder, time: &Res<Time>) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    )).with_children(|task_container| {
        let tasks = [
            ("🐟", "Get White Belly", Color::srgba(0.3, 0.8, 1.0, 1.0)),
            ("🔪", "Chop Ingredients", Color::srgba(0.7, 0.7, 0.7, 1.0)),
            ("🍳", "Fry in Pan", Color::srgba(0.7, 0.7, 0.7, 1.0)),
            ("🔥", "Bake in Oven", Color::srgba(0.7, 0.7, 0.7, 1.0)),
        ];
        
        for (i, (icon, task_name, text_color)) in tasks.iter().enumerate() {
            spawn_fancy_task_item(task_container, i, icon, task_name, *text_color, time);
        }
    });
}

fn spawn_fancy_task_item(
    parent: &mut ChildBuilder,
    index: usize,
    icon: &str,
    task_name: &str,
    text_color: Color,
    time: &Res<Time>
) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(40.0),
            margin: UiRect::bottom(Val::Px(8.0)),
            padding: UiRect::all(Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.0)),
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(if index == 0 {
            Color::srgba(0.05, 0.25, 0.45, 0.7) // よりビビッドなアクティブ
        } else {
            Color::srgba(0.02, 0.02, 0.08, 0.6) // より濃い非アクティブ
        }),
        BorderColor(if index == 0 {
            Color::srgba(0.2, 0.9, 1.0, 1.0) // 強力なブルーグロー
        } else {
            Color::srgba(0.2, 0.2, 0.3, 0.5) // ダークグレー
        }),
        AnimatedElement {
            start_time: time.elapsed_secs() + index as f32 * 0.1,
            animation_type: AnimationType::Slide,
        },
    )).with_children(|task_row| {
        // アイコン
        task_row.spawn((
            Text::new(icon),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 0.9, 0.3, 1.0)), // ゴールド
            Node {
                margin: UiRect::right(Val::Px(15.0)),
                ..default()
            },
            TaskIcon,
        ));
        
        // タスク名
        task_row.spawn((
            Text::new(task_name),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(text_color),
            Node {
                flex_grow: 1.0,
                ..default()
            },
        ));
        
        // プログレスインジケータ
        if index == 0 {
            task_row.spawn((
                Node {
                    width: Val::Px(60.0),
                    height: Val::Px(6.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.3, 0.0, 0.8)),
                BorderColor(Color::srgba(0.3, 0.8, 1.0, 0.6)),
                TaskProgressBar,
                AnimatedElement {
                    start_time: time.elapsed_secs(),
                    animation_type: AnimationType::Pulse,
                },
            )).with_children(|progress_bar| {
                // プログレスフィル
                progress_bar.spawn((
                    Node {
                        width: Val::Percent(65.0), // 65%完了
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.9, 0.3, 1.0)),
                ));
            });
        }
    });
}

fn spawn_chef_status_panel(parent: &mut ChildBuilder, time: &Res<Time>) {
    parent.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(80.0),
            padding: UiRect::all(Val::Px(12.0)),
            border: UiRect::all(Val::Px(1.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.15, 0.08, 0.4)), // ダークグリーン
        BorderColor(Color::srgba(0.3, 0.9, 0.3, 0.6)),
        ChefStatusPanel,
    )).with_children(|chef_panel| {
        chef_panel.spawn((
            Text::new("👨‍🍳 CHEFS STATUS"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgba(0.3, 0.9, 0.3, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(8.0)),
                ..default()
            },
        ));
        
        // シェフステータス行
        chef_panel.spawn((
            Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        )).with_children(|status_row| {
            let chef_statuses = [
                ("Chef A", "Getting Ingredients", Color::srgba(0.3, 0.8, 1.0, 1.0)),
                ("Chef B", "Standby", Color::srgba(0.7, 0.7, 0.7, 1.0)),
                ("Chef C", "Standby", Color::srgba(0.7, 0.7, 0.7, 1.0)),
            ];
            
            for (chef_name, status, color) in chef_statuses.iter() {
                status_row.spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                )).with_children(|chef_col| {
                    chef_col.spawn((
                        Text::new(*chef_name),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
                    ));
                    
                    chef_col.spawn((
                        Text::new(*status),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(*color),
                    ));
                });
            }
        });
    });
}

fn spawn_side_panel(commands: &mut Commands, time: &Res<Time>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(30.0),
            top: Val::Px(30.0),
            width: Val::Px(200.0),
            height: Val::Px(120.0),
            padding: UiRect::all(Val::Px(15.0)),
            border: UiRect::all(Val::Px(2.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.05, 0.1, 0.9)), // ダークレッド
        BorderColor(Color::srgba(1.0, 0.3, 0.5, 0.8)), // ピンクグロー
        AnimatedElement {
            start_time: time.elapsed_secs() + 0.5,
            animation_type: AnimationType::Glow,
        },
    )).with_children(|side_panel| {
        side_panel.spawn((
            Text::new("⚡ KITCHEN STATS"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 0.8, 0.9, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));
        
        let stats = [
            ("Efficiency", "85%"),
            ("Speed", "Fast"),
            ("Quality", "★★★★☆"),
        ];
        
        for (label, value) in stats.iter() {
            side_panel.spawn((
                Node {
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            )).with_children(|stat_row| {
                stat_row.spawn((
                    Text::new(*label),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
                ));
                
                stat_row.spawn((
                    Text::new(*value),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.3, 1.0, 0.3, 1.0)),
                ));
            });
        }
    });
}

pub fn update_cool_ui(
    kitchen_state: Res<crate::KitchenState>,
    mut completion_counter: Query<&mut Text, (With<CompletionCounter>, Without<TaskProgressBar>)>,
    mut task_colors: Query<&mut TextColor, Without<CompletionCounter>>,
    time: Res<Time>,
) {
    // 完成数カウンター更新
    if let Ok(mut counter_text) = completion_counter.get_single_mut() {
        counter_text.0 = format!("COMPLETED: {}", kitchen_state.fish_cakes_made);
    }
    
    // タスクの色更新（現在のステップに基づいて）
    if let Some(recipe) = kitchen_state.current_recipes.get(0) {
        // タスクのアクティブ状態を更新する処理
        // （実装簡略化のため、基本的な色変更のみ）
    }
}

pub fn animate_ui_elements(
    mut animated_elements: Query<(&mut Node, &AnimatedElement, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs();
    
    for (mut node, animation, mut bg_color) in &mut animated_elements {
        let elapsed = current_time - animation.start_time;
        
        match animation.animation_type {
            AnimationType::Pulse => {
                let pulse = (elapsed * 3.0).sin().abs();
                let alpha = 0.3 + pulse * 0.2;
                bg_color.0.set_alpha(alpha);
            },
            AnimationType::Glow => {
                let glow = ((elapsed * 2.0).sin() + 1.0) * 0.5;
                let intensity = 0.1 + glow * 0.15;
                bg_color.0 = Color::srgba(
                    bg_color.0.red() + intensity,
                    bg_color.0.green() + intensity,
                    bg_color.0.blue() + intensity,
                    bg_color.0.alpha()
                );
            },
            AnimationType::Slide => {
                let slide = (elapsed * 4.0).sin() * 2.0;
                node.left = Val::Px(slide);
            },
            AnimationType::Rotate => {
                // 回転エフェクト（UIには適用が難しいため、基本的な揺れ効果）
                let shake = (elapsed * 10.0).sin() * 1.0;
                node.left = Val::Px(shake);
            }
        }
    }
}