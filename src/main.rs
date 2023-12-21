use std::time::{Instant};

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

        (3.0 / 5.0) * ((elapsed / 5.0).cos() + 1.0) + self.power_loss + 0.3
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

    pub fn disable_components(& mut self) -> (){
        self.bbb_connected = false;
        self.ethernet_enabled = false;
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

struct Heater {
    component_name: String,
    rail_voltage: f32,
    power_loss: f32,
    power_draw_per_heater: f32,
    operational: bool,
}

impl Heater {
    pub fn new() -> Heater {
        Heater {
            component_name: String::from("Heater"),
            rail_voltage: 12.0,
            power_loss: 0.175,
            power_draw_per_heater: 2.5, // Half of total as there are two heaters
            operational: false, // Assuming heaters are off by default
        }
    }

    pub fn set_operational(&mut self, state: bool) {
        self.operational = state;
    }
}

impl Component for Heater {
    fn get_name(&self) -> &String {
        &self.component_name
    }

    fn get_power_draw(&mut self) -> f32 {
        if self.operational {
            self.power_draw_per_heater + self.power_loss
        } else {
            0.0
        }
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

fn main() {
    let mut camera = EventCamera::new();
    println!("Event Camera Power Draw: {}W", camera.get_power_draw());

    let mut f_computer = FlightComputer::new();
    println!("Flight Computer Power Draw: {}W", f_computer.get_power_draw());
    f_computer.disable_components();
    println!("Flight Computer Power Draw: {}W", f_computer.get_power_draw());

    let mut heater = Heater::new();
    println!("Heater Power Draw: {}W", heater.get_power_draw());
    heater.set_operational(true);
    println!("Heater Power Draw: {}W", heater.get_power_draw());
    println!("Event Camera Power Draw: {}W", camera.get_power_draw());

    let mut communication_comm = Communication::new();
    println!("Communications Power Draw: {}W", communication_comm.get_power_draw());
    communication_comm.enable_s_band(true);
    println!("Communications Current: {}A", communication_comm.get_current());
    communication_comm.enable_uhf_band(true);
    println!("Communications Power Draw: {}W", communication_comm.get_power_draw());



}
