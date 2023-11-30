use rand::{random, Rng};

struct PowerDistributionSimulator {
    solar_i_v: f32,
    solar_i_c: f32,
    battery_v: f32,
    battery_c: f32,

    rail_3_v: f32,
    rail_3_c: f32,
    rail_5_v: f32,
    rail_5_c: f32,
    rail_12_v: f32,
    rail_12_c: f32,
}


impl PowerDistributionSimulator {
    // Constructor remains public
    pub fn new() -> PowerDistributionSimulator {
        let mut simulator = PowerDistributionSimulator {
            solar_i_v: 5.0,
            solar_i_c: 0.0,

            battery_v: 40.0,
            battery_c: 0.0,

            rail_3_v: 3.3,
            rail_5_v: 5.0,
            rail_12_v: 12.0,

            rail_3_c: 0.0,
            rail_5_c:0.0,
            rail_12_c:0.0,
        };
        // numbers i pulled out of my ass
        simulator.rail_3_c = simulator.random_float(0.0, 1.2);
        // numbers i pulled out of my ass
        simulator.rail_5_c = simulator.random_float(0.0, 0.9);
        // more numbers out of my ass
        simulator.rail_12_c = simulator.random_float(0.0, 30.0);

        // i cannot emphasize further that these numbers are magic as shit
        simulator.solar_i_c = 5.0;
        simulator.battery_c = 12.0;


        simulator
    }
    // I cant be fucked making 6 more classes to return V and A
    pub fn get_3_rail_power(&mut self) -> f32{
        self.rail_3_v * self.rail_3_c
    }
    pub fn get_5_rail_power(&mut self) -> f32{
        self.rail_5_v * self.rail_5_c
    }
    pub fn get_12_rail_power(&mut self) -> f32{
        self.rail_12_v * self.rail_12_c
    }
    pub fn get_battery_power(&mut self) -> f32{
        self.battery_v * self.battery_c
    }

    pub fn get_solar_power(&mut self) -> f32{
        self.solar_i_v * self.solar_i_c
    }

    fn random_float(&mut self, min_range:f32, max_range:f32) -> f32 {
        return (random::<f32>() * (max_range - min_range)) + min_range;
    }
}

// Usage example
fn main() {
    let mut connector = PowerDistributionSimulator::new();
    println!("{}w", connector.get_12_rail_power())
}