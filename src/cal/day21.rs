use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use country_boundaries::{CountryBoundaries, LatLon, BOUNDARIES_ODBL_360X180};
use s2::{cellid::CellID, latlng::LatLng};

pub(crate) fn router() -> Router {
    Router::new()
        .route("/21/coords/:id", get(coords))
        .route("/21/country/:id", get(country))
}

async fn coords(Path(id): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = u64::from_str_radix(&id, 2).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let cell = CellID(id);
    if !cell.is_valid() {
        return Err((StatusCode::BAD_REQUEST, "Invalid cell ID".to_string()));
    }
    let pos = LatLng::from(cell);
    let lat = format!("{:.3}", radians::Deg::new(pos.lat.deg()));
    let lat = if let Some(lat) = lat.strip_prefix('-') {
        format!("{lat}S")
    } else {
        lat + "N"
    };
    let lng = format!("{:.3}", radians::Deg::new(pos.lng.deg()));
    let lng = if let Some(lng) = lng.strip_prefix('-') {
        format!("{lng}W")
    } else {
        lng + "E"
    };
    Ok(lat + " " + &lng)
}

async fn country(Path(id): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = u64::from_str_radix(&id, 2).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let cell = CellID(id);
    if !cell.is_valid() {
        return Err((StatusCode::BAD_REQUEST, "Invalid cell ID".to_string()));
    }

    let pos = LatLng::from(cell);
    let pos = LatLon::new(pos.lat.deg(), pos.lng.deg())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let countries = CountryBoundaries::from_reader(BOUNDARIES_ODBL_360X180)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(countries
        .ids(pos)
        .into_iter()
        .find_map(|id| isocountry::CountryCode::for_alpha2(id).ok())
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Not found".to_string()))?
        .name()
        .trim_end_matches(" Darussalam")
        .to_string())
}
