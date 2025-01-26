use std::fmt::format;

use crate::input;
pub struct WasteGeneration {
    waste_type: String,
    waste_generated: f64,      //in kg per month
    emission_factor: f64,      //in kg of CO2 per kg of waste
    pub carbon_footprint: f64, //in kg of CO2
}

pub fn get_muncipal_waste_generation() -> WasteGeneration {
    let waste_generated =
        input::get_input_f64("Enter your monthly municipal waste disposal (in kg): ".to_string());
    let emission_factor = 0.2; //hardcoded value for nigeria in kg of CO2 per kg of waste
    let carbon_footprint = waste_generated * emission_factor;

    let waste_generation = WasteGeneration {
        waste_type: "muncipal".to_string(),
        waste_generated: waste_generated,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    waste_generation
}

pub fn get_plastic_waste_generation() -> WasteGeneration {
    let waste_generated = input::get_input_f64(
        "Enter your monthly plasitc waste(e.g Bottles, Food Containers) disposal (in kg): "
            .to_string(),
    );
    let emission_factor = 0.15; //hardcoded value for nigeria in kg of CO2 per kg of waste
    let carbon_footprint = waste_generated * emission_factor;

    let waste_generation = WasteGeneration {
        waste_type: "plastic".to_string(),
        waste_generated: waste_generated,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    waste_generation
}

pub fn get_food_waste_generation() -> WasteGeneration {
    let waste_generated =
        input::get_input_f64("Enter your monthly food waste disposal (in kg): ".to_string());
    let emission_factor = 0.7; //hardcoded value for nigeria in kg of CO2 per kg of waste
    let carbon_footprint = waste_generated * emission_factor;

    let waste_generation = WasteGeneration {
        waste_type: "food".to_string(),
        waste_generated: waste_generated,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    waste_generation
}

pub fn get_paper_waste_generation() -> WasteGeneration {
    let waste_generated =
        input::get_input_f64("Enter your monthly paper waste disposal (in kg): ".to_string());
    let emission_factor = 0.1; //hardcoded value for nigeria in kg of CO2 per kg of waste
    let carbon_footprint = waste_generated * emission_factor;

    let waste_generation = WasteGeneration {
        waste_type: "paper".to_string(),
        waste_generated: waste_generated,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    waste_generation
}

pub fn get_textile_waste_generation() -> WasteGeneration {
    let waste_generated =
        input::get_input_f64("Enter your monthly textile waste disposal (in kg): ".to_string());
    let emission_factor = 0.2; //hardcoded value for nigeria in kg of CO2 per kg of waste
    let carbon_footprint = waste_generated * emission_factor;

    let waste_generation = WasteGeneration {
        waste_type: "textile".to_string(),
        waste_generated: waste_generated,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };
    waste_generation
}

pub fn get_total_footprint(filename: String) -> f64 {
    println!("\nCalculating your waste generation carbon footprint...");
    let muncipal_waste = get_muncipal_waste_generation();
    let plastic_waste = get_plastic_waste_generation();
    let food_waste = get_food_waste_generation();
    let paper_waste = get_paper_waste_generation();
    let textile_waste = get_textile_waste_generation();
    let total_footprint = muncipal_waste.carbon_footprint
        + plastic_waste.carbon_footprint
        + food_waste.carbon_footprint
        + paper_waste.carbon_footprint
        + textile_waste.carbon_footprint;

    let data = format!(
        "\n----WASTE GENERATED-----\n\
             Municipal Waste generated: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Plastic Waste generated: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Food Waste generated: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Paper Waste generated: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Textile Waste generated: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Total carbon footprint: {:.2} kg of CO2",
        muncipal_waste.waste_generated,
        muncipal_waste.carbon_footprint,
        plastic_waste.waste_generated,
        plastic_waste.carbon_footprint,
        food_waste.waste_generated,
        food_waste.carbon_footprint,
        paper_waste.waste_generated,
        paper_waste.carbon_footprint,
        textile_waste.waste_generated,
        textile_waste.carbon_footprint,
        total_footprint
    );
    input::write_to_file(filename.as_str(), data.as_str());

    total_footprint
}
