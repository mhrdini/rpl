use std::fmt::{self, Display};

use anyhow::{Context, Result, anyhow};
use reqwest::blocking::Client;
use serde::Deserialize;

pub struct OpenMeteo {
    client: Client,
    weather_forecast_url: String,
    geocoding_url: String,
}

impl OpenMeteo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            weather_forecast_url: "https://api.open-meteo.com/v1/forecast".into(),
            geocoding_url: "https://geocoding-api.open-meteo.com/v1/search".into(),
        }
    }

    fn get_weather(&self, location: &Location) -> Result<OMResponse> {
        let resp = self
            .client
            .get(&self.weather_forecast_url)
            .query(&[
                ("latitude", location.latitude.to_string().as_str()),
                ("longitude", location.longitude.to_string().as_str()),
                (
                    "current",
                    "precipitation,rain,showers,cloud_cover,temperature_2m",
                ),
                // ("timezone", location.timezone.as_str()),
                ("forecast_days", "1"),
            ])
            .send()
            .with_context(|| "failed to request forecast from Open-Meteo")?;

        let weather: OMResponse = resp
            .json()
            .with_context(|| "failed to parse weather forecast response")?;

        Ok(weather)
    }

    pub fn get_weather_by_location(&self, location: &str) -> Result<OMResponse> {
        let resp = self
            .client
            .get(&self.geocoding_url)
            .query(&[("name", location), ("count", "1")])
            .send()
            .with_context(|| "failed to request geocoding results")?;

        let locations: GeocodingResults = resp
            .json()
            .with_context(|| "failed to parse geocoding response")?;

        let location = locations
            .results
            .first()
            .ok_or_else(|| anyhow!("no location found for '{location}'"))?;

        self.get_weather(location)
    }
}

#[derive(Deserialize)]
pub struct GeocodingResults {
    results: Vec<Location>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    name: String,
    latitude: f32,
    longitude: f32,
    timezone: String,
    country: String,
}

#[derive(Deserialize, Debug)]
pub struct OMResponse {
    current: OMCurrent,
    current_units: OMUnits,
    timezone: String,
}

#[derive(Deserialize, Debug)]
pub struct OMCurrent {
    time: String,
    temperature_2m: f32,
    precipitation: f32,
    rain: f32,
    showers: f32,
    cloud_cover: f32,
}

#[derive(Deserialize, Debug)]
pub struct OMUnits {
    temperature_2m: Celsius,
    precipitation: Millimeters,
    rain: Millimeters,
    showers: Millimeters,
    cloud_cover: Percent,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Celsius(String);

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Millimeters(String);

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Percent(String);

impl Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "°C")
    }
}

impl Display for Millimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mm")
    }
}

impl Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%")
    }
}

#[cfg(test)]
mod tests {
    use httpmock::{Method, MockServer};
    use reqwest::StatusCode;

    use super::*;

    #[test]
    fn mock_server_responds_with_hello() {
        // setup mock server
        let server = MockServer::start();
        server.mock(|when, then| {
            when.method(Method::GET);
            then.status(StatusCode::OK).body("hello");
        });

        // send API call to mock server
        let resp = reqwest::blocking::Client::new()
            .get(server.base_url())
            .send()
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK, "mock server: wrong status");
        assert_eq!(resp.text().unwrap(), "hello", "mock server: wrong body");
    }
}
