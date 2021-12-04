//Disable annoying warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
mod world;
use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use world::*;

const WIDTH:usize=100;
const HEIGHT:usize=100;
const PIXEL_SIZE:usize=4;
const WINDOW_WIDTH:usize=WIDTH*PIXEL_SIZE;
const WINDOW_HEIGHT:usize=HEIGHT*PIXEL_SIZE;

fn main() {
    App::build()
    .insert_resource(ClearColor(Color::rgb(130.0/255.0, 163.0/255.0, 1.0)))
    .insert_resource(WindowDescriptor{
        title:"I am a window!".to_string(),
        width:(WINDOW_WIDTH as f32),
        height:(WINDOW_HEIGHT as f32),
        vsync:false,
        resizable:false,
        decorations:true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(create_particle_world.system())
    .add_system(dump_sand.system())
    .add_system(change_title.system())
    .run();
}

fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

}

fn dump_sand(mut universe_query: Query<&mut Universe>) {
    for mut universe in universe_query.iter_mut() {
        universe.set_cell_at(10,10,ParticleTypes::Sand);
        assert_eq!(universe.get_cell_at(10, 10).particle_type,ParticleTypes::Sand);
    }

}


fn create_particle_world(mut commands: Commands) {
    let mut universe = Universe::new(WIDTH,HEIGHT,PIXEL_SIZE);

    for x in 0..(universe.width as usize){
        for y in 0..5{
            universe.set_cell_at(x, y, ParticleTypes::Sand)
        }
    }
    //Quick tests to make sure that the sand was, most likely, correct placed.
    assert_eq!(universe.get_cell_at(0, 0).particle_type,ParticleTypes::Sand);
    assert_eq!(universe.get_cell_at(0, 4).particle_type,ParticleTypes::Sand);

    commands.spawn().insert(universe);
}