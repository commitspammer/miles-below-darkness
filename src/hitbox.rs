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
    mut event_writer: EventWriter<Collision>,
    query: Query<(Entity, &Hitbox, &Transform)>
) {
    let entities: Vec<(Entity, &Hitbox, &Transform)> = query.iter().collect();
    for (i, (entity_a, hitbox_a, transform_a)) in entities.iter().enumerate() {
        for (entity_b, hitbox_b, transform_b) in entities.iter().skip(i + 1) {
            //temporary AABB algo, soon will be SAT 
            let a_min_x = transform_a.translation.x - hitbox_a.width / 2.0;
            let a_max_x = transform_a.translation.x + hitbox_a.width / 2.0;
            let b_min_x = transform_b.translation.x - hitbox_b.width / 2.0;
            let b_max_x = transform_b.translation.x + hitbox_b.width / 2.0;

            let a_min_y = transform_a.translation.y - hitbox_a.height / 2.0;
            let a_max_y = transform_a.translation.y + hitbox_a.height / 2.0;
            let b_min_y = transform_b.translation.y - hitbox_b.height / 2.0;
            let b_max_y = transform_b.translation.y + hitbox_b.height / 2.0;

            let overlap_x = a_min_x < b_max_x && a_max_x > b_min_x;
            let overlap_y = a_min_y < b_max_y && a_max_y > b_min_y;

            println!("{} {}", overlap_x, overlap_y);
            if overlap_x && overlap_y {
                event_writer.send(Collision(*entity_a, *entity_b));
            }
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
                transform: transform.clone(),
                ..default()
            },
            Debugbox,
        ));
    }
}
