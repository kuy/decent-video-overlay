use crate::components::Title;
use crate::prelude::*;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub title: Option<String>,
}

pub struct Page;

impl Component for Page {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <>
            <Title text={ctx.props().title.clone()} />
            <header>
              <div class={css!(r#"
            display: flex;
            margin: 0 auto;
            width: 1024px;
            align-items: center;

            a {
              margin: 4px 14px 0;
            }
          "#)}>
                <Link to={Route::Player}>{ "Player" }</Link>
                <Link to={Route::About}>{ "About" }</Link>
              </div>
            </header>
            <main>
              <div class={css!(r#"
                margin: 16px auto 0;
                width: 1024px;
              "#)}>
                { for ctx.props().children.iter() }
              </div>
            </main>
          </>
        }
    }
}
