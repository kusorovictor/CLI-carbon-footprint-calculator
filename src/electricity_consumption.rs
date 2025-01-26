use core::num;
use std::fmt::format;

use crate::{electricity_consumption, input};

pub struct ElectricityConsumption {
    power_rating: f64,         // in watts
    usage_time: f64,           // in hours per day
    pub carbon_footprint: f64, // in kg of CO2
}

//get the electricity consumption of a device/appliance
fn electricity_consumption(msg: String) -> ElectricityConsumption {
    let power_rating = input::get_input_f64(msg);
    let usage_time = input::get_input_f64(
        "Enter the number of hours you use the device/appliance per day: ".to_string(),
    );
    let carbon_intensity = 0.402; // hardcoded value for Nigeria in kg of CO2 per kWh
    let carbon_footprint = (power_rating / 1000.0) * usage_time * carbon_intensity;

    let electricity_consumption = ElectricityConsumption {
        power_rating: power_rating,
        usage_time: usage_time,
        carbon_footprint: carbon_footprint,
    };

    electricity_consumption
}

//get the electricity consumption for n number of devices/appliances
pub fn get_total_footprint(filename: String) -> f64 {
    println!("Calculating the carbon footprint of your electricity consumption...");
    let number_of_devices =
        input::get_input_i32("Enter the number of devices/appliances you use: ".to_string());
    let mut total_carbon_footprint = 0.0;

    let data = format!(
        "----ELECTRICITY CONSUMPTION----\nNumber of devices/appliances used: {}",
        number_of_devices
    );
    input::write_to_file(filename.as_str(), data.as_str());

    if number_of_devices.eq(&1) {
        let device_electricity_consumption = electricity_consumption(
            "Enter the power rating of the device/appliance (in watts):".to_string(),
        );

        let data = format!(
            "Device/appliance power rating: {:.2} watts, Usage time: {:.2} hours per day, Carbon footprint: {:.2} kg of CO2",
            device_electricity_consumption.power_rating,
            device_electricity_consumption.usage_time,
            device_electricity_consumption.carbon_footprint
        );
        input::write_to_file(filename.as_str(), data.as_str());

        total_carbon_footprint = device_electricity_consumption.carbon_footprint;
    } else {
        for i in 0..number_of_devices {
            let device_electricity_consumption = electricity_consumption(format!(
                "Enter the power rating of device/appliance {} (in watts): ",
                i + 1
            ));

            let data = format!(
                "Device/appliance power rating: {:.2} watts, Usage time: {:.2} hours per day, Carbon footprint: {:.2} kg of CO2",
                device_electricity_consumption.power_rating,
                device_electricity_consumption.usage_time,
                device_electricity_consumption.carbon_footprint
            );
            input::write_to_file(filename.as_str(), data.as_str());

            total_carbon_footprint += device_electricity_consumption.carbon_footprint;
        }
    }

    let data = format!(
        "Total carbon footprint: {:.2} kg of CO2",
        total_carbon_footprint
    );
    input::write_to_file(filename.as_str(), data.as_str());
    total_carbon_footprint
}
