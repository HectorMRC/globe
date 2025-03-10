//! Equirectangular projection.

use crate::{cartesian, float::PositiveFloat, geographic};

use super::Projection;

/// The [equirectangular projection](https://en.wikipedia.org/wiki/Equirectangular_projection).
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Equirectangular {
    radius: PositiveFloat,
}

impl Projection for Equirectangular {
    fn forward(&self, coordinates: &geographic::Coordinates) -> cartesian::Coordinates {
        cartesian::Coordinates {
            x: self.radius.as_float() * coordinates.longitude.as_float(),
            y: self.radius.as_float() * coordinates.latitude.as_float(),
            ..Default::default()
        }
    }

    fn reverse(&self, coordinates: &cartesian::Coordinates) -> geographic::Coordinates {
        geographic::Coordinates {
            latitude: (coordinates.y / self.radius.as_float()).into(),
            longitude: (coordinates.x / self.radius.as_float()).into(),
            ..Default::default()
        }
    }
}
