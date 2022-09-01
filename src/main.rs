# [macro_use]
extern crate rocket;

use rocket::{http::Status, serde::json::Json};
use rand::Rng;

//fuel usage for distance
fn fuel_usage(distance: f32, fuel: f32) -> f32 {
    distance * fuel
}

# [get("/calculateDieselUsageForDistance?<distance>&<year_of_production>&<fuel_usage_per_100_km>")]
pub fn calculate_diesel_usage(distance: f32, year_of_production: u16, fuel_usage_per_100_km: f32) -> Result<Json<f32>, Status> {
    Ok(
        Json(
            fuel_usage(distance, fuel_usage_per_100_km / 100.0)
        )
    )
}

// random number between 0 and 1 
fn generate_random() -> f32 {
	let mut rng = rand::thread_rng();
	f32::trunc(rng.gen_range(0.01..1.00) * 100.0) / 100.0
}

# [get("/probabilityOfUnitInjectorFail?<vin>")]
pub fn probability_of_unit_injector_fail(vin: String) -> Result<Json<f32>, Status> {
    Ok(
		Json(
			generate_random()
		)
	)
}

// Run the server
# [launch]
fn rocket() -> _ {
    rocket::build().mount("/api",
     routes!
     [calculate_diesel_usage,
      probability_of_unit_injector_fail])
}