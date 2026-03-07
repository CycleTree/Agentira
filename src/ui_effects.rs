use bevy::prelude::*;

#[derive(Component)]
pub struct ParticleEffect {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct FloatingText {
    pub start_pos: Vec2,
    pub target_pos: Vec2,
    pub progress: f32,
}

#[derive(Component)]
pub struct UIGlow {
    pub intensity: f32,
    pub color_shift: f32,
}

pub fn spawn_completion_effect(commands: &mut Commands, position: Vec2) {
    // 完成時の花火エフェクト
    for i in 0..12 {
        let angle = (i as f32 / 12.0) * 2.0 * std::f32::consts::PI;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * 100.0;
        
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(position.x),
                top: Val::Px(position.y),
                width: Val::Px(4.0),
                height: Val::Px(4.0),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 0.8, 0.2, 1.0)),
            ParticleEffect {
                lifetime: 0.0,
                max_lifetime: 2.0,
                velocity,
            },
        ));
    }
}

pub fn spawn_floating_text(commands: &mut Commands, text: String, position: Vec2, color: Color) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            ..default()
        },
        Text::new(text),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(color),
        FloatingText {
            start_pos: position,
            target_pos: position + Vec2::new(0.0, -50.0),
            progress: 0.0,
        },
    ));
}

pub fn update_particle_effects(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Node, &mut ParticleEffect, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (entity, mut node, mut particle, mut color) in &mut particles {
        particle.lifetime += time.delta_secs();
        
        if particle.lifetime >= particle.max_lifetime {
            commands.entity(entity).despawn();
            continue;
        }
        
        // パーティクルの位置更新
        if let (Val::Px(ref mut x), Val::Px(ref mut y)) = (&mut node.left, &mut node.top) {
            *x += particle.velocity.x * time.delta_secs();
            *y += particle.velocity.y * time.delta_secs();
            
            // 重力効果
            particle.velocity.y += 200.0 * time.delta_secs();
        }
        
        // フェードアウト
        let alpha = 1.0 - (particle.lifetime / particle.max_lifetime);
        color.0.set_alpha(alpha);
    }
}

pub fn update_floating_text(
    mut commands: Commands,
    mut floating_texts: Query<(Entity, &mut Node, &mut FloatingText, &mut TextColor)>,
    time: Res<Time>,
) {
    for (entity, mut node, mut floating_text, mut color) in &mut floating_texts {
        floating_text.progress += time.delta_secs() * 0.8;
        
        if floating_text.progress >= 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        
        // イージング付きの移動
        let ease_progress = 1.0 - (1.0 - floating_text.progress).powi(3);
        let current_pos = floating_text.start_pos.lerp(floating_text.target_pos, ease_progress);
        
        node.left = Val::Px(current_pos.x);
        node.top = Val::Px(current_pos.y);
        
        // フェードアウト
        let alpha = 1.0 - floating_text.progress;
        color.0.set_alpha(alpha);
    }
}

pub fn update_ui_glow_effects(
    mut ui_elements: Query<(&mut BackgroundColor, &UIGlow)>,
    time: Res<Time>,
) {
    let time_factor = time.elapsed_secs();
    
    for (mut bg_color, glow) in &mut ui_elements {
        let pulse = ((time_factor * 2.0 + glow.color_shift).sin() + 1.0) * 0.5;
        let glow_intensity = glow.intensity * pulse;
        
        // グローエフェクトを背景色に追加
        let base_color = bg_color.0;
        bg_color.0 = Color::srgba(
            (base_color.red() + glow_intensity * 0.3).min(1.0),
            (base_color.green() + glow_intensity * 0.5).min(1.0),
            (base_color.blue() + glow_intensity * 1.0).min(1.0),
            base_color.alpha(),
        );
    }
}