use gloo::timers::callback::Interval;

use crate::libs::{models::{
    json::{ShotDataJson, SHOT1},
    ShotData, PressureData, DataPoint,
}, scale, Range};
use crate::prelude::*;

pub enum Msg {
    Tick,
}

pub struct Chart {
    _standalone: Interval,
    started: f64,
    elapsed: f64,
    data: PressureData,
    span: Range,
}

const INNER: (f32, f32) = (800.0, 450.0);

impl Chart {
    fn get_timestamp() -> f64 {
        js_sys::Date::now() as f64
    }

    fn render_pressure(&self) -> String {
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
            if dp.t() > (self.elapsed * 0.001) as f32 {
                break;
            }
            match dp {
                DataPoint::Present((t, v)) => buf.push_str(format!("L{} {} ", x(*t), y(*v)).as_str()),
                _ => ()
            }
        }
        buf
    }
}

impl Component for Chart {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let shot_data: ShotData = serde_json::from_str::<ShotDataJson>(SHOT1).unwrap().into();
        let data = PressureData::from_shot_data(&shot_data);

        let tick_handle = {
            let link = ctx.link().clone();
            Interval::new(50, move || link.send_message(Msg::Tick))
        };

        Self {
            _standalone: tick_handle,
            started: Self::get_timestamp(),
            elapsed: 0.0,
            data,
            span: Range::from_series(&shot_data.elapsed),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Tick => {
                self.elapsed = Self::get_timestamp() - self.started;
                true
            }
        }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <>
                <div>{ format!("Elapsed: {} ms", self.elapsed) }</div>
                <svg width={ format!("{}", INNER.0) } height={ format!("{}", INNER.1) } viewBox={ format!("0 0 {} {}", INNER.0, INNER.1) } xmlns="http://www.w3.org/2000/svg">
                    <g transform={ format!("translate(0, {})", 0) }><path d={ self.render_pressure() } stroke="darkgreen" stroke-width="1.5px" fill="transparent"/></g>
                </svg>
            </>
        }
    }
}
