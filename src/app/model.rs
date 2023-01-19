use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rapier2d::{na::Vector2, prelude::ColliderHandle};
use std::{collections::HashMap, cell::Ref, time::{Duration, Instant}};

use crate::utils::physics::Physics;
use super::{particle::Particle, settings::Settings};

pub struct Model {
    egui: Egui,
    particles: HashMap<ColliderHandle,Particle>,
    physics: Physics,
    reload_pending: bool,
    settings: Settings,
    window_rect: Rect,
}

impl Model {

    pub fn new(window: Ref<Window>) -> Self {
        Self {
            egui: Egui::from_window(&window),
            particles: HashMap::new(),
            physics: Physics::default(),
            reload_pending: true,
            settings: Settings::default(),
            window_rect: window.rect(),
        }
    }

    pub fn handle_raw_event(&mut self, event: &nannou::winit::event::WindowEvent) {
        self.egui.handle_raw_event(event);
    }

    pub fn update(&mut self, since_start: Duration, since_last: Duration) {
        if self.reload_pending {
            self.reload();
        }
        self.gui_update(since_start);
        self.physics_update(since_last.as_secs_f32());
    }

    fn reload(&mut self) {
        self.particles = HashMap::with_capacity(self.settings.population_size.value);
        self.physics = Physics::new(self.window_rect);

        let rect = self.window_rect.pad(self.settings.particle_radius.value * 5.0);
        for i in 0..self.settings.population_size.value {
            let mut particle = self.physics.add_particle(rect, self.settings.particle_radius.value, self.settings.particle_velocity.value);
            if i == 0 {
                particle.infect(self.settings.infection_time.value);
            }
            self.particles.insert(particle.handle(), particle);
        }

        self.reload_pending = false;
    }

    fn physics_update(&mut self, dt: f32) {
        let gravity: Vector2<f32> = Vector2::zeros();
        self.physics.update(&self.particles, &gravity, self.settings.infection_rate.value * 0.01, dt);

        let mut deaths = Vec::new();
        for (handle, particle) in &mut self.particles {
            if self.physics.infected(handle) {
                particle.infect(self.settings.infection_time.value);
            }
            particle.update(dt);
            if particle.infected() {
                if random_f32() <= self.settings.death_chance.value * 0.01 * dt {
                    deaths.push(*handle);
                }
            }
        }

        for death in deaths {
            self.particles.remove(&death);
            self.physics.remove(death);
        }
    }

    fn gui_update(&mut self, elapsed: Duration) {
        self.egui.set_elapsed_time(elapsed);
        let ctx = self.egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label(self.settings.population_size.label.clone());
            ui.add(egui::Slider::new(&mut self.settings.population_size.value, self.settings.population_size.range.clone()).logarithmic(true));
            ui.label(self.settings.infection_rate.label.clone());
            ui.add(egui::Slider::new(&mut self.settings.infection_rate.value, self.settings.infection_rate.range.clone()).suffix("%"));
            ui.label(self.settings.infection_time.label.clone());
            ui.add(egui::Slider::new(&mut self.settings.infection_time.value, self.settings.infection_time.range.clone()));
            ui.label(self.settings.death_chance.label.clone());
            ui.add(egui::Slider::new(&mut self.settings.death_chance.value, self.settings.death_chance.range.clone()).suffix("%"));
            ui.label(self.settings.particle_radius.label.clone());
            ui.add(egui::Slider::new(&mut self.settings.particle_radius.value, self.settings.particle_radius.range.clone()));
            ui.label(self.settings.particle_velocity.label.clone());
            ui.add(egui::Slider::new(&mut self.settings.particle_velocity.value, self.settings.particle_velocity.range.clone()));
            if ui.button("Generate").clicked() {
                self.reload_pending = true;
            }
            if ui.button("Reset Settings").clicked() {
                self.settings = Settings::default();
                self.reload_pending = true;
            }
        });
    }

    pub fn view(&self, app: &App, draw: &Draw, frame: Frame) {
        for (handle, particle) in &self.particles {
            if let Some(meta) = self.physics.get_body_meta(*handle) {
                draw.ellipse()
                    .color(particle.color())
                    .radius(meta.radius)
                    .xy(Vec2::from_slice(meta.position.as_slice()));
            }
        }
        draw.to_frame(&app, &frame).unwrap();
        self.egui.draw_to_frame(&frame).unwrap();
    }

}