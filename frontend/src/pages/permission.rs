use crate::backend;
use crate::shared::*;
use shared::{id::Id, schema::FileNode};
use yew::prelude::*;
use yew::suspense::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub file_id: Id,
}

#[function_component]
pub fn Permission(props: &Props) -> Html {
    let file_node: UseStateHandle<Option<FileNode>> = use_state(|| None);

    let _file_node = file_node.clone();
    let _file_id = props.file_id.clone();
    use_future(move || async move {
        let res = backend::get_file(_file_id.clone()).await;
    });

    html! {
        <div class="m-8 flex flex-col gap-8 items-center justify-center">
            <div class="flex flex-row gap-8">
                <div>
                    <input type="radio" id="public" name="permission" value="Public"/>
                    <label class="cursor-pointer" for="public">{"Public"}</label>
                </div>
                <div>
                    <input type="radio" id="private" name="permission" value="Private"/>
                    <label class="cursor-pointer" for="private">{"Private"}</label>
                </div>
                <div>
                    <input type="radio" id="restricted" name="permission" value="Restricted"/>
                    <label class="cursor-pointer" for="restricted">{"Restricted"}</label>
                </div>
            </div>
            <button class="px-4">{"Save"}</button>
        </div>
    }
}
