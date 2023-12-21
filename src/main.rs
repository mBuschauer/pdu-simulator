use std::time::{Instant};
use std::{thread, time};
use rand::Rng;

trait Component{
    fn get_name(& self) -> &String;
    fn get_power_draw(&mut self) -> f32;
    fn get_current(& mut self) -> f32;
    fn get_voltage(& mut self) -> f32;
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

        (3.0 / 5.0) * (((elapsed + rand::thread_rng().gen_range(-2..2) as f32) / 5.0).cos() + 1.0) + self.power_loss + 0.3
    }

    fn get_current(&mut self) -> f32 {
        self.get_power_draw() / self.rail_voltage
    }

    fn get_voltage(&mut self) -> f32{
        self.rail_voltage
    }
}

struct FlightComputer{
    component_name: String,
    rail_voltage: f32,
    power_loss: f32,
    booting_power: f32,
    nominal_power: f32,
    last_update: Instant,
    start_up_time: f32,
    bbb_connected: bool,
    ethernet_enabled: bool,

}

impl FlightComputer{
    pub fn new() -> FlightComputer {
        FlightComputer {
            component_name: String::from("Command and Data Handling"),
            rail_voltage: 5.0,
            power_loss: 0.105,
            booting_power: 1.75,
            nominal_power: 1.75,
            last_update: Instant::now(),
            start_up_time: 5.0, // in seconds
            bbb_connected: true,
            ethernet_enabled: true,

        }
    }

    pub fn enable_components(& mut self, state: bool) -> (){
        self.bbb_connected = state;
        self.ethernet_enabled = state;
    }

}

impl Component for FlightComputer {
    fn get_name(&self) -> &String{
        &self.component_name
    }

    fn get_power_draw(&mut self) -> f32 {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f32(); // Get the elapsed time in seconds
        let mut power_draw: f32;

        // Determine if the system booting
        if elapsed <= self.start_up_time {
            power_draw = self.booting_power;
        } else {
            power_draw = self.nominal_power;
        }

        // Add the power loss
        power_draw += self.power_loss;

        // If BBB and Ethernet are enabled, use the max power draw, otherwise reduce it
        if self.bbb_connected && self.ethernet_enabled {
            power_draw
        } else {
            // Assume BBB and Ethernet together account for 0.350A, which at 5V is 1.75W
            power_draw - 1.75
        }
    }

    fn get_current(&mut self) -> f32 {
        self.get_power_draw() / self.rail_voltage
    }

    fn get_voltage(&mut self) -> f32{
        self.rail_voltage
    }
}

struct Heaters {
    component_name: String,
    rail_voltage: f32,
    power_loss_per_heater: f32,
    power_draw_per_heater: f32,
    heater_1_on: bool,
    heater_2_on: bool,
}

impl Heaters {
    pub fn new() -> Heaters {
        Heaters {
            component_name: String::from("Heaters"),
            rail_voltage: 12.0,
            power_loss_per_heater: 0.175,
            power_draw_per_heater: 2.5, // Half of total as there are two heaters
            heater_1_on: false, // Assuming heaters are off by default
            heater_2_on: false, // Assuming heaters are off by default
        }
    }

    pub fn on_heater_1(&mut self, state: bool) {
        self.heater_1_on = state;
    }
    pub fn on_heater_2(&mut self, state: bool) {
        self.heater_2_on = state;
    }

}

impl Component for Heaters {
    fn get_name(&self) -> &String {
        &self.component_name
    }

    fn get_power_draw(&mut self) -> f32 {
        let mut power: f32 = 0.0;
        if self.heater_1_on {
            power += self.power_draw_per_heater + self.power_loss_per_heater
        }
        if self.heater_2_on{
            power += self.power_draw_per_heater + self.power_loss_per_heater
        }
        power
    }

    fn get_current(&mut self) -> f32 {
        // Current = Power / Voltage
        self.get_power_draw() / self.rail_voltage
    }

    fn get_voltage(&mut self) -> f32 {
        self.rail_voltage
    }
}
struct Communication{
    component_name: String,
    uhf_rail_voltage: f32,
    s_rail_voltage: f32,
    uhf_power_loss: Vec<f32>,
    s_power_loss: Vec<f32>,
    uhf_state: bool, // true is transmit, false is receive (standby)
    s_state: bool, // true is transmit, false is receive (standby)
}

impl Communication{
    pub fn new() -> Communication {
        Communication {
            component_name: String::from("Communication"),
            uhf_rail_voltage: 3.3,
            s_rail_voltage: 12.0,
            uhf_power_loss: vec![0.8, 0.025], // transmit, receive
            s_power_loss: vec![0.84, 0.007], // transmit, receive
            uhf_state: false,
            s_state: false,
        }
    }
    // true is transmit, false is receive (standby)
    pub fn enable_s_band(& mut self, state: bool) {
        self.s_state = state;
    }
    // true is transmit, false is receive (standby)
    pub fn enable_uhf_band(& mut self, state: bool) {
        self.uhf_state = state;

    }

    pub fn get_uhf_power(& mut self) -> f32{
        let power: f32;

        if self.uhf_state{
            // if transmitting
            power = 8.0 + self.uhf_power_loss[0];
        }
        else{
            power = 0.25 + self.uhf_power_loss[1];
        }
        power
    }

    pub fn get_s_power(& mut self) -> f32{
        let power: f32;

        if self.s_state{
            // if transmitting
            power = 12.0 + self.s_power_loss[0];
        }
        else{
            power = 0.1 + self.s_power_loss[1];
        }
        power
    }

    pub fn get_s_voltage(&mut self) -> f32{
        self.s_rail_voltage
    }
    pub fn get_uhf_voltage(&mut self) -> f32{
        self.uhf_rail_voltage
    }
    pub fn get_uhf_current(&mut self) -> f32 {
        // Current = Power / Voltage
        self.get_uhf_power() / self.uhf_rail_voltage
    }
    pub fn get_s_current(&mut self) -> f32 {
        // Current = Power / Voltage
        self.get_s_power() / self.s_rail_voltage
    }
}

impl Component for Communication{
    fn get_name(&self) -> &String {
        &self.component_name
    }

    fn get_power_draw(&mut self) -> f32 {
        self.get_s_power() + self.get_uhf_power()
    }

    fn get_current(&mut self) -> f32 {
        // Current = Power / Voltage
        (self.get_s_power() / self.s_rail_voltage) + (self.get_uhf_power() + self.uhf_rail_voltage)
    }

    fn get_voltage(&mut self) -> f32 {
        // use get_s_voltage() or get_uhf_voltage()
        0.0
    }
}


struct Navigation{
    component_name: String,
    gps_rail_voltage: f32,
    maneuvering_rail_voltage: f32,
    gps_power_loss: f32,
    maneuvering_power_loss: Vec<f32>,
    gps_state: bool, // true is on, false is off (should always be on)
    maneuvering_state: bool, // true is maneuvering, false is "passive
}

impl Navigation{
    pub fn new() -> Navigation{
        Navigation{
            component_name: String::from("Navigation"),
            gps_rail_voltage: 5.0,
            maneuvering_rail_voltage: 5.0, // this might not be correct 
            gps_power_loss: 0.18,
            maneuvering_power_loss: vec![0.2614, 0.0887], // maneuvering, passive
            gps_state: true,
            maneuvering_state: false,
        }
    }

    // true is transmit, false is receive (standby)
    pub fn enable_gps(& mut self, state: bool) {
        self.gps_state = state;
    }
    // true is transmit, false is receive (standby)
    pub fn enable_maneuvering(& mut self, state: bool) {
        self.maneuvering_state = state;

    }

    pub fn get_gps_power(& mut self) -> f32{
        if self.gps_state{
            return 1.8 + self.gps_power_loss;
        }
        else{
            return 0.0
        }
    }

    pub fn get_maneuvering_power(& mut self) -> f32{
        let power: f32;

        if self.maneuvering_state{
            // if transmitting
            power = 3.72 + self.maneuvering_power_loss[0];
        }
        else{
            power = 1.27 + self.maneuvering_power_loss[1];
        }
        power
    }

    pub fn get_gps_voltage(&mut self) -> f32{
        self.gps_rail_voltage
    }
    pub fn get_maneuvering_voltage(&mut self) -> f32{
        self.maneuvering_rail_voltage
    }
    pub fn get_gps_current(&mut self) -> f32 {
        // Current = Power / Voltage
        self.get_gps_power() / self.gps_rail_voltage
    }
    pub fn get_maneuvering_current(&mut self) -> f32 {
        // Current = Power / Voltage
        self.get_maneuvering_power() / self.maneuvering_rail_voltage
    }


}

impl Component for Navigation{
    fn get_name(&self) -> &String {
        &self.component_name
    }

    fn get_power_draw(&mut self) -> f32 {
        self.get_gps_power() + self.get_maneuvering_power()
    }

    fn get_current(&mut self) -> f32 {
        // Current = Power / Voltage
        (self.get_gps_power() / self.gps_rail_voltage) + (self.get_maneuvering_power() + self.maneuvering_rail_voltage)
    }

    fn get_voltage(&mut self) -> f32 {
        // use get_gps_voltage() or get_maneuvering_voltage()
        0.0
    }
}

struct PowerSupply{
    camera: EventCamera,
    flight_computer: FlightComputer,
    heater_components: Heaters,
    communication_computer: Communication,
    navigation_computer: Navigation,
}

impl PowerSupply{
    pub fn new() -> PowerSupply{
        PowerSupply{
            camera: EventCamera::new(),
            flight_computer: FlightComputer::new(),
            heater_components: Heaters::new(),
            communication_computer: Communication::new(),
            navigation_computer: Navigation::new()
        }
    }
    pub fn get_power_draw(& mut self) -> f32{
        0.636 + self.camera.get_power_draw() + self.flight_computer.get_power_draw() + self.heater_components.get_power_draw() + self.communication_computer.get_power_draw() + self.navigation_computer.get_maneuvering_power()
    }

}

fn main() {
    let mut psu = PowerSupply::new();
    // println!("Event Camera Power Draw: {}W", psu.camera.get_power_draw());

    // println!("Flight Computer Power Draw: {}W", psu.flight_computer.get_power_draw());
    psu.flight_computer.enable_components(true);
    // println!("Flight Computer Power Draw: {}W", psu.flight_computer.get_power_draw());

    // println!("Heaters Power Draw: {}W", psu.heater_components.get_power_draw());
    psu.heater_components.on_heater_1(true);
    // println!("Heaters Power Draw: {}W", psu.heater_components.get_power_draw());
    psu.heater_components.on_heater_2(true);
    // println!("Heaters Power Draw: {}W", psu.heater_components.get_power_draw());

    // println!("Communications Power Draw: {}W", psu.communication_computer.get_power_draw());
    psu.communication_computer.enable_s_band(true);
    // println!("Communications Current: {}A", psu.communication_computer.get_current());
    psu.communication_computer.enable_uhf_band(true);
    // println!("Communications Power Draw: {}W", psu.communication_computer.get_power_draw());

    // println!("Navigations Power Draw: {}W", psu.navigation_computer.get_power_draw());
    psu.navigation_computer.enable_maneuvering(true);
    // println!("Navigations Power Draw: {}W", psu.navigation_computer.get_power_draw());

    while true{
        let ten_millis = time::Duration::from_millis(100);
        // println!("Camera Power Draw: {}W", psu.camera.get_power_draw());
        println!("Total Power Draw: {}W", psu.get_power_draw());

        thread::sleep(ten_millis);
    }    

}
