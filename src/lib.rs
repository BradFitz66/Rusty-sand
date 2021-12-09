//Code untethered from any library (apart from the standard library, I suppose.)
use std::*;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParticleTypes{
    Air=0,
    Wall=1,
    Sand=2,
}


impl fmt::Display for ParticleTypes{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f,"{:?}",self)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point{
    pub x:u32,
    pub y:u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParticleCell{
    pub particle_type: ParticleTypes,
    pub particle_timer:u8,
    pub pos: Point
}

impl ParticleCell{
    pub fn update(&mut self, universe:&mut Universe,frame:&mut[u8]){
        match self.particle_type{
            ParticleTypes::Air=>{}
            ParticleTypes::Wall=>{}
            ParticleTypes::Sand=>update_sand(self,universe,frame)
        }
    }
}

impl Default for ParticleCell{
    fn default()->ParticleCell{
        ParticleCell{
            particle_type:ParticleTypes::Air,
            particle_timer:0,
            pos:Point{x:0,y:0}
        }
    }
}

pub struct Universe{
    pub particles: Vec<ParticleCell>,
    pub width:u32,
    pub height:u32,
    pub pixel_size:u32,
    pub universe_timer:u8,
    pub rng:StdRng
}

impl Default for Universe{
    fn default()->Universe{
        let particles = (0..100 * 100).map(|i| 
            ParticleCell{particle_type:ParticleTypes::Air,particle_timer:0,pos:Point{x:(i%100),y:(i/100)}}
        ).collect();
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Universe{
            particles,
            width:100,
            height:100,
            pixel_size:4,
            universe_timer:0,
            rng:rand::rngs::StdRng::seed_from_u64(since_the_epoch.as_millis() as u64)
        }

    }
}

impl Universe{
    pub fn new(width:u32, height:u32,pixel_size:u32)->Universe{
        let particles = (0..width * height).map(|i| 
            ParticleCell{particle_type:ParticleTypes::Air,particle_timer:0,pos:Point{x:(i%width),y:(i/width)}}
        ).collect();
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Universe{
            particles,
            width,
            height,
            pixel_size,
            universe_timer:0,
            rng:rand::rngs::StdRng::seed_from_u64(since_the_epoch.as_millis() as u64)
        }
    }

    pub fn set_cell_at(&mut self, x:u32,y:u32,new_cell:ParticleTypes,frame: &mut [u8]){
        let index:usize  = self.get_index(x, y) as usize;
        //I should probably store the actual color with the particle so I don't need this, but I cba.
        let color:[u8;4]=match new_cell{
            ParticleTypes::Air=>[0,0,0,255],
            ParticleTypes::Wall=>[100,100,100,255],
            ParticleTypes::Sand=>[194,178,128,255],
        };
        //Translate x and y to an index in the frame buffer.
        let frame_i =index*4;
        //Take the color array and put it into the frame buffer(overwriting the values already there)
        frame[frame_i..][..4].clone_from_slice(&color);
        self.particles[index].particle_type=new_cell;
    }
    pub fn get_index(&mut self,x:u32,y:u32)->u32{
        self.width.wrapping_mul(y).wrapping_add(x)
    }


    pub fn get_mutable_cell_at(&mut self, x:u32,y:u32)->&mut ParticleCell{
        let index:usize = self.get_index(x, y) as usize;

        if x < self.width && y < self.height{
            &mut self.particles[index]
        }
        else{
            panic!("Tried to get mutable outside of bounds");
        }
    }

    pub fn get_cell_at(&mut self,x:u32,y:u32)->ParticleCell{
        let index:usize = self.get_index(x, y) as usize;
        if x < self.width && y < self.height{
            self.particles[index]
        }
        else{
            ParticleCell{
                particle_timer:0,
                particle_type:ParticleTypes::Wall,
                pos:Point{x:x,y:y}
            }
        }
    }
}


pub fn update_sand(cell:&mut ParticleCell, universe:&mut Universe, frame: &mut [u8]){
    if cell.particle_timer.wrapping_sub(universe.universe_timer) == 1 {
        return;
    }
    let uni_timer = universe.universe_timer;
    let our_pos=cell.pos;
    let neighbours: [ParticleTypes;8]=[
        //Above/below
      universe.get_cell_at(our_pos.x,our_pos.y.wrapping_add(1)).particle_type, //0
      universe.get_cell_at(our_pos.x,our_pos.y.wrapping_sub(1)).particle_type, //1
        //Left/right
      universe.get_cell_at(our_pos.x.wrapping_add(1),our_pos.y).particle_type, //2
      universe.get_cell_at(our_pos.x.wrapping_sub(1),our_pos.y).particle_type, //3
        //Diagonals
      universe.get_cell_at(our_pos.x.wrapping_add(1),our_pos.y.wrapping_add(1) ).particle_type,//4
      universe.get_cell_at(our_pos.x.wrapping_sub(1),our_pos.y.wrapping_sub(1) ).particle_type,//5
      universe.get_cell_at(our_pos.x.wrapping_add(1),our_pos.y.wrapping_sub(1) ).particle_type,//6
      universe.get_cell_at(our_pos.x.wrapping_sub(1),our_pos.y.wrapping_add(1) ).particle_type,//7
    ];
    
    if neighbours[0]==ParticleTypes::Air {
        universe.set_cell_at(our_pos.x, our_pos.y, ParticleTypes::Air,frame);
        universe.set_cell_at(our_pos.x, our_pos.y.wrapping_add(1), cell.particle_type,frame);
        let mut new_cell=universe.get_mutable_cell_at(our_pos.x, our_pos.y.wrapping_add(1));
        new_cell.particle_timer=uni_timer.wrapping_add(1);  
    }
    else if neighbours[0]!=ParticleTypes::Air && neighbours[4]==ParticleTypes::Air && neighbours[7]==ParticleTypes::Air{
        //Have empty spaces to the bottom left and right. Choose a random direction.
        
        let dir_array:[i32;2] =[-1,1];
        let dir = dir_array.choose(&mut rand::thread_rng());

        
        universe.set_cell_at(our_pos.x, our_pos.y, ParticleTypes::Air,frame);
        match dir{
            Some(1) => {universe.set_cell_at(our_pos.x.wrapping_add(1), our_pos.y.wrapping_add(1), cell.particle_type,frame);}
            Some(-1) => {universe.set_cell_at(our_pos.x.wrapping_sub(1), our_pos.y.wrapping_add(1), cell.particle_type,frame);}
            _=>{}
        }

    }
    else if neighbours[0]!=ParticleTypes::Air && neighbours[4]!=ParticleTypes::Air && neighbours[7]==ParticleTypes::Air{
        //Empty space only to bottom right
        universe.set_cell_at(our_pos.x, our_pos.y, ParticleTypes::Air,frame);
        universe.set_cell_at(our_pos.x.wrapping_sub(1), our_pos.y.wrapping_add(1), cell.particle_type,frame);
        
    }
    else if neighbours[0]!=ParticleTypes::Air && neighbours[4]==ParticleTypes::Air && neighbours[7]!=ParticleTypes::Air{
        //Empty space only to bottom left
        universe.set_cell_at(our_pos.x, our_pos.y, ParticleTypes::Air,frame);
        universe.set_cell_at(our_pos.x.wrapping_add(1), our_pos.y.wrapping_add(1), cell.particle_type,frame);
        
    }

    
}