use std::rc::Rc;

use gloo::timers::callback::Interval;

use crate::libs::models::ChartData;
use crate::libs::Range;
use crate::prelude::*;
use crate::{
    components::Chart,
    libs::models::{
        json::{ShotDataJson, SHOT1},
        ShotData,
    },
};

pub enum Msg {
    PlayOrPause,
    Stop,
    Tick,
}

pub enum State {
    Stopped,
    Playing(TimerState),
    Paused(TimerState),
}

#[derive(Debug)]
pub struct TimerState {
    pub ticked: f64,
    pub elapsed: f64,
}

impl Default for TimerState {
    fn default() -> Self {
        Self {
            ticked: Self::get_timestamp(),
            elapsed: 0.0,
        }
    }
}

impl TimerState {
    fn get_timestamp() -> f64 {
        js_sys::Date::now()
    }

    pub fn into_pause(&self) -> Self {
        let t = Self::get_timestamp();
        Self {
            ticked: t,
            elapsed: self.elapsed + (t - self.ticked),
        }
    }

    pub fn into_playing(&self) -> Self {
        Self {
            ticked: Self::get_timestamp(),
            elapsed: self.elapsed,
        }
    }

    pub fn tick(&self) -> Self {
        let t = Self::get_timestamp();
        Self {
            ticked: t,
            elapsed: self.elapsed + (t - self.ticked),
        }
    }
}

pub struct Graph {
    state: State,
    handle: Option<Interval>,
    data: ShotData,
    time_span: Range,
    pressure_data: Rc<ChartData>,
}

pub const INNER: (f32, f32) = (800.0, 450.0);
const TIMER_DURATION: u32 = 75;

impl Graph {
    fn is_playing(&self) -> bool {
        matches!(self.state, State::Playing(_))
    }

    fn elapsed_or_zero(&self) -> f64 {
        match &self.state {
            State::Stopped => 0.0,
            State::Playing(ts) => ts.elapsed,
            State::Paused(ts) => ts.elapsed,
        }
    }

    fn render_timer(&self) -> String {
        match &self.state {
            State::Stopped => format!("Elapsed: -"),
            State::Playing(ts) => format!("Elapsed: {} ms", ts.elapsed),
            State::Paused(ts) => format!("Elapsed: {} ms", ts.elapsed),
        }
    }
}

impl Component for Graph {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let data: ShotData = serde_json::from_str::<ShotDataJson>(SHOT1).unwrap().into();
        let time_span = Range::from_series(&data.elapsed);
        let pressure_data = Rc::new(ChartData::for_pressure(&data));

        Self {
            state: State::Stopped,
            handle: None,
            data,
            time_span,
            pressure_data,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayOrPause => {
                let tick_handle = {
                    let link = ctx.link().clone();
                    Interval::new(TIMER_DURATION, move || link.send_message(Msg::Tick))
                };
                match &self.state {
                    State::Stopped => {
                        self.state = State::Playing(TimerState::default());
                        self.handle = Some(tick_handle);
                    }
                    State::Playing(ts) => {
                        self.state = State::Paused(ts.into_pause());
                        self.handle = None;
                    }
                    State::Paused(ts) => {
                        self.state = State::Playing(ts.into_playing());
                        self.handle = Some(tick_handle);
                    }
                }
                true
            }
            Msg::Stop => {
                self.state = State::Stopped;
                self.handle = None;
                true
            }
            Msg::Tick => {
                if let State::Playing(ts) = &self.state {
                    self.state = State::Playing(ts.tick());
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                    <button onclick={ctx.link().callback(|_| Msg::PlayOrPause)}>{ if self.is_playing() { "Pause" } else { "Play" } }</button>
                    <button onclick={ctx.link().callback(|_| Msg::Stop)}>{ "Stop" }</button>
                    <span>{ self.render_timer() }</span>
                </div>
                <div>
                    <Chart data={self.pressure_data.clone()} time_span={self.time_span.clone()} elapsed={self.elapsed_or_zero()} />
                </div>
            </>
        }
    }
}
