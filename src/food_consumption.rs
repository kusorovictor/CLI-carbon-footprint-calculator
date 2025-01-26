use crate::input;

pub struct FoodConsumption {
    food_type: String,
    food_consumed: f64,        // in kg per month
    emission_factor: f64,      // in kg of CO2 per kg of food
    pub carbon_footprint: f64, // in kg of CO2
}

// Calculate the carbon footprint of meat consumption
pub fn get_meat_consumption() -> FoodConsumption {
    let meat_consumed =
        input::get_input_f64("Enter your monthly meat consumption in kg: ".to_string());
    let emission_factor = 23.0; // hardcoded value for Nigeria in kg of CO2 per kg of beef
    let carbon_footprint = meat_consumed * emission_factor;

    let food_consumption = FoodConsumption {
        food_type: "meat".to_string(),
        food_consumed: meat_consumed,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    food_consumption
}

// Calculate the carbon footprint of egg consumption
pub fn get_egg_consumption() -> FoodConsumption {
    let egg_consumed =
        input::get_input_f64("Enter your monthly egg consumption (in kg): ".to_string());
    let emission_factor = 5.0; // hardcoded value for Nigeria in kg of CO2 per kg of milk
    let carbon_footprint = egg_consumed * emission_factor;

    let food_consumption = FoodConsumption {
        food_type: "egg".to_string(),
        food_consumed: egg_consumed,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    food_consumption
}

// Calculate the carbon footprint of milk consumption
pub fn get_milk_consumption() -> FoodConsumption {
    let milk_consumed =
        input::get_input_f64("Enter your monthly milk consumption (in kg): ".to_string());
    let emission_factor = 1.9; // hardcoded value for Nigeria in kg of CO2 per kg of milk
    let carbon_footprint = milk_consumed * emission_factor;

    let food_consumption = FoodConsumption {
        food_type: "milk".to_string(),
        food_consumed: milk_consumed,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    food_consumption
}

// Calculate the carbon footprint of grain consumption
pub fn get_grain_consumption() -> FoodConsumption {
    let grain_consumed =
        input::get_input_f64("Enter your monthly grain consumption (in kg): ".to_string());
    let emission_factor = 1.5; // hardcoded value for Nigeria in kg of CO2 per kg of grain
    let carbon_footprint = grain_consumed * emission_factor;

    let food_consumption = FoodConsumption {
        food_type: "grain".to_string(),
        food_consumed: grain_consumed,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    food_consumption
}

// Calculate the carbon footprint of vegetable consumption
pub fn get_vegetable_consumption() -> FoodConsumption {
    let vegetable_consumed =
        input::get_input_f64("Enter your monthly vegetable/fruit consumption in kg: ".to_string());
    let emission_factor = 1.0; // hardcoded value for Nigeria in kg of CO2 per kg of vegetables
    let carbon_footprint = vegetable_consumed * emission_factor;

    let food_consumption = FoodConsumption {
        food_type: "vegetable".to_string(),
        food_consumed: vegetable_consumed,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    food_consumption
}

// Calculate the carbon footprint of nuts consumption
pub fn get_nuts_consumption() -> FoodConsumption {
    let nuts_consumed =
        input::get_input_f64("Enter your monthly nuts consumption in kg: ".to_string());
    let emission_factor = 1.0; // hardcoded value for Nigeria in kg of CO2 per kg of nuts
    let carbon_footprint = nuts_consumed * emission_factor;

    let food_consumption = FoodConsumption {
        food_type: "nuts".to_string(),
        food_consumed: nuts_consumed,
        emission_factor: emission_factor,
        carbon_footprint: carbon_footprint,
    };

    food_consumption
}

//get the total food carbon footprint
pub fn get_total_footprint(filename: String) -> f64 {
    println!("\nCalculating the carbon footprint of your food consumption...");
    let meat_consumption = get_meat_consumption();
    let egg_consumption = get_egg_consumption();
    let milk_consumption = get_milk_consumption();
    let grain_consumption = get_grain_consumption();
    let vegetable_consumption = get_vegetable_consumption();
    let nuts_consumption = get_nuts_consumption();

    let total_carbon_footprint = meat_consumption.carbon_footprint
        + egg_consumption.carbon_footprint
        + milk_consumption.carbon_footprint
        + grain_consumption.carbon_footprint
        + vegetable_consumption.carbon_footprint
        + nuts_consumption.carbon_footprint;

    let data = format!(
        "\n----FOOD CONSUMPTION----\n\
             Meat consumed: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Egg consumed: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Milk consumed: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Grain consumed: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Vegetable consumed: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Nuts consumed: {:.2} kg, Carbon footprint: {:.2} kg of CO2\n\
             Total carbon footprint: {:.2} kg of CO2",
        meat_consumption.food_consumed,
        meat_consumption.carbon_footprint,
        egg_consumption.food_consumed,
        egg_consumption.carbon_footprint,
        milk_consumption.food_consumed,
        milk_consumption.carbon_footprint,
        grain_consumption.food_consumed,
        grain_consumption.carbon_footprint,
        vegetable_consumption.food_consumed,
        vegetable_consumption.carbon_footprint,
        nuts_consumption.food_consumed,
        nuts_consumption.carbon_footprint,
        total_carbon_footprint
    );

    input::write_to_file(filename.as_str(), data.as_str());
    total_carbon_footprint
}
