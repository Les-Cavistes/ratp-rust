use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct StopRecord {
    id: String,
    route_long_name: String,
    stop_id: String,
    stop_name: String,
    stop_lon: f64,
    stop_lat: f64,
    // operatorname: String, // Not used
    shortname: String,
    mode: String,
    pointgeo: String,
    nom_commune: String,
    code_insee: String,
}

// New struct for output without operatorname and mode
#[derive(Debug, Serialize)]
struct StopOutput {
    id: String,
    route_long_name: String,
    stop_id: String,
    stop_name: String,
    stop_lon: f64,
    stop_lat: f64,
    shortname: String,
    pointgeo: String,
    nom_commune: String,
    code_insee: String,
}

impl From<&StopRecord> for StopOutput {
    fn from(record: &StopRecord) -> Self {
        StopOutput {
            id: record.id.clone(),
            route_long_name: record.route_long_name.clone(),
            stop_id: record.stop_id.clone(),
            stop_name: record.stop_name.clone(),
            stop_lon: record.stop_lon,
            stop_lat: record.stop_lat,
            shortname: record.shortname.clone(),
            pointgeo: record.pointgeo.clone(),
            nom_commune: record.nom_commune.clone(),
            code_insee: record.code_insee.clone(),
        }
    }
}

fn read_all_stops() -> Result<Vec<StopRecord>, Box<dyn Error>> {
    let file = File::open("src/assets/arrets-lignes.csv")?;
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let records: Vec<StopRecord> = reader.deserialize().collect::<Result<_, _>>()?;

    Ok(records)
}

fn save_to_csv(stops: &[&StopRecord], path: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;
    let mut writer = csv::WriterBuilder::new().delimiter(b';').from_writer(file);

    // Convert and write records
    for stop in stops {
        let output: StopOutput = (*stop).into();
        writer.serialize(output)?;
    }

    writer.flush()?;

    Ok(())
}

fn save_to_json(stops: &[&StopRecord], path: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;

    // Convert records to output format
    let outputs: Vec<StopOutput> = stops.iter().map(|&stop| stop.into()).collect();

    serde_json::to_writer_pretty(file, &outputs)?;

    Ok(())
}

fn main() {
    match read_all_stops() {
        Ok(stops) => {
            println!("Read {} stops", stops.len());

            let metro_stops: Vec<&StopRecord> = stops
                .iter()
                .filter(|stop| stop.mode.to_lowercase() == "metro")
                .collect();

            println!("Found {} metro stops", metro_stops.len());

            let output_dir = "output";
            fs::create_dir_all(output_dir).expect("Failed to create output directory");

            if let Err(e) = save_to_csv(&metro_stops, "output/metro_stops.csv") {
                eprintln!("Error saving CSV: {e}");
            }

            if let Err(e) = save_to_json(&metro_stops, "output/metro_stops.json") {
                eprintln!("Error saving JSON: {e}");
            }

            println!("Files saved in the 'output' directory");
        }
        Err(e) => eprintln!("Error reading stops: {e}"),
    }
}
