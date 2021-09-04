mod grid;

use grid::*;
use yew::prelude::*;

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Grid>
                // <div class="radio_container">
                    // <span class="radio_button"><Radio radio_value="S">{"S"}</Radio></span>
                    // <label for="S">{ "S" }</label>
                    // <span class="radio_button"><Radio radio_value="O">{"O"}</Radio></span>
                    // <label for="O">{ "O" }</label>
                // </div> 
                // { self.children.props.items.iter().map(|x| x.view()).collect::<Html>() }
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
                <Tile>{""}</Tile>
            </Grid>
        }
    }
}