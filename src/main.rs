use ratp_rust::{
    file::{read_all_stops, save_to_csv, save_to_json},
    StopRecord,
};

fn main() {
    match read_all_stops() {
        Ok(stops) => {
            let output_dir = "/Users/tom_planche/Desktop/CNAM/COURS/PROJET_SEMESTRE_2/project-semester-2/back/src/assets";

            println!("Read {} stops", stops.len());

            // Métro stops
            let all_stops: Vec<&StopRecord> = stops
                .iter()
                .filter(|stop| {
                    let mode = stop.mode.to_lowercase();

                    mode == "localtrain" || mode == "metro" || mode == "rapidtransit"
                })
                .collect();

            if let Err(e) = save_to_csv(
                &all_stops,
                format!("{}/metro_transilien_rer_stops.csv", output_dir).as_str(),
            ) {
                eprintln!("Error saving CSV: {e}");
            }

            if let Err(e) = save_to_json(
                &all_stops,
                format!("{}/metro_transilien_rer_stops.json", output_dir).as_str(),
            ) {
                eprintln!("Error saving JSON: {e}");
            }

            println!("Files saved in the 'output' directory");
        }
        Err(e) => eprintln!("Error reading stops: {e}"),
    }
}
