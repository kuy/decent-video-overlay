use crate::libs::{models::ShotData, Range};

#[derive(Debug, PartialEq)]
pub struct ChartData {
    pub series: Vec<DataPoint>,
    pub range: Range,
}

impl ChartData {
    pub fn for_pressure(data: &ShotData) -> Self {
        let mut series = vec![];
        for (t, v) in data.elapsed.iter().zip(data.pressure.pressure.iter()) {
            series.push(DataPoint::Present((*t, *v)));
        }
        Self {
            series,
            range: Range::from_series(&data.pressure.pressure),
        }
    }

    pub fn for_temp_basket(data: &ShotData) -> Self {
        let mut series = vec![];
        for (t, v) in data.elapsed.iter().zip(data.temperature.basket.iter()) {
            series.push(DataPoint::Present((*t, *v)));
        }
        Self {
            series,
            range: Range::from_series(&data.temperature.basket),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataPoint {
    NoData(f32),
    Present((f32, f32)),
}

impl DataPoint {
    pub fn t(&self) -> f32 {
        match self {
            Self::NoData(t) => *t,
            Self::Present((t, _)) => *t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::libs::models::PressureSection;

    #[test]
    fn test_chart_data_for_pressure() {
        let data = ShotData {
            elapsed: vec![0.0, 0.044, 0.268],
            pressure: PressureSection {
                pressure: vec![0.0, 0.03, 0.22],
                ..Default::default()
            },
            ..Default::default()
        };
        let actual = ChartData::for_pressure(&data);
        let expected = ChartData {
            series: vec![
                DataPoint::Present((0.0, 0.0)),
                DataPoint::Present((0.044, 0.03)),
                DataPoint::Present((0.268, 0.22)),
            ],
            range: Range {
                min: 0.0,
                max: 0.22,
            },
        };
        assert_eq!(actual, expected);
    }
}
