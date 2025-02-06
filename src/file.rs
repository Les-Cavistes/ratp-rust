use crate::{StopOutput, StopRecord};
use csv::ReaderBuilder;
use std::error::Error;
use std::{
    fs::{self, File},
    path::Path,
};

pub fn read_all_stops() -> Result<Vec<StopRecord>, Box<dyn Error>> {
    let file = File::open("src/assets/arrets-lignes.csv")?;
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let records: Vec<StopRecord> = reader.deserialize().collect::<Result<_, _>>()?;

    Ok(records)
}

pub fn save_to_csv(stops: &[&StopRecord], path: &str) -> Result<(), Box<dyn Error>> {
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

pub fn save_to_json(stops: &[&StopRecord], path: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let file = File::create(path)?;

    // Convert records to output format
    let outputs: Vec<StopOutput> = stops.iter().map(|&stop| stop.into()).collect();

    serde_json::to_writer_pretty(file, &outputs)?;

    Ok(())
}
