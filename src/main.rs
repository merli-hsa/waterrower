//! WaterRower Command Line Tool

mod wr_utils;

use std::{fs, path::PathBuf, str};
use structopt::StructOpt;

use crate::wr_utils::InstantWorkoutValues;

const DEFAULT_WORKOUT_DIR: &str = "./workouts";

#[derive(StructOpt)]
#[structopt(name = "waterrower", about = "WaterRower Command Line Tool")]
enum WaterRower {
    Record {
        /// Serial device for WaterRower communication
        #[structopt(short, long)]
        serial_dev: String,
        /// Directory to store workouts' data
        #[structopt(short, long, parse(from_os_str), default_value = DEFAULT_WORKOUT_DIR)]
        workout_dir: PathBuf,
        /// Prints debug information during runtime
        #[structopt(short, long)]
        debug: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match WaterRower::from_args() {
        WaterRower::Record {
            serial_dev,
            workout_dir,
            debug,
        } => {
            println!("\n### Initializing WaterRower workout recording ...");

            if debug {
                println!("--- Initializing workout context ...");
            }
            let mut workout_context = wr_utils::workout_context_init(serial_dev.as_str(), debug);

            if debug {
                println!("--- Starting WaterRower communication ...");
            }
            wr_utils::start(&mut workout_context);

            if debug {
                println!("--- Initializing global workout values ...");
            }
            let mut global_workout_values =
                wr_utils::global_workout_values_init(&mut workout_context);

            // Print basic workout information
            println!(
                "--- Date and Time of Start:    {}",
                global_workout_values.date_time_start
            );
            println!(
                "--- WaterRower Model:          {}",
                global_workout_values.model
            );
            println!(
                "--- Firmware Version:          {}",
                global_workout_values.fw_version
            );

            if debug {
                println!("--- Creating workout directory ...");
            }
            let workout_path = PathBuf::from(format!(
                "{}{}{}",
                &workout_dir.to_str().unwrap(),
                "/",
                global_workout_values
                    .date_time_start
                    .replace(" ", "_")
                    .replace(":", "-")
            ));
            fs::create_dir_all(&workout_path)?;

            println!("\n### Waiting for first stroke on WaterRower to begin ...");
            wr_utils::wait_for_first_stroke(&mut workout_context);
            println!("--- Detected!");

            println!("\n### Recording workout ...");
            let mut datapoints: Vec<InstantWorkoutValues> = Vec::new();

            loop {
                let mut instant_workout_values = wr_utils::instant_workout_values_init();

                // Get current workout values
                wr_utils::workout_values_update(
                    &mut workout_context,
                    &mut instant_workout_values,
                    &mut global_workout_values,
                );

                // Check if workout finished
                if let wr_utils::WorkoutState::Finished = workout_context.state {
                    break;
                }

                // Append values to datapoint vector
                datapoints.push(instant_workout_values);
            }

            println!("\n### Closing WaterRower workout session ...");
            wr_utils::stop(&mut workout_context);

            if debug {
                println!("--- Finalizing global workout values ...");
            }
            wr_utils::global_workout_values_finalize(&datapoints, &mut global_workout_values);

            println!(
                "--- Date and Time of End:      {}",
                global_workout_values.date_time_end
            );
            println!(
                "--- Workout Duration:          {:02}:{:02}:{:02}",
                global_workout_values.total_time_in_seconds / 3600,
                global_workout_values.total_time_in_seconds % 3600 / 60,
                global_workout_values.total_time_in_seconds % 3600 % 60,
            );
            println!(
                "--- Total Distance in Meters:  {}",
                global_workout_values.total_distance_in_meters
            );

            println!("\n### Writing workout data and meta data to CSV files ...");
            wr_utils::write_meta_data_file(&workout_path, &global_workout_values)?;
            wr_utils::write_workout_data_file(&workout_path, &datapoints)?;

            println!("\n### Bye!");
        }
    }
    Ok(())
}
