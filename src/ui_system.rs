use bevy::prelude::*;

#[derive(Component)]
pub struct TaskListUI;

#[derive(Component)]
pub struct TaskText;

pub fn setup_ui(mut commands: Commands) {
    
    // タスクリストのUIコンテナ
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            width: Val::Px(300.0),
            height: Val::Px(200.0),
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.9)),
        TaskListUI,
    )).with_children(|parent| {
        // タイトル
        parent.spawn((
            Text::new("FISH CAKES"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        ));
        
        // タスクリスト
        let tasks = vec![
            "◇ Get White Belly",
            "◇ Chop",
            "◇ Fry",
            "◇ Bake",
        ];
        
        for (i, task) in tasks.iter().enumerate() {
            parent.spawn((
                Text::new(*task),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(if i == 0 { 
                    Color::srgb(0.3, 0.8, 1.0) // アクティブなタスクを青色で
                } else { 
                    Color::srgb(0.7, 0.7, 0.7) // 未完了は灰色
                }),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
                TaskText,
            ));
        }
    });
}

pub fn update_task_ui(
    kitchen_state: Res<crate::KitchenState>,
    mut task_texts: Query<(&mut TextColor, &mut Text), With<TaskText>>,
) {
    if let Some(recipe) = kitchen_state.current_recipes.get(0) {
        for (i, (mut color, mut text)) in task_texts.iter_mut().enumerate() {
            if i < recipe.current_step {
                // 完了したタスクは緑色
                *color = TextColor(Color::srgb(0.2, 0.8, 0.2));
                text.0 = format!("✓ {}", get_task_name(i));
            } else if i == recipe.current_step {
                // 現在のタスクは青色
                *color = TextColor(Color::srgb(0.3, 0.8, 1.0));
                text.0 = format!("◇ {}", get_task_name(i));
            } else {
                // 未来のタスクは灰色
                *color = TextColor(Color::srgb(0.5, 0.5, 0.5));
                text.0 = format!("◇ {}", get_task_name(i));
            }
        }
    }
}

fn get_task_name(index: usize) -> &'static str {
    match index {
        0 => "Get White Belly",
        1 => "Chop",
        2 => "Fry",
        3 => "Bake",
        _ => "Unknown",
    }
}