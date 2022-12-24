use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

// use shared::log;
use shared::schema::{FileDirectory, FileNode};

use crate::backend;
use crate::router::{Route, switch};
use crate::specific_components::{ButtonsGroup, SearchFiltes};
use crate::utils::filetree::FileTree;
use crate::utils::GetTitleBar;

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async move {
        // log!(&backend::read().await);
        // log!(JsValue::js_typeof(
        //     &backend::read().await
        // ));
    });
    let aside_bar_toggle = use_state_eq(|| "".to_string());
    let toggle_aside = aside_bar_toggle.clone();
    let file_dispatch = Dispatch::<FileDirectory>::new();
    // only do it once
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let _ = crate::hooks::init_files().await;
            });
            || {}
        },
        (),
    );

    let onclick_market_place: Callback<MouseEvent> = Callback::from(move |_e: MouseEvent| {
        //TODO
        // history.push(Route::File { id: market_page });
    });

    let handle_create_file: Callback<MouseEvent> =
        file_dispatch.reduce_mut_future_callback(|state| {
            Box::pin(async move {
                let file = FileNode::default();
                let x = crate::backend::create_file(
                    state.id,
                    state.files.root.unwrap(),
                    "untitled".to_string(),
                    file.id,
                )
                    .await;
                // console::log_1(&format!("create_file response : {:?}", x).into());
                if x.is_ok() {
                    state
                        .files
                        .push_children(state.files.root.unwrap(), file.id, file);
                }
            })
        });
    html! {
        <BrowserRouter>

            <div id = "app">
            <GetTitleBar toggle = { toggle_aside }/>
        <aside style={(*aside_bar_toggle).clone().to_string()}>

            <SearchFiltes/>

            <ButtonsGroup/>

            <ul  id="myUL">
            <FileTree/>
            <bottom_buttons>
            <button onclick={handle_create_file}><i class="fa-solid fa-plus"></i>{"Add file"}</button>
            <span ><input placeholder="Add from test"/></span>
            <button onclick={onclick_market_place} ><i class="fa-solid fa-globe"></i>{"Market place"}</button>
            </bottom_buttons>

            </ul>
            </aside>
            <main style="margin-top: 35px;">
                <Switch<Route> render= {switch} />
            </main>
            //<Editor title = "text" element_tree = { element_tree }/>
            </div>
        </BrowserRouter>
    }
}
