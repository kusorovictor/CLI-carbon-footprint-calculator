use chrono::Local;

mod electricity_consumption;
mod food_consumption;
mod fuel_consumption;
mod input;
mod vehicle_usage;
mod waste_generation;
mod water_usage;

fn main() {
    println!("Welcome to the Carbon Footprint Calculator!");
    println!("This program will help you calculate your carbon footprint based on your electricity consumption, food consumption, fuel consumption, waste generation, water usage, and vehicle usage.");
    println!("Please provide the requested information to get started...\n");

    let filename = build_filename();
    launch_carbon_footprint_calculator(filename.clone());

    println!("Carbon footprint analysis completed successfully!");
    println!("Analysis written to file: {}", filename.clone());
}

fn calculate_all_footprints(filename: String) {
    let mut data =
        format!("--------------------CARBON FOOTPRINT ANALYSIS--------------------").to_string();
    input::write_to_file(filename.as_str(), data.as_str());

    let electricity_footprint = electricity_consumption::get_total_footprint(filename.clone());
    let food_footprint = food_consumption::get_total_footprint(filename.clone());
    let petrol_footprint = fuel_consumption::get_total_footprint(filename.clone());
    let waste_footprint = waste_generation::get_total_footprint(filename.clone());
    let water_footprint = water_usage::get_total_footprint(filename.clone());
    let vehicle_footprint = vehicle_usage::get_total_footprint(filename.clone());
    let total_carbon_footprint = electricity_footprint
        + food_footprint
        + petrol_footprint
        + waste_footprint
        + water_footprint
        + vehicle_footprint;

    data = format!(
        "\n----TOTAL CARBON FOOTPRINT----\n\
             Total carbon footprint: {:.2} kg of CO2\n\n
             Written at {} {}",
        total_carbon_footprint,
        input::match_month(&format_day("%m")),
        format_day("%d %H:%M:%S")
    );
    input::write_to_file(filename.as_str(), data.as_str());
}

fn build_filename() -> String {
    //get the current month
    let binding = format_day("%m");
    let formatted_month = input::match_month(&binding);
    let mut filename = format!("{}_carbon_footprint_analysis", formatted_month);

    //make sure the file created is a text file
    filename.push_str(".txt");

    filename
}

//function to format the current day to your liking
fn format_day(format: &str) -> String {
    let now = Local::now();
    now.format(format).to_string()
}

fn launch_carbon_footprint_calculator(filename: String) {
    let mut choice = input::get_input_i32("Select a choice: \n1. Calculate your total carbon footprint.\n2. Calculate a selected carbon footprint.".to_string());

    while choice < 1 || choice > 2 {
        choice = input::get_input_i32("Select a number between 1 and 2".to_string());
    }

    if choice.eq(&1) {
        calculate_all_footprints(filename.clone());
    } else {
        calculate_selected_footprints(filename.clone());
    }
}

fn calculate_selected_footprints(filename: String) {
    println!("Please select up to 3 categories to calculate:");
    println!("1. Electricity Consumption");
    println!("2. Food Consumption");
    println!("3. Fuel Consumption");
    println!("4. Waste Generation");
    println!("5. Water Usage");
    println!("6. Vehicle Usage");

    let mut num_of_categories =
        input::get_input_i32("How many number of categories are you selecting? ".to_string());

    while num_of_categories.eq(&0) || num_of_categories > 3 {
        num_of_categories = input::get_input_i32(
            "Invalid choice, please select a number between 1 and 3: ".to_string(),
        );
    }

    let mut selected_categories = Vec::new();
    for _ in 0..num_of_categories {
        let choice = input::get_input_i32("Select a category: ".to_string());

        if choice < 1 || choice > 6 {
            println!("Invalid choice, please select a number between 1 and 6.");
            continue;
        }

        if selected_categories.contains(&choice) {
            println!("Category already selected, please choose a different one.");
            continue;
        }

        selected_categories.push(choice);
    }

    let mut total_carbon_footprint = 0.0;
    for category in selected_categories {
        total_carbon_footprint += match category {
            1 => electricity_consumption::get_total_footprint(filename.clone()),
            2 => food_consumption::get_total_footprint(filename.clone()),
            3 => fuel_consumption::get_total_footprint(filename.clone()),
            4 => waste_generation::get_total_footprint(filename.clone()),
            5 => water_usage::get_total_footprint(filename.clone()),
            6 => vehicle_usage::get_total_footprint(filename.clone()),
            _ => 0.0,
        };
    }

    let data = format!(
        "\n----SELECTED CARBON FOOTPRINT----\n\
             Total selected carbon footprint: {:.2} kg of CO2\n\n
             Written at {} {}",
        total_carbon_footprint,
        input::match_month(&format_day("%m")),
        format_day("%d %H:%M:%S")
    );
    input::write_to_file(filename.as_str(), data.as_str());
}

