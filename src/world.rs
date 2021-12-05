use std::*;
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
pub struct ParticleCell{
    pub particle_type: ParticleTypes,
    pub particle_timer:i8
}

impl ParticleCell{
    pub fn update(&self){
        match self.particle_type{
            ParticleTypes::Air=>{}
            ParticleTypes::Wall=>{}
            ParticleTypes::Sand=>{}
        }
    }
}


pub struct Universe{
    pub particles: Vec<ParticleCell>,
    pub width:usize,
    pub height:usize,
    pub pixel_size:usize
}


impl<'a> Universe{
    pub fn new(width:usize, height:usize,pixel_size:usize)->Universe{
        let particles = (0..width * height).map(|_i| ParticleCell{particle_type:ParticleTypes::Air,particle_timer:0}).collect();
        Universe{
            particles,
            width,
            height,
            pixel_size
        }
    }

    fn update_cell(particle_cell:ParticleCell){
        particle_cell.update()
    }

    pub fn set_cell_at(&mut self, x:usize,y:usize,new_cell:ParticleTypes){
        let index = self.get_index(x, y);
        self.particles[index].particle_type=new_cell;
    }
    pub fn get_index(&mut self,x:usize,y:usize)->usize{
        x*self.width+y
    }

    pub fn get_cell_at(&mut self,x:usize,y:usize)->ParticleCell{
        let index = self.get_index(x, y);
        self.particles[index]
    }
}
