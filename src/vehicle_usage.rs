use std::fmt::format;

use crate::input;

pub struct VehicleUsage {
    vehicle_type: String,
    fuel_consumption_rate: f64, // in litres per 100km
    distance_traveled: f64,     // in km
    emission_factor: f64,       // in kg of CO2 per litre
    pub carbon_footprint: f64,  // in kg of CO2
}

pub fn get_petrol_vehicle() -> VehicleUsage {
    let fuel_consumption_rate = input::get_input_f64(
        "Enter the petrol consumption rate of your vehicle (in litres per 100km) : ".to_string(),
    );
    let distance_traveled = input::get_input_f64(
        "Enter the distance traveled by your vehicle this month (in km): ".to_string(),
    );
    let emission_factor = 2.31;
    let carbon_footprint = fuel_consumption_rate * (distance_traveled / 100.0) * emission_factor;
    let petrol_vehicle = VehicleUsage {
        vehicle_type: "Petrol".to_string(),
        fuel_consumption_rate: fuel_consumption_rate,
        distance_traveled: distance_traveled,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };
    petrol_vehicle
}

pub fn get_diesel_vehicle() -> VehicleUsage {
    let fuel_consumption_rate = input::get_input_f64(
        "Enter the diesel consumption rate of your vehicle (in litres per 100km) : ".to_string(),
    );
    let distance_traveled = input::get_input_f64(
        "Enter the distance traveled by your vehicle this month (in km): ".to_string(),
    );
    let emission_factor = 2.68;
    let carbon_footprint = fuel_consumption_rate * (distance_traveled / 100.0) * emission_factor;
    let diesel_vehicle = VehicleUsage {
        vehicle_type: "Diesel".to_string(),
        fuel_consumption_rate: fuel_consumption_rate,
        distance_traveled: distance_traveled,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };
    diesel_vehicle
}

pub fn get_cng_vehicle() -> VehicleUsage {
    let fuel_consumption_rate = input::get_input_f64(
        "Enter the CNG consumption rate of your vehicle (in kg per 100km) : ".to_string(),
    );
    let distance_traveled = input::get_input_f64(
        "Enter the distance traveled by your vehicle this month (in km): ".to_string(),
    );
    let emission_factor = 2.75;
    let carbon_footprint = fuel_consumption_rate * (distance_traveled / 100.0) * emission_factor;
    let cng_vehicle = VehicleUsage {
        vehicle_type: "CNG".to_string(),
        fuel_consumption_rate: fuel_consumption_rate,
        distance_traveled: distance_traveled,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };
    cng_vehicle
}

pub fn get_total_footprint(filename: String) -> f64 {
    println!("\nCalculating the carbon footprint of your vehicle usage...");
    let number_of_vehicles: i32 =
        input::get_input_i32("Enter the number of vehicles in your household:".to_string());
    let mut total_footprint = 0.0;

    let mut data = format!(
        "\n----VEHICLE USAGE----\nNumber of vehicles used: {}",
        number_of_vehicles
    );
    input::write_to_file(filename.as_str(), data.as_str());

    for i in 0..number_of_vehicles {
        let vehicle_type = input::get_input_i32(
            "Enter the type of vehicle (1 for petrol, 2 for diesel, 3 for CNG):".to_string(),
        );
        match vehicle_type {
            1 => {
                let petrol_vehicle = get_petrol_vehicle();
                total_footprint += petrol_vehicle.carbon_footprint;

                let data = format!(
                    "Vehicle type: {}, Fuel consumption rate: {:.2} litres per 100km, Distance traveled: {:.2} km, Carbon footprint: {:.2} kg of CO2",
                    petrol_vehicle.vehicle_type,
                    petrol_vehicle.fuel_consumption_rate,
                    petrol_vehicle.distance_traveled,
                    petrol_vehicle.carbon_footprint
                );
                input::write_to_file(filename.as_str(), data.as_str());
            }
            2 => {
                let diesel_vehicle = get_diesel_vehicle();
                total_footprint += diesel_vehicle.carbon_footprint;

                let data = format!(
                    "Vehicle type: {}, Fuel consumption rate: {:.2} litres per 100km, Distance traveled: {:.2} km, Carbon footprint: {:.2} kg of CO2",
                    diesel_vehicle.vehicle_type,
                    diesel_vehicle.fuel_consumption_rate,
                    diesel_vehicle.distance_traveled,
                    diesel_vehicle.carbon_footprint
                );
                input::write_to_file(filename.as_str(), data.as_str());
            }
            3 => {
                let cng_vehicle = get_cng_vehicle();
                total_footprint += cng_vehicle.carbon_footprint;

                let data = format!(
                    "Vehicle type: {}, Fuel consumption rate: {:.2} litres per 100km, Distance traveled: {:.2} km, Carbon footprint: {:.2} kg of CO2",
                    cng_vehicle.vehicle_type,
                    cng_vehicle.fuel_consumption_rate,
                    cng_vehicle.distance_traveled,
                    cng_vehicle.carbon_footprint
                );
                input::write_to_file(filename.as_str(), data.as_str());
            }
            _ => {
                println!("Invalid vehicle type. Please enter a valid vehicle type.");
            }
        }
    }

    data = format!("Total carbon footprint: {} kg of CO2", total_footprint);
    input::write_to_file(filename.as_str(), data.as_str());

    total_footprint
}
