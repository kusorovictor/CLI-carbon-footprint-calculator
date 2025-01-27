use core::num;

use crate::input;
pub struct FuelConsumption {
    fuel_usage: f64,           //in litres per month
    emission_factor: f64,      //in kg of CO2 per kg
    pub carbon_footprint: f64, //in kg of CO2
}

pub fn get_petrol_usage() -> FuelConsumption {
    let fuel_usage =
        input::get_input_f64("Enter your monthly household petrol usage (in litres): ".to_string());
    let emission_factor = 2.31; //hardcoded value for nigeria in kg of CO2 per kg
    let carbon_footprint = fuel_usage * emission_factor;
    let petrol_consumption = FuelConsumption {
        fuel_usage: fuel_usage,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };
    petrol_consumption
}

pub fn get_lpg_usage() -> FuelConsumption {
    let fuel_usage =
        input::get_input_f64("Enter your monthly cooking gas usage (in litres): ".to_string());
    let emission_factor = 2.96; //hardcoded value for nigeria in kg of CO2 per kg
    let carbon_footprint = fuel_usage * emission_factor;
    let petrol_consumption = FuelConsumption {
        fuel_usage: fuel_usage,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };
    petrol_consumption
}

pub fn get_total_footprint(filename: String) -> f64 {
    println!("\nCalculating your household fuel consumption carbon footprint...");
    let petrol_consumption = get_petrol_usage();
    let lpg_consumption = get_lpg_usage();
    let total_footprint = petrol_consumption.carbon_footprint + lpg_consumption.carbon_footprint;

    let data = format!(
        "\n----FUEL USAGE-----\n\
         Petrol consumption: {:.2} litres, Carbon footprint: {:.2} kg of CO2\n\
         LPG consumption: {:.2} litres, Carbon footprint: {:.2} kg of CO2\n\
         Total carbon footprint: {:.2} kg of CO2",
        petrol_consumption.fuel_usage,
        petrol_consumption.carbon_footprint,
        lpg_consumption.fuel_usage,
        lpg_consumption.carbon_footprint,
        total_footprint
    );
    input::write_to_file(filename.as_str(), data.as_str());
    total_footprint
}
