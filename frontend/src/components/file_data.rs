use crate::backend::create_element;
use crate::backend::update_element;
use editor::Editor;
use editor::EditorChange;
use shared::id::Id;
use shared::log;
use shared::schema::{Attrs, EditorElement, ElementTree, FileDirectory};
use yew::suspense::use_future_with_deps;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::backend::create_element_tree;
use crate::backend::get_element_tree;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Id,
}

fn onchange_element_tree(element_tree: Rc<RefCell<ElementTree>>) -> Callback<EditorChange> {
    Callback::from(move |e| {
        match e {
            EditorChange::Update(x) => {
                log!(&x);
                let update_data = x.clone();
                spawn_local(async move {
                    let x = update_element(update_data).await;
                    log!(x);
                });
                if let Some(element) = element_tree
                    .as_ref()
                    .borrow_mut()
                    .elements
                    .vertices
                    .get_mut(&x.id)
                {
                    if let Some(text) = x.text {
                        element.text = text;
                    }
                    if let Some(attrs) = x.attrs {
                        element.attrs = attrs;
                    }
                }
            }
            EditorChange::Create(x) => {
                log!(&x);
                let create_data = x.clone();
                spawn_local(async move {
                    let result = create_element(create_data).await;
                    log!(result);
                });
                element_tree.as_ref().borrow_mut().elements.push_children(
                    x.parent_id.clone(),
                    x.id.clone(),
                    x.clone().into(),
                );
                //if let Some(prev_element_id) = x.prev_element_id{
                    //let mut element_tree = element_tree.as_ref().borrow_mut();
                    //let children_list_of_parent_element = element_tree.elements.adjacency.get_mut(&x.parent_id).unwrap();
                    //let index_of_prev_element  = children_list_of_parent_element.get_index_of(&prev_element_id).unwrap();
                    //let index_of_last_element =  children_list_of_parent_element.get_index_of(&x.id).unwrap();
                    //children_list_of_parent_element.move_index(index_of_last_element, index_of_prev_element + 1);
                    ////log!(element_tree.elements.adjacency.get(&x.parent_id));
                //}
            }
            _ => {}
        };
    })
}

#[function_component(FileData)]
pub fn file_data(props: &Props) -> HtmlResult {
    let dispatch = Dispatch::<FileDirectory>::new();
    let element_tree: Rc<RefCell<ElementTree>>;
    // TODO : create a hook for this
    let res = use_future_with_deps(|file_id| async move {
        match dispatch.get().files.vertices.get(&file_id) {
            Some(x) => {
                match x.element_tree {
                    Some(tree_id) => {
                        return get_element_tree(&tree_id).await;
                    }
                    None => {
                        // create new element_tree
                        let mut r = ElementTree::default();
                        let root = r.elements.root.unwrap();
                        let id: Id = Uuid::new_v4().into();
                        r.elements.push_children(
                            root,
                            id.clone(),
                            EditorElement::new(
                                id,
                                "bold text".to_string(),
                                HashMap::from([(Attrs::Style, "font-weight: bold;".to_string())]),
                            ),
                        );
                        let id: Id = Uuid::new_v4().into();
                        r.elements.push_children(
                            root,
                            id,
                            EditorElement::new(
                                id,
                                r#"Element is here."#.to_string(),
                                HashMap::new(),
                            ),
                        );
                        let _ = create_element_tree(&r, *file_id).await?;
                        let tree_id = r.id;
                        dispatch.reduce_mut(|f| {
                            let file_node = f.files.vertices.get_mut(&file_id).unwrap();
                            file_node.id = tree_id;
                        });
                        return Ok(r);
                    }
                };
            }
            None => return Err(String::from("Not found!")),
        }
    }, props.id)?;
    let result_html = match *res {
        Ok(ref tree) => {
            let file_node = Dispatch::<FileDirectory>::new()
                .get()
                .files
                .vertices
                .get(&props.id)
                .unwrap()
                .clone();
            element_tree = Rc::new(RefCell::new(tree.clone()));
            html! {
                <Editor
                    title = { file_node.name.clone() }
                element_tree = { element_tree.clone() }
                onchange = { onchange_element_tree(element_tree.clone())}
                />
            }
        }
        Err(ref failure) => {
            log!(failure);
            failure.to_string().into()
        }
    };
    Ok(result_html)
}
