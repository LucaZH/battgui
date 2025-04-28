use battery::{Battery, State as BatteryState, units::energy::watt_hour};
use std::time::Duration;
#[derive(Debug)]
pub struct BatteryInfo {
    pub name: String,
    pub state: BatteryState,
    pub technology: String,
    pub voltage: f32,
    pub percentage: f32,
    pub energy: f32,
    pub energy_full: f32,
    pub energy_full_design: f32,
    pub energy_rate: f32,
    pub time_to_empty: Option<Duration>,
    pub time_to_full: Option<Duration>,
    pub cycle_count: Option<u32>,
    pub temperature: Option<f32>,
    pub model: String,
}

impl BatteryInfo {
    pub fn from_battery(battery: &Battery) -> Self {
        Self {
            name: format!("Battery {}", battery.vendor().unwrap_or("Unknown")),
            state: battery.state(),
            technology: format!("{:?}", battery.technology()),
            voltage: battery.voltage().value,
            percentage: battery.state_of_charge().value * 100.0,
            energy: battery.energy().get::<watt_hour>(),
            energy_full: battery.energy_full().get::<watt_hour>(),
            energy_full_design: battery.energy_full_design().get::<watt_hour>(),
            energy_rate: battery.energy_rate().value,
            time_to_empty: battery
                .time_to_empty()
                .map(|q| Duration::from_secs_f32(q.value)),
            time_to_full: battery
                .time_to_full()
                .map(|q| Duration::from_secs_f32(q.value)),
            cycle_count: battery.cycle_count(),
            temperature: battery.temperature().map(|t| t.value),
            model: battery.model().unwrap_or("Unknown").to_string(),
        }
    }
}
