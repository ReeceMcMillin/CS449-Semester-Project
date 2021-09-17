use yew::prelude::*;

#[derive(Debug, Clone)]
pub enum PlayStyle {
    Human,
    Computer
}

pub struct RecordGameInput {
    props: RecordGameProps,
    link: ComponentLink<Self>
}

#[derive(Debug, Clone, Properties)]
pub struct RecordGameProps {
    pub children: Children,
    #[prop_or(false)]
    pub record_game: bool
}

impl Component for RecordGameInput {
    type Message = ();
    type Properties = RecordGameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input type="checkbox" id="record" name="record"/>
                <label for="record">{ "Record Game" }</label>
            </div>
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerSettings {
    props: PlayerSettingsProps,
    link: ComponentLink<Self>
}

#[derive(Debug, Clone, Properties)]
pub struct PlayerSettingsProps {
    pub children: Children,
    #[prop_or(PlayStyle::Human)]
    pub play_style: PlayStyle,
    #[prop_or(TileValue::S)]
    pub tile_value: TileValue,
    #[prop_or(PlayerTurn::PlayerOne)]
    pub player: PlayerTurn
}

pub enum PlayerSettingsMsg {

}

impl Component for PlayerSettings {
    type Message = PlayerSettingsMsg;
    type Properties = PlayerSettingsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let (player_id, player_name) = match self.props.player {
            PlayerTurn::PlayerOne => ("player_one", "Player One"),
            PlayerTurn::PlayerTwo => ("player_two", "Player Two"),
        };

        html! {
            <>
            <div id=player_id style="font-size: 1em; width: 110px; margin-bottom: 50px;">
            <span class="name" style="margin-bottom: 50px;" text-align="center">{ player_name }</span><br/>
            <input type="radio" name=format!("play_style_{}", player_id) value="Human" style="margin-bottom: 8px;"/>
            <label for="human">{ "Human" }</label><br/>
            <input type="radio" name=format!("tile_value_{}", player_id) value="S" style="margin-left: 30px; margin-bottom: 8px;"/>
            <label for="s">{ "S" }</label><br/>
            <input type="radio" name=format!("tile_value_{}", player_id) value="O" style="margin-left: 30px; margin-bottom: 8px;"/>
            <label for="o">{ "O" }</label><br/>
            <input type="radio" name=format!("play_style_{}", player_id) value="Computer" style="margin-bottom: 8px;"/>
            <label for="computer">{ "Computer" }</label>
            </div>
            </>
        }   
    }
}

#[derive(Debug, Clone)]
pub enum PlayerTurn {
    PlayerOne,
    PlayerTwo,
}

#[derive(Debug, Clone)]
pub enum Message {
    FlipTile,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileValue {
    S,
    O,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub props: TileProps,
    pub link: ComponentLink<Self>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (&self.props.value, &other.props.value) {
            (Some(self_value), Some(other_value)) => self_value == other_value,
            _ => false
        }
    }
}

#[derive(Debug, Properties, Clone)]
pub struct TileProps {
    pub children: Children,
    #[prop_or_default(None)]
    value: Option<TileValue>,
    #[prop_or(false)]
    set: bool,
    #[prop_or(PlayerTurn::PlayerOne)]
    pub turn: PlayerTurn,
    #[prop_or(None)]
    pub set_by: Option<PlayerTurn>,
    #[prop_or(None)]
    pub grid_link: Option<ComponentLink<Grid>>
}

impl Component for Tile {
    type Message = Message;
    type Properties = TileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::FlipTile => {
                match self.props.set {
                    true => (),
                    false => {
                        // This should match against a radio button selection
                        self.props.value = match self.props.turn {
                            PlayerTurn::PlayerOne => Some(TileValue::S),
                            PlayerTurn::PlayerTwo => Some(TileValue::O),
                        };
                        self.props.set = true;
                        self.props.grid_link.as_ref().unwrap().callback(|_| Message::FlipTile ).emit("");
                        self.props.set_by = Some(self.props.turn.clone());
                    }
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.turn = props.turn;
        true
    }

    fn view(&self) -> Html {
        let color = match self.props.set {
            false => "powderblue",
            true => match self.props.set_by.clone().unwrap() {
                PlayerTurn::PlayerOne => "thistle",
                PlayerTurn::PlayerTwo => "peachpuff",
            },
        };
        let button_style = format!("height:50px; width:50px; padding:0px; background-color:{}; margin:5px; border-radius: 15px; border: none;", color);
        let text = match &self.props.value {
            Some(v) => match v {
                TileValue::S => "S",
                TileValue::O => "O",
            },
            None => "\u{2063}",
        };
        
        let onclick = {
            self.link.callback(move |_: MouseEvent| Message::FlipTile)
        };
        html! {
            <>
                <button class="tile" style=button_style.clone() onclick=onclick> { text } </button>
            </>
        }
    }
}

pub struct Grid {
    props: GridProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Properties)]
pub struct GridProps {
    pub children: ChildrenWithProps<Tile>,
    #[prop_or(vec![])]
    pub items: Vec<Tile>,
    #[prop_or(PlayerTurn::PlayerOne)]
    pub turn: PlayerTurn,
    #[prop_or(String::new())]
    pub message: String
}

impl Component for Grid {
    type Message = Message;
    type Properties = GridProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::FlipTile => {
                self.props.turn = match self.props.turn {
                    PlayerTurn::PlayerOne => PlayerTurn::PlayerTwo,
                    PlayerTurn::PlayerTwo => PlayerTurn::PlayerOne,
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let color = match self.props.turn {
            PlayerTurn::PlayerOne => "thistle",
            PlayerTurn::PlayerTwo => "peachpuff",
        };
        let button_style = format!("width: 100%; height: 40px; padding:0px; background-color:{}; margin:0px; margin-bottom: 20px; border-radius: 15px; border: none;", color);
        let player = match self.props.turn {
            PlayerTurn::PlayerOne => "Player One",
            PlayerTurn::PlayerTwo => "Player Two",
        };
        html! {
            <>
                // <p>{ "Select Player" }</p>
                <p>{ "Active Player" }</p>
                <div style="width: 540px; position: relative">
                <button style=button_style class="choose">{ player }</button>
                <br/>

                    {for self.props.children.iter().map(|mut item| {
                        item.props.turn = self.props.turn.clone();
                        item.props.grid_link = Some(self.link.clone());
                        item
                    })}
                </div>
                // <p>{ &self.props.message }</p>
            </>
        }
    }
}
