//Disable annoying warnings
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}


mod world;
use bevy::{app::AppExit, prelude::*, window::WindowResizeConstraints};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_pixels::prelude::*;
use world::*;

const WIDTH:usize=100;
const HEIGHT:usize=100;
const PIXEL_SIZE:usize=4;
const WINDOW_WIDTH:usize=WIDTH*PIXEL_SIZE;
const WINDOW_HEIGHT:usize=HEIGHT*PIXEL_SIZE;

fn main() {
    App::build()
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
    .add_plugin(PixelsPlugin)
    .add_startup_system(create_particle_world.system())
    .add_system(draw_world.system())
    .add_system(dump_sand.system())
    .add_system(change_title.system())
    .run();
    
}

fn draw_world(mut pixels_resource: ResMut<PixelsResource>, mut universe_query: Query<&mut Universe>) {
    let mut frame =pixels_resource.pixels.get_frame();


    
    for mut universe in universe_query.iter_mut() {
        let mut i=0;
        for x in 0..WIDTH{
            for y in 0..HEIGHT {
                let frame_i=i*4;
                let particle=universe.get_cell_at(x,y);
                let color=match particle.particle_type{
                    ParticleTypes::Air=>[0,0,0,0],
                    ParticleTypes::Wall=>[100,100,100,255],
                    ParticleTypes::Sand=>[194, 178, 128,255],
                };
                frame[frame_i]=color[0];
                frame[frame_i+1]=color[1];
                frame[frame_i+2]=color[2];
                frame[frame_i+3]=color[3];
                i+=1;
            }
        }       
    }


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