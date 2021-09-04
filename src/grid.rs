use yew::prelude::*;

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
                        self.props.value = match self.props.turn {
                            PlayerTurn::PlayerOne => Some(TileValue::S),
                            PlayerTurn::PlayerTwo => Some(TileValue::O),
                        };
                        self.props.set = true;
                        self.props.grid_link.as_ref().unwrap().callback(|_| Message::FlipTile ).emit("");
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
            true => match self.props.value.clone().unwrap() {
                TileValue::S => "thistle",
                TileValue::O => "peachpuff",
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
            PlayerTurn::PlayerOne => "S",
            PlayerTurn::PlayerTwo => "O",
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
                <p>{ &self.props.message }</p>
            </>
        }
    }
}
