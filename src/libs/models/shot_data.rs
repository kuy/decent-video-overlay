#[derive(Debug, PartialEq)]
pub struct ShotData {
    pub version: String,
    pub clock: usize,
    pub date: String,
    pub timestamp: usize,
    pub elapsed: Vec<f32>,
    pub pressure: PressureSection,
    pub flow: FlowSection,
    pub temperature: TemperatureSection,
    pub totals: TotalsSection,
    pub resistance: ResistanceSection,
    pub state_change: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct PressureSection {
    pub pressure: Vec<f32>,
    pub goal: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct FlowSection {
    pub flow: Vec<f32>,
    pub by_weight: Vec<f32>,
    pub by_weight_raw: Vec<f32>,
    pub goal: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct TemperatureSection {
    pub basket: Vec<f32>,
    pub mix: Vec<f32>,
    pub goal: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct TotalsSection {
    pub weight: Vec<f32>,
    pub water_dispensed: Vec<f32>,
}

#[derive(Debug, PartialEq)]
pub struct ResistanceSection {
    pub resistance: Vec<f32>,
    pub by_weight: Vec<f32>,
}

impl From<crate::libs::models::json::ShotDataJson> for ShotData {
    fn from(value: crate::libs::models::json::ShotDataJson) -> Self {
        Self {
            version: value.version,
            clock: usize::from_str_radix(value.clock.as_str(), 10).expect("clock"),
            date: value.date,
            timestamp: usize::from_str_radix(value.clock.as_str(), 10).expect("timestamp"),
            elapsed: to_f32(value.elapsed),
            pressure: PressureSection {
                pressure: to_f32(value.pressure.pressure),
                goal: to_f32(value.pressure.goal),
            },
            flow: FlowSection {
                flow: to_f32(value.flow.flow),
                by_weight: to_f32(value.flow.by_weight),
                by_weight_raw: to_f32(value.flow.by_weight_raw),
                goal: to_f32(value.flow.goal),
            },
            temperature: TemperatureSection {
                basket: to_f32(value.temperature.basket),
                mix: to_f32(value.temperature.mix),
                goal: to_f32(value.temperature.goal),
            },
            totals: TotalsSection {
                weight: to_f32(value.totals.weight),
                water_dispensed: to_f32(value.totals.water_dispensed),
            },
            resistance: ResistanceSection {
                resistance: to_f32(value.resistance.resistance),
                by_weight: to_f32(value.resistance.by_weight),
            },
            state_change: to_f32(value.state_change),
        }
    }
}

fn to_f32(src: Vec<String>) -> Vec<f32> {
    src.into_iter()
        .map(|s| s.as_str().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libs::models::json::*;

    #[test]
    fn test_convert_shot_data_from_json() {
        let json = ShotDataJson {
            version: "2".into(),
            clock: "1685582363".into(),
            date: "Thu Jun 01 10:19:23 JST 2023".into(),
            timestamp: "1685582363".into(),
            elapsed: vec!["0.0".into(), "0.044".into(), "0.268".into()],
            pressure: PressureSectionJson {
                pressure: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                goal: vec!["-1.0".into(), "-1.0".into(), "-1.0".into()],
            },
            flow: FlowSectionJson {
                flow: vec!["0.0".into(), "3.26".into(), "3.98".into()],
                by_weight: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                by_weight_raw: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                goal: vec!["-1.0".into(), "6.75".into(), "7.1875".into()],
            },
            temperature: TemperatureSectionJson {
                basket: vec!["87.0".into(), "85.07".into(), "85.37".into()],
                mix: vec!["87.0".into(), "87.17".into(), "87.14".into()],
                goal: vec!["87.0".into(), "87.0".into(), "87.0".into()],
            },
            totals: TotalsSectionJson {
                weight: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                water_dispensed: vec![
                    "0.0".into(),
                    "0.08161010742187501".into(),
                    "0.181060791015625".into(),
                ],
            },
            resistance: ResistanceSectionJson {
                resistance: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                by_weight: vec!["0.0".into(), "0.0".into(), "0.0".into()],
            },
            state_change: vec!["0.0".into(), "10000000.0".into(), "10000000.0".into()],
        };
        let actual: ShotData = json.into();
        let expected = ShotData {
            version: "2".into(),
            clock: 1685582363,
            date: "Thu Jun 01 10:19:23 JST 2023".into(),
            timestamp: 1685582363,
            elapsed: vec![0.0, 0.044, 0.268],
            pressure: PressureSection {
                pressure: vec![0.0, 0.0, 0.0],
                goal: vec![-1.0, -1.0, -1.0],
            },
            flow: FlowSection {
                flow: vec![0.0, 3.26, 3.98],
                by_weight: vec![0.0, 0.0, 0.0],
                by_weight_raw: vec![0.0, 0.0, 0.0],
                goal: vec![-1.0, 6.75, 7.1875],
            },
            temperature: TemperatureSection {
                basket: vec![87.0, 85.07, 85.37],
                mix: vec![87.0, 87.17, 87.14],
                goal: vec![87.0, 87.0, 87.0],
            },
            totals: TotalsSection {
                weight: vec![0.0, 0.0, 0.0],
                water_dispensed: vec![0.0, 0.08161010742187501, 0.181060791015625],
            },
            resistance: ResistanceSection {
                resistance: vec![0.0, 0.0, 0.0],
                by_weight: vec![0.0, 0.0, 0.0],
            },
            state_change: vec![0.0, 10000000.0, 10000000.0],
        };
        assert_eq!(actual, expected);
    }
}
