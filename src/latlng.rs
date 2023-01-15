use crate::{convert, delegate_inner, H3Error, H3ErrorCodes, H3Index};
use std::ffi::c_int;

/// Latitude/longitude in radians.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct LatLng {
    /// Latitude in radians.
    pub lat: f64,
    /// Longitude in radians.
    pub lng: f64,
}

impl From<h3o::LatLng> for LatLng {
    fn from(value: h3o::LatLng) -> Self {
        Self {
            lat: value.lat_radians(),
            lng: value.lng_radians(),
        }
    }
}

impl From<geo_types::Coord> for LatLng {
    fn from(value: geo_types::Coord) -> Self {
        Self {
            lat: value.y,
            lng: value.x,
        }
    }
}

impl From<LatLng> for geo_types::Coord {
    fn from(value: LatLng) -> Self {
        Self {
            x: value.lng,
            y: value.lat,
        }
    }
}

impl TryFrom<LatLng> for h3o::LatLng {
    type Error = H3Error;

    fn try_from(value: LatLng) -> Result<Self, Self::Error> {
        Ok(Self::new(value.lat, value.lng)?)
    }
}

// -----------------------------------------------------------------------------

/// The great circle distance in kilometers between two spherical coordinates.
#[no_mangle]
pub extern "C" fn greatCircleDistanceKm(
    a: Option<&LatLng>,
    b: Option<&LatLng>,
) -> f64 {
    fn inner(a: LatLng, b: LatLng) -> Result<f64, H3Error> {
        let a = h3o::LatLng::try_from(a)?;
        let b = h3o::LatLng::try_from(b)?;

        Ok(a.distance_km(b))
    }

    inner(*a.expect("null pointer"), *b.expect("null pointer"))
        .unwrap_or(f64::NAN)
}

/// The great circle distance in meters between two spherical coordinates.
#[no_mangle]
pub extern "C" fn greatCircleDistanceM(
    a: Option<&LatLng>,
    b: Option<&LatLng>,
) -> f64 {
    fn inner(a: LatLng, b: LatLng) -> Result<f64, H3Error> {
        let a = h3o::LatLng::try_from(a)?;
        let b = h3o::LatLng::try_from(b)?;

        Ok(a.distance_m(b))
    }

    inner(*a.expect("null pointer"), *b.expect("null pointer"))
        .unwrap_or(f64::NAN)
}

/// The great circle distance in radians between two spherical coordinates.
///
/// This function uses the Haversine formula.
/// For math details, see:
///     <https://en.wikipedia.org/wiki/Haversine_formula>
///     `<https://www.movable-type.co.uk/scripts/latlong.html>`
///
/// @param  a  the first lat/lng pair (in radians)
/// @param  b  the second lat/lng pair (in radians)
///
/// @return    the great circle distance in radians between a and b
#[no_mangle]
pub extern "C" fn greatCircleDistanceRads(
    a: Option<&LatLng>,
    b: Option<&LatLng>,
) -> f64 {
    fn inner(a: LatLng, b: LatLng) -> Result<f64, H3Error> {
        let a = h3o::LatLng::try_from(a)?;
        let b = h3o::LatLng::try_from(b)?;

        Ok(a.distance_rads(b))
    }

    inner(*a.expect("null pointer"), *b.expect("null pointer"))
        .unwrap_or(f64::NAN)
}

/// Encodes a coordinate on the sphere to the H3 index of the containing cell at
/// the specified resolution.
///
/// Returns 0 on invalid input.
///
/// @param g The spherical coordinates to encode.
/// @param res The desired H3 resolution for the encoding.
/// @param out The encoded H3Index.
/// @returns E_SUCCESS (0) on success, another value otherwise
#[no_mangle]
pub extern "C" fn latLngToCell(
    g: Option<&LatLng>,
    res: c_int,
    out: Option<&mut H3Index>,
) -> H3Error {
    fn inner(g: LatLng, res: c_int) -> Result<H3Index, H3Error> {
        let res = convert::h3res_to_resolution(res)?;
        let ll = h3o::LatLng::try_from(g)?;
        Ok(ll.to_cell(res).into())
    }

    delegate_inner!(inner(*g.expect("null pointer"), res), out)
}
