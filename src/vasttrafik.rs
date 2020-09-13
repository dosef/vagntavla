use chrono::Local;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::configuration::Configuration;

const API_BASE_URL: &str = "https://api.vasttrafik.se/bin/rest.exe/v2/";

#[derive(Serialize, Deserialize, Debug)]
pub struct Vasttrafik {
    scope: String,
    token_type: String,
    expires_in: i32,
    access_token: String,
}

impl Vasttrafik {
    pub fn new(cfg: Configuration) -> Result<Vasttrafik, String> {
        let token = match Vasttrafik::get_token(cfg.key, cfg.secret) {
            Ok(value) => value,
            Err(value) => return Err(value),
        };

        Ok(token)
    }

    pub fn get_upcoming_at_stop(&self, stop_id: &str) -> Result<DepartureBoard, reqwest::Error> {
        let now = Local::now();
        let time = now.format("%H:%M").to_string();
        let date = now.format("%Y-%m-%d").to_string();

        let service = "departureBoard";
        let query = format!("?id={}&date={}&time={}&format=json", stop_id, date, time);
        let response = Vasttrafik::send_request(self, service, &query);

        response.unwrap().json::<DepartureBoard>()
    }

    pub fn get_stop_info(&self, stop_name: &str) -> Result<Vec<StopLocation>, reqwest::Error> {
        let service = "location.name";
        let query = format!("?input={}&format=json", stop_name);
        let response = Vasttrafik::send_request(self, service, &query);
        Ok(response
            .unwrap()
            .json::<Stops>()
            .unwrap()
            .location_list
            .stop_location)
    }

    fn send_request(
        &self,
        service: &str,
        query: &str,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = API_BASE_URL.to_owned() + &service + &query;
        let client = reqwest::blocking::Client::new();
        client
            .get(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .bearer_auth(&self.access_token)
            .send()
    }

    fn get_token(key: String, secret: String) -> Result<Vasttrafik, String> {
        let mut params = HashMap::new();
        params.insert("Content-Type", "application/x-www-form-urlencoded");
        let client = reqwest::blocking::Client::new();
        let response = client
            .post("https://api.vasttrafik.se/token")
            .form(&params)
            .basic_auth(key, Some(secret))
            .body("grant_type=client_credentials&scope=1")
            .send()
            .unwrap()
            .json::<Vasttrafik>();

        match response {
            Ok(value) => Ok(value),
            Err(_) => Err("Could not retrieve token with given (key, secret)".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepartureBoard {
    #[serde(rename = "DepartureBoard")]
    pub departure_board: Departure,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Departure {
    pub servertime: String,
    pub serverdate: String,
    #[serde(rename = "Departure")]
    pub departure: Vec<DepartureList>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DepartureList {
    pub name: String,
    pub sname: String,
    #[serde(rename = "journeyNumber")]
    pub journey_number: String,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub stopid: String,
    pub stop: String,
    pub time: String,
    pub date: String,
    pub journeyid: String,
    pub direction: String,
    #[serde(rename = "rtTime", default = "default_value")]
    pub rt_time: String,
    #[serde(rename = "rtDate", default = "default_value")]
    pub rt_date: String,
    #[serde(rename = "fgColor")]
    pub fg_color: String,
    #[serde(rename = "bgColor")]
    pub bg_color: String,
    pub stroke: String,
    //pub accessibility: String,
    pub track: String,
}

fn default_value() -> String {
    "".to_string()
}

#[derive(Deserialize, Debug)]
pub struct StopLocation {
    pub name: String,
    pub lon: String,
    pub lat: String,
    pub id: String,
    pub idx: String,
}

#[derive(Deserialize, Debug)]
pub struct LocationList {
    pub servertime: String,
    pub serverdate: String,
    #[serde(rename = "StopLocation")]
    pub stop_location: Vec<StopLocation>,
}

#[derive(Deserialize, Debug)]
pub struct Stops {
    #[serde(rename = "LocationList")]
    pub location_list: LocationList,
}
