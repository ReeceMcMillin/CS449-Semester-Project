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
            <>
            <div style="display: flex;">
                <div style="margin-top: 33%;">
                    <PlayerSettings player=PlayerTurn::PlayerOne>{""}</PlayerSettings>
                </div>
                <div style="margin: 20px; margin-bottom: 0px;">
                    <Grid>
                        // self.tiles.iter().map(|x| x.view()).collect::<Html>()
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
                </div>
                <div style="margin-top: 33%;">
                    <PlayerSettings player=PlayerTurn::PlayerTwo>{""}</PlayerSettings>
                </div>
            </div>
            <RecordGameInput>{""}</RecordGameInput>
            </>
        }
    }
}