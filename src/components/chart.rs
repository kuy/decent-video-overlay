use std::rc::Rc;

use crate::prelude::*;
use crate::{
    components::INNER,
    libs::{
        models::{ChartData, DataPoint},
        scale, Range,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: Rc<ChartData>,
    pub data_codomain: (f32, f32),
    pub time_span: Range,
    pub elapsed: f64,
    pub color: &'static str,
}

pub struct Chart {
    data_domain: (f32, f32),
    time_domain: (f32, f32),
}

impl Chart {
    fn render_svg_path(&self, data: &ChartData, elapsed: f64, data_codomain: (f32, f32)) -> String {
        let mut buf = String::default();

        let x = scale(self.time_domain, (0., INNER.0));
        let y = scale(self.data_domain, data_codomain);

        let first = data.series.first().unwrap();
        if let DataPoint::Present((t, v)) = first {
            buf = format!("M{} {} ", x(*t), y(*v));
        } else {
            // NOWAY!
        }

        for dp in data.series.iter().skip(1) {
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data_domain: ctx.props().data.range.as_tuple(),
            time_domain: ctx.props().time_span.as_tuple(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <g stroke={ ctx.props().color } stroke-width="1.5px" stroke-linecap="round" stroke-linejoin="round" fill="transparent">
                <path d={ self.render_svg_path(&ctx.props().data, ctx.props().elapsed, ctx.props().data_codomain) } />
            </g>
        }
    }
}
