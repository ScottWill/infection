use config::Config;
use lazy_static::lazy_static;
use std::ops::RangeInclusive;

lazy_static! {
    static ref CONFIG: Config = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .unwrap();
}

pub struct Setting<T> {
    pub label: String,
    pub range: RangeInclusive<T>,
    pub value: T,
}

impl<T> Setting<T> {
    fn new(label: String, value: T, range: RangeInclusive<T>) -> Self {
        Self { label, range, value }
    }
}

pub struct Settings {
    pub death_chance: Setting<f32>,
    pub infection_rate: Setting<f32>,
    pub infection_time: Setting<f32>,
    pub particle_radius: Setting<f32>,
    pub population_size: Setting<usize>,
    pub particle_velocity: Setting<f32>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            death_chance: Setting::new(
                CONFIG.get("death_chance_label").unwrap(),
                CONFIG.get("death_chance_value").unwrap(),
                CONFIG.get("death_chance_range").unwrap()
            ),
            infection_rate: Setting::new(
                CONFIG.get("infection_rate_label").unwrap(),
                CONFIG.get("infection_rate_value").unwrap(),
                CONFIG.get("infection_rate_range").unwrap()
            ),
            infection_time: Setting::new(
                CONFIG.get("infection_time_label").unwrap(),
                CONFIG.get("infection_time_value").unwrap(),
                CONFIG.get("infection_time_range").unwrap()
            ),
            particle_radius: Setting::new(
                CONFIG.get("particle_radius_label").unwrap(),
                CONFIG.get("particle_radius_value").unwrap(),
                CONFIG.get("particle_radius_range").unwrap()
            ),
            population_size: Setting::new(
                CONFIG.get("population_size_label").unwrap(),
                CONFIG.get("population_size_value").unwrap(),
                CONFIG.get("population_size_range").unwrap()
            ),
            particle_velocity: Setting::new(
                CONFIG.get("particle_velocity_label").unwrap(),
                CONFIG.get("particle_velocity_value").unwrap(),
                CONFIG.get("particle_velocity_range").unwrap()
            ),
        }
    }
}