#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

impl Range {
    pub fn from_series(series: &Vec<f32>) -> Self {
        if series.is_empty() {
            panic!("Requires non-empty vector");
        }

        let mut min = series.first().unwrap();
        let mut max = series.first().unwrap();

        for value in series.iter() {
            if value < min {
                min = value;
                continue;
            }

            if max < value {
                max = value;
                continue;
            }
        }

        Self {
            min: *min,
            max: *max,
        }
    }

    pub fn as_tuple(&self) -> (f32, f32) {
        (self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_from_series() {
        let series = vec![0.0, -10.5, 2.3, 2.33, 5.22, 1.22, 8.91, 8.912, -2.86];
        let actual = Range::from_series(&series);
        let expected = Range {
            min: -10.5,
            max: 8.912,
        };
        assert_eq!(actual, expected);
    }

    fn test_range_from_series_zero() {
        let series = vec![0.0, 0.0];
        let actual = Range::from_series(&series);
        let expected = Range { min: 0.0, max: 0.0 };
        assert_eq!(actual, expected);
    }
}
