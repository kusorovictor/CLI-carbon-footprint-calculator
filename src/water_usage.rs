use crate::input;

pub struct WaterUsage {
    water_consumption: f64,    //in litres per month
    energy_intensity: f64,     //in kwh per litre per month
    carbon_intensity: f64,     //in kg per
    pub carbon_footprint: f64, //in kg of CO2
}

pub fn get_water_usage() -> WaterUsage {
    let water_consumption =
        input::get_input_f64("Enter your monthly water consumption (in litres): ".to_string());
    let energy_intensity = 0.005; //hardcoded value in kwh per litre per month
    let carbon_intensity = 0.402; //hardcoded value for nigeria in kg of CO2 per kwh
    let carbon_footprint = water_consumption * energy_intensity * carbon_intensity;

    let water_usage = WaterUsage {
        water_consumption: water_consumption,
        energy_intensity: energy_intensity,
        carbon_intensity: carbon_intensity,
        carbon_footprint: carbon_footprint,
    };

    water_usage
}

pub fn get_total_footprint(filename: String) -> f64 {
    println!("\nCalculating the carbon footprint of your water usage...");
    let water_usage = get_water_usage();

    let data = format!(
        "\n----WATER USAGE-----\n\
         Water consumption: {:.2} litres, Carbon footprint: {:.2} kg of CO2",
        water_usage.water_consumption, water_usage.carbon_footprint
    );
    input::write_to_file(filename.as_str(), data.as_str());

    water_usage.carbon_footprint
}
