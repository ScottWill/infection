use std::collections::{HashMap, HashSet};

use crate::app::particle::Particle;
use nannou::prelude::*;
use rapier2d::{prelude::*, na::{Vector2, UnitComplex}, crossbeam::{self, channel::Receiver}};

pub struct BodyMeta {
    pub position: Vector2<f32>,
    pub radius: f32,
}

impl BodyMeta {

    pub fn new(radius: f32, position: Vector2<f32>) -> Self {
        Self {
            position,
            radius,
        }
    }

}

#[derive(Default)]
pub struct Physics {
    broad_phase: BroadPhase,
    ccd_solver: CCDSolver,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    multibody_joint_set: MultibodyJointSet,
    narrow_phase: NarrowPhase,
    physics_pipeline: PhysicsPipeline,
    query_pipeline: QueryPipeline,
    rigid_body_set: RigidBodySet,
}

impl Physics {

    pub fn new(rect: Rect) -> Self {
        let mut collider_set = ColliderSet::new();
        create_walls(&mut collider_set, rect);
        Self {
            collider_set,
            ..Default::default()
        }
    }

    pub fn add_particle(&mut self, rect: Rect, radius: f32, velocity: f32) -> Particle {
        let translation = random_position(rect);
        let linvel = random_vector(velocity);
        let rb = RigidBodyBuilder::dynamic()
            .can_sleep(false)
            .linvel(linvel)
            .translation(translation)
            .build();
        let coll = ColliderBuilder::ball(radius)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .friction(0.0)
            .restitution(1.0)
            .build();
        let parent_handle = self.rigid_body_set.insert(rb);
        let handle = self.collider_set.insert_with_parent(coll, parent_handle, &mut self.rigid_body_set);
        Particle::new(handle)
    }

    pub fn get_body_meta(&self, handle: ColliderHandle) -> Option<BodyMeta> {
        if let Some(coll) = self.collider_set.get(handle) {
            if let Some(shape) = coll.shape().as_shape::<Ball>() {
                return Some(BodyMeta::new(shape.radius, coll.position().translation.vector));
            }
        }
        None
    }

    pub fn remove(&mut self, handle: ColliderHandle) {
        if let Some(coll) = self.collider_set.get(handle) {
            self.rigid_body_set.remove(
                coll.parent().unwrap(),
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true
            );
        }
    }

    pub fn update(
        &mut self,
        particles: &HashMap<ColliderHandle, Particle>,
        gravity: &Vector2<f32>,
        infection_rate: f32,
        dt: f32
    ) -> HashSet<ColliderHandle> {

        let (collision_event_sender, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_event_sender, _) = crossbeam::channel::unbounded();
        let events = ChannelEventCollector::new(collision_event_sender, contact_force_event_sender);

        self.integration_parameters.dt = dt;
        self.physics_pipeline.step(
            gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &events
        );

        self.get_infections(&collision_recv, particles, infection_rate)

    }

    fn get_infections(
        &self,
        collision_recv: &Receiver<CollisionEvent>,
        particles: &HashMap<ColliderHandle, Particle>,
        infection_rate: f32
    ) -> HashSet<ColliderHandle> {
        let mut infections = HashSet::new();
        while let Ok(event) = collision_recv.try_recv() {
            if let Some(p1) = particles.get(&event.collider1()) {
                if let Some(p2) = particles.get(&event.collider2()) {
                    if p1.infected() && p2.can_be_infected() && random_f32() <= infection_rate {
                        infections.insert(p2.handle());
                    }
                    else if p2.infected() && p1.can_be_infected() && random_f32() <= infection_rate {
                        infections.insert(p1.handle());
                    }
                }
            }
        }
        infections
    }

}

fn create_walls(collider_set: &mut ColliderSet, rect: Rect) {
    let half_height = rect.h() * 0.5;
    let half_width = rect.w() * 0.5;
    let thickness = 25.0;
    new_wall(collider_set, vector![0.0, -half_height - thickness], half_width * 2.2, thickness);
    new_wall(collider_set, vector![0.0, half_height + thickness], half_width * 2.2, thickness);
    new_wall(collider_set, vector![-half_width - thickness, 0.0], thickness, half_height * 2.2);
    new_wall(collider_set, vector![half_width + thickness, 0.0], thickness, half_height * 2.2);
}

fn new_wall(collider_set: &mut ColliderSet, position: Vector2<f32>, width: f32, height: f32) {
    let wall = ColliderBuilder::cuboid(width, height)
        .restitution_combine_rule(CoefficientCombineRule::Max)
        .restitution(1.0)
        .friction(0.0)
        .translation(position)
        .build();
    collider_set.insert(wall);
}

fn random_position(rect: Rect) -> Vector2<f32> {
    Vector2::new(
        random_range(rect.left(), rect.right()),
        random_range(rect.top(), rect.bottom())
    )
}

fn random_vector(velocity: f32) -> Vector2<f32> {
    UnitComplex::new(random_range(0.0, TAU)) * Vector2::x() * velocity
}