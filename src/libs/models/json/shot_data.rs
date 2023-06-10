use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ShotDataJson {
    pub version: String,
    pub clock: String,
    pub date: String,
    pub timestamp: String,
    pub elapsed: Vec<String>,
    pub pressure: PressureSectionJson,
    pub flow: FlowSectionJson,
    pub temperature: TemperatureSectionJson,
    pub totals: TotalsSectionJson,
    pub resistance: ResistanceSectionJson,
    pub state_change: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PressureSectionJson {
    pub pressure: Vec<String>,
    pub goal: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct FlowSectionJson {
    pub flow: Vec<String>,
    pub by_weight: Vec<String>,
    pub by_weight_raw: Vec<String>,
    pub goal: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TemperatureSectionJson {
    pub basket: Vec<String>,
    pub mix: Vec<String>,
    pub goal: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct TotalsSectionJson {
    pub weight: Vec<String>,
    pub water_dispensed: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ResistanceSectionJson {
    pub resistance: Vec<String>,
    pub by_weight: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_shot_data() {
        let json = r#"
            {
                "version": "2",
                "clock": "1685582363",
                "date": "Thu Jun 01 10:19:23 JST 2023",
                "timestamp": "1685582363",
                "elapsed": [
                    "0.0",
                    "0.044",
                    "0.268"
                ],
                "timers": {
    
                },
                "pressure": {
                    "pressure": [
                        "0.0",
                        "0.0",
                        "0.0"
                    ],
                    "goal": [
                        "-1.0",
                        "-1.0",
                        "-1.0"
                    ]
                },
                "flow": {
                    "flow": [
                        "0.0",
                        "3.26",
                        "3.98"
                    ],
                    "by_weight": [
                        "0.0",
                        "0.0",
                        "0.0"
                    ],
                    "by_weight_raw": [
                        "0.0",
                        "0.0",
                        "0.0"
                    ],
                    "goal": [
                        "-1.0",
                        "6.75",
                        "7.1875"
                    ]
                },
                "temperature": {
                    "basket": [
                        "87.0",
                        "85.07",
                        "85.37"
                    ],
                    "mix": [
                        "87.0",
                        "87.17",
                        "87.14"
                    ],
                    "goal": [
                        "87.0",
                        "87.0",
                        "87.0"
                    ]
                },
                "scale": {
    
                },
                "totals": {
                    "weight": [
                        "0.0",
                        "0.0",
                        "0.0"
                    ],
                    "water_dispensed": [
                        "0.0",
                        "0.08161010742187501",
                        "0.181060791015625"
                    ]
                },
                "resistance": {
                    "resistance": [
                        "0.0",
                        "0.0",
                        "0.0"
                    ],
                    "by_weight": [
                        "0.0",
                        "0.0",
                        "0.0"
                    ]
                },
                "state_change": [
                    "0.0",
                    "10000000.0",
                    "10000000.0"
                ],
                "profile": {
                    "title": "TurboBloom 87c",
                    "author": "Decent"
                },
                "meta": {
                    "bean": {
                        "brand": "Wakaki Coffee",
                        "type": "Ethiopia Worka Sakaro Natural"
                    }
                },
                "app": {
                    "app_name": "DE1App",
                    "app_version": "1.42.1.19"
                }
            }"#;
        let shot_data: serde_json::Result<ShotDataJson> = serde_json::from_str(json);
        if shot_data.is_err() {
            panic!("Error: {:?}", shot_data.err().unwrap());
        }
        assert_eq!(
            shot_data.ok(),
            Some(ShotDataJson {
                version: "2".into(),
                clock: "1685582363".into(),
                date: "Thu Jun 01 10:19:23 JST 2023".into(),
                timestamp: "1685582363".into(),
                elapsed: vec!["0.0".into(), "0.044".into(), "0.268".into()],
                pressure: PressureSectionJson {
                    pressure: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                    goal: vec!["-1.0".into(), "-1.0".into(), "-1.0".into()]
                },
                flow: FlowSectionJson {
                    flow: vec!["0.0".into(), "3.26".into(), "3.98".into()],
                    by_weight: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                    by_weight_raw: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                    goal: vec!["-1.0".into(), "6.75".into(), "7.1875".into()]
                },
                temperature: TemperatureSectionJson {
                    basket: vec!["87.0".into(), "85.07".into(), "85.37".into()],
                    mix: vec!["87.0".into(), "87.17".into(), "87.14".into()],
                    goal: vec!["87.0".into(), "87.0".into(), "87.0".into()]
                },
                totals: TotalsSectionJson {
                    weight: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                    water_dispensed: vec![
                        "0.0".into(),
                        "0.08161010742187501".into(),
                        "0.181060791015625".into()
                    ]
                },
                resistance: ResistanceSectionJson {
                    resistance: vec!["0.0".into(), "0.0".into(), "0.0".into()],
                    by_weight: vec!["0.0".into(), "0.0".into(), "0.0".into()]
                },
                state_change: vec!["0.0".into(), "10000000.0".into(), "10000000.0".into()]
            })
        );
    }
}
