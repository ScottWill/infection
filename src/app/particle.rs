use nannou::prelude::*;
use rapier2d::prelude::ColliderHandle;

pub struct Particle {
    handle: ColliderHandle,
    immune: bool,
    infected: f32,
}

impl Particle {

    pub fn new(handle: ColliderHandle) -> Self {
        Self {
            handle,
            immune: false,
            infected: 0.0
        }
    }

    pub fn can_be_infected(&self) -> bool {
        !self.immune && self.infected == 0.0
    }

    pub fn color(&self) -> Rgb<u8> {
        match self.immune {
            true => BLUE,
            false => match self.infected() {
                true  => RED,
                false => GREEN,
            }
        }
    }

    pub fn handle(&self) -> ColliderHandle {
        self.handle
    }

    pub fn infect(&mut self, time: f32) {
        if !self.immune {
            self.infected = time;
        }
    }

    pub fn infected(&self) -> bool {
        self.infected > 0.0
    }

    pub fn update(&mut self, dt: f32) {
        if self.infected() {
            self.infected -= dt;
            if self.infected <= 0.0 {
                self.immune = true;
                self.infected = 0.0;
            }
        }
    }

}