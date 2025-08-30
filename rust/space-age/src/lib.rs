// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // todo!("s, measured in seconds: {s}")
        Self { seconds: s }
    }
}

pub trait Planet {
    fn orbital_period() -> f32;

    fn years_during(d: &Duration) -> f64 {
        // todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
        let earth_year_in_seconds = 31_557_600.0;
        let earth_years = d.seconds as f64 / earth_year_in_seconds;
        let planet_years = earth_years / Self::orbital_period() as f64;

        (planet_years * 100.0).round() / 100.0
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_period() -> f32 {
        0.2408467
    }
}
impl Planet for Venus {
    fn orbital_period() -> f32 {
        0.61519726
    }
}
impl Planet for Earth {
    fn orbital_period() -> f32 {
        1.0
    }
}
impl Planet for Mars {
    fn orbital_period() -> f32 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn orbital_period() -> f32 {
        11.862615
    }
}
impl Planet for Saturn {
    fn orbital_period() -> f32 {
        29.447498
    }
}
impl Planet for Uranus {
    fn orbital_period() -> f32 {
        84.016846
    }
}
impl Planet for Neptune {
    fn orbital_period() -> f32 {
        164.79132
    }
}
