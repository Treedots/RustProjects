use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
    core::FixedTimestep
};
use rand::prelude::random;

const CELL_SIZE : f32 = 20.;

fn main(){
    App::new()
        .insert_resource(WindowDescriptor { // <--
            title: "Snake!".to_string(), // <--
            width: 500.0,                 // <--
            height: 500.0,                // <--
            ..default()         // <--
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.10))
                .with_system(snake_movement)
                .with_system(spawn_food)
        )
        .run();
}

fn setup_camera(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct  SnakeHead{
    speed: i32,
    current_step: i32,
    direction: SnakeHeadDirection
}

#[derive(Component)]
#[derive(PartialEq)]
enum SnakeHeadDirection{
    Up,
    Down,
    Left,
    Right
}

impl SnakeHeadDirection {
    fn opposite(&self)-> Self{
        match self{
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
}

#[derive(Component)]
struct Food;

const SNAKE_HEAD_COLOR: Color =  Color::rgb(0.25,0.75,0.75);
const FOOD_COLOR: Color =  Color::rgb(1.,0.,0.);

fn random_grid() -> f32 {
    let r: f32 = 500./CELL_SIZE as f32;
    let c = random::<f32>() * r*2. - r;
    c.round() as f32 * CELL_SIZE
}

fn spawn_food(mut commands: Commands){

    commands.spawn_bundle(SpriteBundle{
        sprite:Sprite{
            color: FOOD_COLOR,
            ..default()
        },
        transform:Transform{
            scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
            translation: Vec3::new(random_grid()  ,random_grid() ,0.),
            ..default()
        },
        ..default()
    })
    .insert(Food);
}

fn spawn_snake(mut commands: Commands){
    commands.spawn_bundle(SpriteBundle{
        sprite: Sprite{
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(CELL_SIZE, CELL_SIZE, CELL_SIZE),
            translation: Vec3::new(random_grid()  ,random_grid() ,0.),
            ..default()
        },
        ..default()
    })
    .insert(SnakeHead{
        speed: 1,
        current_step: 0,
        direction: SnakeHeadDirection::Up
    })
    ;
}

fn snake_movement(
    mut head_positions: Query<(&mut SnakeHead,&mut Transform)>,
    keys: Res<Input<KeyCode>>
){
    if let Some((mut head, mut transform)) = head_positions.iter_mut().next() {
        let t: SnakeHeadDirection = head.direction.opposite();
        // Need to use if else
        if keys.any_pressed([KeyCode::A,KeyCode::Left]) &&  t != SnakeHeadDirection::Left{
            head.direction = SnakeHeadDirection::Left;
        }
        else if keys.any_pressed([KeyCode::D, KeyCode::Right]) &&  t !=  SnakeHeadDirection::Right{
            head.direction = SnakeHeadDirection::Right;
        }
        else if keys.any_pressed([KeyCode::W,KeyCode::Up]) && t !=  SnakeHeadDirection::Up{
            head.direction = SnakeHeadDirection::Up;
        }
        else if keys.any_pressed([KeyCode::S,KeyCode::Down]) && t !=  SnakeHeadDirection::Down{
            head.direction = SnakeHeadDirection::Down;
        } 
        
        head.current_step += head.speed;
        if head.current_step >= 10{
            match head.direction{
                SnakeHeadDirection::Up => transform.translation.y += CELL_SIZE,
                SnakeHeadDirection::Down => transform.translation.y -= CELL_SIZE,
                SnakeHeadDirection::Left => transform.translation.x -= CELL_SIZE,
                SnakeHeadDirection::Right => transform.translation.x += CELL_SIZE,
            }
            head.current_step -= 10
        }  
        
    }

}