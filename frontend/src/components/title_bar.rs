use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use web_sys::{Element, MouseEvent, window, Document};

#[derive(PartialEq, Properties)]
pub struct TitleBarProps {
    pub title: String,
    pub children: Children,

}

#[function_component(TitleBar)]
pub fn title_bar(props: &TitleBarProps) -> Html {
    html! {
        <div class="titlebar">
            <div class="buttons">
            {props.children.clone()}
            </div>
            {props.title.clone()}
          </div>
    }
}