mod home;

use yew::{html, Component};
use yew_router::{router::Router, Switch};

#[derive(Switch, Clone)]
enum AppRouter {
    #[to = "/"]
    Home,
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> yew::ShouldRender {
        false
    }

    fn create(_props: Self::Properties, _link: yew::ComponentLink<Self>) -> Self {
        Self
    }

    fn view(&self) -> yew::Html {
        html! {
            <Router<AppRouter, ()>
                render = Router::render(|switch: AppRouter| {
                    match switch {
                        AppRouter::Home => html!{<home::HomePage />}
                    }
                })
            />
        }
    }
}
