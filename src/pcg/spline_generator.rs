use splines::{Interpolation, Key, Spline};

pub struct SplineGenerator<'a> {
    points: &'a [(f32, f32)],
    interpolation: Interpolation<f32, f32>,
}

impl<'a> SplineGenerator<'a> {
    pub fn new(points: &'a [(f32, f32)], interpolation: Interpolation<f32, f32>) -> Self {
        Self {
            points,
            interpolation,
        }
    }
}

impl<'a> From<SplineGenerator<'a>> for Spline<f32, f32> {
    fn from(value: SplineGenerator<'a>) -> Self {
        let keys = value
            .points
            .iter()
            .map(|&(t, v)| Key::new(t, v, value.interpolation));

        Spline::from_iter(keys)
    }
}
