use yew::{create_portal, function_component, html, Properties};

use crate::prelude::*;

static TITLE: &str = "Decent Video Overlay";

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    #[prop_or_default]
    pub text: Option<String>,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let el = gloo_utils::document()
        .query_selector("head > title")
        .expect("Missing <title> element")
        .unwrap();

    if let Some(text) = &props.text {
        create_portal(html! { format!("{} | {}", text, TITLE) }, el)
    } else {
        create_portal(html! { TITLE }, el)
    }
}
