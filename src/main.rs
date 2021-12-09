#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#[deny(clippy::int_plus_one)]


//Modules
mod lib;

//Imports
use lib::Universe;
use bevy::{app::AppExit, prelude::*, window::WindowResizeConstraints};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_pixels::prelude::*;

//Constants
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const PIXEL_SIZE:u32 = 4;
const WINDOW_WIDTH: u32 = WIDTH * PIXEL_SIZE;
const WINDOW_HEIGHT: u32 = HEIGHT * PIXEL_SIZE;


#[derive(Default)]
struct MouseState{
    x:f32,
    y:f32
}

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
    .insert_resource(PixelsOptions {
        width: WIDTH,
        height: HEIGHT,
    })    
    .add_plugins(DefaultPlugins)
    .add_plugin(PixelsPlugin)
    .add_startup_system_to_stage(StartupStage::PostStartup, dump_sand.system())
    .add_startup_system(create_particle_world.system())
    .add_system(set_pixels_size.system())
    .add_system(change_title.system())
    .add_system(update_universe.system())
    .add_system(painting.system().config(|params|{
        params.4 = Some(MouseState{
            x:0.0,
            y:0.0
        })
    }))
    .run();
    
}

fn set_pixels_size(mut pixels_resource: ResMut<PixelsResource>){
    pixels_resource.pixels.resize_surface(WINDOW_WIDTH,WINDOW_HEIGHT);
}

fn painting(buttons: Res<Input<MouseButton>>, 
            mut pixels_resource: ResMut<PixelsResource>,
            mut cursor_evr: EventReader<CursorMoved>,
            mut universe_query: Query<&mut Universe>,
            mut state: Local<MouseState>) 
{
    let mut frame = pixels_resource.pixels.get_frame();
    for ev in cursor_evr.iter() {
        state.x = ev.position.x;
        state.y = ev.position.y;
        println!(
            "New cursor position: X: {}, Y: {}, in Window ID: {:?}",
            ev.position.x, ev.position.y, ev.id
        );
    }

    if buttons.pressed(MouseButton::Left) {
        println!("Left pressed");
        for mut universe in universe_query.iter_mut() {
            universe.set_cell_at((state.x as u32)/PIXEL_SIZE,(state.y as u32)/PIXEL_SIZE,lib::ParticleTypes::Sand,frame);
        }     
    }
}

fn update_universe(mut universe_query: Query<&mut Universe>,mut pixels_resource: ResMut<PixelsResource>){
    for mut universe in universe_query.iter_mut() {
        universe.universe_timer=universe.universe_timer.wrapping_add(1);
        for x in 0..universe.width{
            for y in 0..universe.height{
                universe.get_cell_at(x,y).update(&mut *universe,pixels_resource.pixels.get_frame());
            }
        }
    }
}

fn change_title(time: Res<Time>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
}

fn dump_sand(mut universe_query: Query<&mut Universe>, mut pixels_resource: ResMut<PixelsResource>) {
    let mut frame =pixels_resource.pixels.get_frame();
    let lines=2;
    for mut universe in universe_query.iter_mut() {
        for x in 0..universe.width{
            for y in 0..lines{
                universe.set_cell_at(x,y,lib::ParticleTypes::Sand,frame);
            }
        }
    }
}


fn create_particle_world(mut commands: Commands) {
    let mut universe = Universe::new(WIDTH,HEIGHT,PIXEL_SIZE);
    commands.spawn().insert(universe);
}