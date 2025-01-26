use chrono::{format, Local};

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

fn launch_carbon_footprint_calculator(filename: String) {
    let mut data =
        format!("            --------------------CARBON FOOTPRINT ANALYSIS--------------------");
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
        "\n----TOTAL CARBON ----\n\
             Total carbon footprint: {:.2} kg of CO2\n\n
             Written at {},{}",
        total_carbon_footprint,
        match_month(&format_day("%m")),
        format_day("%d %H:%M:%S")
    );
    input::write_to_file(filename.as_str(), data.as_str());
}

fn build_filename() -> String {
    //get the current month
    let binding = format_day("%m");
    let formatted_month = match_month(&binding);
    let mut filename = format!("{}_carbon_footprint_analysis", formatted_month);

    //make sure the file created is a text file
    filename.push_str(".txt");

    filename
}

//function to match the month to the current month
fn match_month(month: &str) -> &str {
    match month {
        "01" => "January",
        "02" => "February",
        "03" => "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" => "July",
        "08" => "August",
        "09" => "September",
        "10" => "October",
        "11" => "November",
        "12" => "December",
        _ => "Invalid month",
    }
}

//function to format the current day to your liking
fn format_day(format: &str) -> String {
    let now = Local::now();
    now.format(format).to_string()
}
