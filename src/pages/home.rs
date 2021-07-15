use yew::{html, Component};

pub struct HomePage;

impl Component for HomePage {
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
            <div class="home-page">
                <div class="stars-animation" />
                <div class="stats2-animation" />
                <div class="stars3-animation" />
                <div class="home-page__flex">
                    <div class="home-page__content">
                        <h1 class="home-page__content__title">{"Takasaki"}</h1>
                        <div class="home-page__content__links">
                            <a href="https://github.com/Takasakiii">
                                <img src="svgs/github-brands.svg" alt="Logo do GitHub"/>
                            </a>
                            <a href="https://www.linkedin.com/in/lucas-mendes-campos-30a3891a3/">
                                <img src="svgs/linkedin-brands.svg" alt="Logo do LinkedIn" />
                            </a>
                            <a href="https://matrix.to/#/@takasaki:matrix.org">
                                <img class="home-page__content__links--matrix" src="svgs/matrix-logo.svg" alt="Logo do projeto matrix" />
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
