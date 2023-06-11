use crate::prelude::*;
use crate::{
    components::INNER,
    libs::{
        models::{
            json::{ShotDataJson, SHOT1},
            DataPoint, PressureData, ShotData,
        },
        scale, Range,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub elapsed: f64,
}

pub struct Chart {
    data: PressureData,
    span: Range,
}

impl Chart {
    fn render_pressure(&self, elapsed: f64) -> String {
        let mut buf = String::default();
        let x = scale(self.span.as_tuple(), (0., INNER.0));
        let y = scale(self.data.range.as_tuple(), (INNER.1, 10.0));

        let first = self.data.series.first().unwrap();
        if let DataPoint::Present((t, v)) = first {
            buf = format!("M{} {} ", x(*t), y(*v));
        } else {
            // NOWAY!
        }
        for dp in self.data.series.iter().skip(1) {
            if dp.t() > (elapsed * 0.001) as f32 {
                break;
            }
            match dp {
                DataPoint::Present((t, v)) => {
                    buf.push_str(format!("L{} {} ", x(*t), y(*v)).as_str())
                }
                _ => (),
            }
        }
        buf
    }
}

impl Component for Chart {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        let shot_data: ShotData = serde_json::from_str::<ShotDataJson>(SHOT1).unwrap().into();
        let data = PressureData::from_shot_data(&shot_data);

        Self {
            data,
            span: Range::from_series(&shot_data.elapsed),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <svg width={ format!("{}", INNER.0) } height={ format!("{}", INNER.1) } viewBox={ format!("0 0 {} {}", INNER.0, INNER.1) } xmlns="http://www.w3.org/2000/svg">
                <g stroke="darkgreen" stroke-width="1.5px" stroke-linecap="round" stroke-linejoin="round" fill="transparent">
                    <path d={ self.render_pressure(ctx.props().elapsed) } />
                </g>
            </svg>
        }
    }
}
