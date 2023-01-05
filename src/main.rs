use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

#[macro_use]
extern crate approx;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(ball_movement)
        .add_system(collision)
        .run();
}

struct Linear_Equation {
    b: f32,
    m: f32,
}

#[derive(Component)]
struct Ball {
    position: Vec2,
    previous_position: Vec2,
    x_change: f32,
    y_change: f32,
}

impl Ball {
    fn new() -> Self {
        Ball {
            x_change: 1.0,
            y_change: 1.0,
            position: Vec2::new(0.0, 0.0),
            previous_position: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(400.0, 0.0, 0.0),
                scale: Vec3::new(50.0, 100.0, 0.0),
                ..default()
            },
            ..default()
        },
        Collider,
        Player,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-400.0, 0.0, 0.0),
                scale: Vec3::new(50.0, 100.0, 0.0),
                ..default()
            },
            ..default()
        },
        Enemy,
        Collider,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(20.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        Ball::new(),
    ));
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<Player>>,
) {
    let mut transform = sprite_position.single_mut();
    if keyboard_input.pressed(KeyCode::Up) {
        transform.translation.y += 10.0
    }
    if keyboard_input.pressed(KeyCode::Down) {
        transform.translation.y -= 10.0
    }
}

fn ball_movement(mut sprite_position: Query<(&mut Ball, &mut Transform)>) {
    let (mut ball, mut transform) = sprite_position.single_mut();
    ball.previous_position = ball.position;
    transform.translation.x += 1.0 * ball.x_change;
    //transform.translation.y += 1.0 * ball.x_change;
    ball.position = transform.translation.truncate();
}

fn collision(
    mut ball_query: Query<(&mut Ball, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let (mut ball, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();
    for collider_tarnsform in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            collider_tarnsform.translation,
            ball_transform.scale.truncate(),
        );
        if let Some(collision) = collision {
            println!("collision {:?}", collision);
            match collision {
                Collision::Left | Collision::Right => ball.x_change *= -1.0,
                Collision::Top | Collision::Bottom => ball.y_change *= -1.0,
                Collision::Inside => (),
            }
        }
    }
}

fn create_linear_equation(a: Vec2, b: Vec2) -> Linear_Equation {
    let m = (b.y - a.y) / (b.x - a.x);
    let b = b.y - b.x * m;
    Linear_Equation { m, b }
}

fn predict_ball(ball: Ball, enemy_horizontal_position: f32) -> f32 {
    let linnear_equation = create_linear_equation(ball.previous_position, ball.position);
    linnear_equation.m * enemy_horizontal_position + linnear_equation.b
}

#[cfg(test)]
mod tests {
    use super::{create_linear_equation, predict_ball, Ball, Linear_Equation, Vec2};

    #[test]
    fn test_linear_equation() {
        let point1 = Vec2::new(1.0, 2.0);
        let point2 = Vec2::new(4.0, 5.0);

        let expected_linear_equation = Linear_Equation { b: 1.0, m: 1.0 };
        let result_linear_equation = create_linear_equation(point1, point2);

        assert_abs_diff_eq!(result_linear_equation.m, expected_linear_equation.m);
        assert_abs_diff_eq!(result_linear_equation.b, expected_linear_equation.b);
    }

    #[test]
    fn test_predict_ball() {
        let mut ball = Ball::new();
        ball.previous_position = Vec2::new(1.0, 2.0);
        ball.position = Vec2::new(4.0, 5.0);

        let prediction = predict_ball(ball, 10.0);
        let expected = 11.0;
        assert_abs_diff_eq!(prediction, expected);
    }
}
