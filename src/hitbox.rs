use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::gamestate::GameState;

pub struct HitboxPlugin;
impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Collision>()
            .add_systems(Update, draw_debug_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, read_event_debug_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, collision_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, collide_system.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Hitbox {
    //relative_transform: Transform,
    pub width: f32,
    pub height: f32,
    colliding: bool,
}

#[derive(Component)]
struct Debugbox;

#[derive(Event)]
struct Collision(Entity, Entity);

impl Hitbox {
    pub fn new(w: f32, h: f32) -> Hitbox {
        Hitbox { width: w, height: h, colliding: false }
    }

    pub fn intersects(&self, _h: &Hitbox) -> bool {
        unimplemented!();
    }
}

fn collision_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_writer: EventWriter<Collision>,
    query: Query<(Entity, &Hitbox, &Transform)>
) {
    let entities: Vec<(Entity, &Hitbox, &Transform)> = query.iter().collect();
    for (i, (entity_a, hitbox_a, transform_a)) in entities.iter().enumerate() {
        for (entity_b, hitbox_b, transform_b) in entities.iter().skip(i + 1) {
            //temporary AABB algo, soon will be SAT 
            let a_min_x = - hitbox_a.width  / 2.0;
            let a_max_x =   hitbox_a.width  / 2.0;
            let a_min_y = - hitbox_a.height / 2.0;
            let a_max_y =   hitbox_a.height / 2.0;

            let b_min_x = - hitbox_b.width  / 2.0;
            let b_max_x =   hitbox_b.width  / 2.0;
            let b_min_y = - hitbox_b.height / 2.0;
            let b_max_y =   hitbox_b.height / 2.0;

            let vertices = (
                Vec2::new(a_min_x, a_min_y),
                Vec2::new(a_max_x, a_min_y),
                Vec2::new(a_min_x, a_max_y),
                Vec2::new(a_max_x, a_max_y),

                Vec2::new(b_min_x, b_min_y),
                Vec2::new(b_max_x, b_min_y),
                Vec2::new(b_min_x, b_max_y),
                Vec2::new(b_max_x, b_max_y),
            );

            let (_, angle_a) = transform_a.rotation.to_axis_angle();
            let (_, angle_b) = transform_b.rotation.to_axis_angle();

            let rotated_vertices_a = vec![
                Vec2::new(
                    vertices.0.x * angle_a.cos() - vertices.0.y * angle_a.sin(),
                    vertices.0.y * angle_a.cos() + vertices.0.x * angle_a.sin()),
                Vec2::new(
                    vertices.1.x * angle_a.cos() - vertices.1.y * angle_a.sin(),
                    vertices.1.y * angle_a.cos() + vertices.1.x * angle_a.sin()),
                Vec2::new(
                    vertices.2.x * angle_a.cos() - vertices.2.y * angle_a.sin(),
                    vertices.2.y * angle_a.cos() + vertices.2.x * angle_a.sin()),
                Vec2::new(
                    vertices.3.x * angle_a.cos() - vertices.3.y * angle_a.sin(),
                    vertices.3.y * angle_a.cos() + vertices.3.x * angle_a.sin()),
            ];
            let rotated_vertices_b = vec![
                Vec2::new(
                    vertices.4.x * angle_b.cos() - vertices.4.y * angle_b.sin(),
                    vertices.4.y * angle_b.cos() + vertices.4.x * angle_b.sin()),
                Vec2::new(
                    vertices.5.x * angle_b.cos() - vertices.5.y * angle_b.sin(),
                    vertices.5.y * angle_b.cos() + vertices.5.x * angle_b.sin()),
                Vec2::new(
                    vertices.6.x * angle_b.cos() - vertices.6.y * angle_b.sin(),
                    vertices.6.y * angle_b.cos() + vertices.6.x * angle_b.sin()),
                Vec2::new(
                    vertices.7.x * angle_b.cos() - vertices.7.y * angle_b.sin(),
                    vertices.7.y * angle_b.cos() + vertices.7.x * angle_b.sin()),
            ];

            let a_min_x = rotated_vertices_a.iter().map(|v| v.x).reduce(f32::min).unwrap() + transform_a.translation.x;
            let a_max_x = rotated_vertices_a.iter().map(|v| v.x).reduce(f32::max).unwrap() + transform_a.translation.x;
            let a_min_y = rotated_vertices_a.iter().map(|v| v.y).reduce(f32::min).unwrap() + transform_a.translation.y;
            let a_max_y = rotated_vertices_a.iter().map(|v| v.y).reduce(f32::max).unwrap() + transform_a.translation.y;

            let b_min_x = rotated_vertices_b.iter().map(|v| v.x).reduce(f32::min).unwrap() + transform_b.translation.x;
            let b_max_x = rotated_vertices_b.iter().map(|v| v.x).reduce(f32::max).unwrap() + transform_b.translation.x;
            let b_min_y = rotated_vertices_b.iter().map(|v| v.y).reduce(f32::min).unwrap() + transform_b.translation.y;
            let b_max_y = rotated_vertices_b.iter().map(|v| v.y).reduce(f32::max).unwrap() + transform_b.translation.y;

            let overlap_x = a_min_x < b_max_x && a_max_x > b_min_x;
            let overlap_y = a_min_y < b_max_y && a_max_y > b_min_y;

            if overlap_x && overlap_y {
                event_writer.send(Collision(*entity_a, *entity_b));
            }

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(a_max_x - a_min_x, a_max_y - a_min_y)).into(),
                    material: materials.add(Color::RED),
                    transform: Transform::from_xyz((a_max_x + a_min_x) / 2.0, (a_max_y + a_min_y) / 2.0, 0.0),
                    ..default()
                },
                Debugbox,
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::new(b_max_x - b_min_x, b_max_y - b_min_y)).into(),
                    material: materials.add(Color::RED),
                    transform: Transform::from_xyz((b_max_x + b_min_x) / 2.0, (b_max_y + b_min_y) / 2.0, 0.0),
                    ..default()
                },
                Debugbox,
            ));
        }
    }
}

fn collide_system(
    mut event_reader: EventReader<Collision>,
    mut query: Query<(Entity, &mut Hitbox)>
) {
    for event in event_reader.read() {
        for (entity, mut hitbox) in query.iter_mut() {
            if entity == event.0 || entity == event.1 {
                hitbox.colliding = true;
            }
        }
    }
}

static mut COUNTER: u64 = 0;

fn read_event_debug_system(
    mut event_reader: EventReader<Collision>,
) {
    for event in event_reader.read() {
        unsafe {
            println!("Entity {:?} and {:?} have collided! ({})", event.0, event.1, COUNTER);
            COUNTER += 1;
        }
    }
}

fn draw_debug_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hitbox_query: Query<(&Hitbox, &Transform)>,
    debugbox_query: Query<Entity, With<Debugbox>>,
) {
    for entity in debugbox_query.iter() {
        commands.entity(entity).despawn();
    }
    for (hitbox, transform) in hitbox_query.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(hitbox.width, hitbox.height)).into(),
                material: materials.add(if hitbox.colliding { Color::BLUE } else { Color::GREEN }),
                transform: Transform {
                    translation: transform.translation.clone(),
                    rotation: transform.rotation.clone(),
                    ..default()
                },
                ..default()
            },
            Debugbox,
        ));
    }
}
