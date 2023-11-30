use std::time::{Instant};
use rand::{random};

trait Component{
    fn get_name(& self) -> &String;
    fn get_power_draw(&mut self) -> f32;
    fn get_current(& mut self) -> f32;
    fn get_voltage(& mut self) -> f32;
    fn random_float(&mut self, min_range:f32, max_range:f32) -> f32 {
        return (random::<f32>() * (max_range - min_range)) + min_range;
    }
}

struct EventCamera {
    component_name: String,
    rail_voltage: f32,
    power_loss: f32,
    last_update: Instant,
}

impl EventCamera {
    pub fn new() -> EventCamera {
        EventCamera {
            component_name: String::from("Event Camera"),
            rail_voltage: 5.0,
            power_loss: 0.09,
            last_update: Instant::now(),
        }
    }

}
impl Component for EventCamera {
    fn get_name(&self) -> &String{
        &self.component_name
    }

    fn get_power_draw(&mut self) -> f32 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f32(); // Get the elapsed time in seconds
        self.last_update = now; // Update the last update time

        (3.0 / 5.0) * ((elapsed / 5.0).cos() + 1.0) + self.power_loss + 0.3
    }

    fn get_current(&mut self) -> f32 {
        self.get_power_draw() / self.rail_voltage
    }

    fn get_voltage(&mut self) -> f32{
        self.rail_voltage
    }
}

fn main() {
    let mut camera = EventCamera::new();
    println!("{}w", camera.get_power_draw());
}
