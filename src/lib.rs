pub mod file;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct StopRecord {
    pub id: String,
    pub route_long_name: String,
    pub stop_id: String,
    pub stop_name: String,
    pub stop_lon: f64,
    pub stop_lat: f64,
    pub operatorname: String, // Not used
    pub shortname: String,
    pub mode: String,
    pub pointgeo: String,
    pub nom_commune: String,
    pub code_insee: String,
}

// New struct for output without operatorname and mode
#[derive(Debug, Serialize)]
pub struct StopOutput {
    pub route_id: String,
    pub route_long_name: String,
    pub id: String,
    pub stop_name: String,
    pub stop_lon: f64,
    pub stop_lat: f64,
    pub operatorname: String,
    pub shortname: String,
    pub pointgeo: String,
    pub nom_commune: String,
    pub code_insee: String,
    pub mode: String,
}

impl From<&StopRecord> for StopOutput {
    fn from(record: &StopRecord) -> Self {
        StopOutput {
            route_id: record.id.clone(),
            route_long_name: record.route_long_name.clone(),
            id: record.stop_id.clone(),
            stop_name: record.stop_name.clone(),
            stop_lon: record.stop_lon,
            stop_lat: record.stop_lat,
            operatorname: record.operatorname.clone(),
            shortname: record.shortname.clone(),
            pointgeo: record.pointgeo.clone(),
            nom_commune: record.nom_commune.clone(),
            code_insee: record.code_insee.clone(),
            mode: record.mode.clone(),
        }
    }
}
