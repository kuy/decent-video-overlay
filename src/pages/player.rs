use crate::components::{Chart, Content, Graph, Heading};
use crate::prelude::*;

pub struct PlayerPage;

impl Component for PlayerPage {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <Page title="Player">
                <Heading>{ "Player" }</Heading>
                <Content>
                    <Graph />
                </Content>
            </Page>
        }
    }
}
