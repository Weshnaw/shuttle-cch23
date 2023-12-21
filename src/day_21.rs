use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use country_boundaries::LatLon;
use isocountry::CountryCode;
use s2::{cellid::CellID, latlng::LatLng};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::router::{self, ResponseError};

fn dms(dec: f64) -> (f64, f64, f64) {
    let d = dec.trunc();
    let m = (60f64 * (dec - d).abs()).trunc();
    let s = 3600f64 * (dec - d).abs() - 60f64 * m;

    (d, m, s)
}

pub async fn task_01(Path(binary): Path<String>) -> Result<impl IntoResponse, ResponseError> {
    let s2 = u64::from_str_radix(&binary, 2)?;
    let lat_lng: LatLng = CellID(s2).into();

    let lat = lat_lng.lat.deg();
    let lng = lat_lng.lng.deg();

    let n_s = if lat.is_sign_negative() { "S" } else { "N" };
    let e_w = if lng.is_sign_negative() { "W" } else { "E" };

    let lat_dms = dms(lat.abs());
    let lng_dms = dms(lng.abs());

    let result = format!(
        "{}°{}'{:.3}''{} {}°{}'{:.3}''{}",
        lat_dms.0, lat_dms.1, lat_dms.2, n_s, lng_dms.0, lng_dms.1, lng_dms.2, e_w
    );

    info!(result);

    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    #[serde(rename = "countryName")]
    country_name: String,
}

pub async fn task_02(
    Path(binary): Path<String>,
    State(state): State<Arc<router::State>>,
) -> Result<impl IntoResponse, ResponseError> {
    let s2 = u64::from_str_radix(&binary, 2)?;
    let lat_lng: LatLng = CellID(s2).into();

    let lat = lat_lng.lat.deg();
    let lng = lat_lng.lng.deg();

    info!("{} {}", lat, lng);

    let details = state.boundaries.ids(LatLon::new(lat, lng)?);

    let country_code =
        CountryCode::for_alpha2(details.last().ok_or(ResponseError::CountryNotFound)?)?;

    info!(?country_code);

    if country_code == CountryCode::BRN {
        return Ok("Brunei");
    }

    Ok(country_code.name())
}
