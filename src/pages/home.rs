use crate::Item;
use leptos::{ev::SubmitEvent, *};
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    enum ItemError {
        Empty,
    }

    enum InputError {
        Item(ItemError),
    }

    fn validate(data: AddItem) -> Result<AddItem, InputError> {
        let item = data.item.trim();

        if item.is_empty() {
            return Err(InputError::Item(ItemError::Empty));
        }

        Ok(AddItem {
            item: item.to_owned(),
        })
    }

    let items = create_blocking_resource(cx, || (), move |_| async move { get_items(cx).await });
    let add_item = create_server_action::<AddItem>(cx);

    let (item_input, set_item_input) = create_signal(cx, String::new());

    create_effect(cx, move |_| {
        if !add_item.pending().get() {
            items.refetch();
        }
    });

    let on_add_item = move |ev: SubmitEvent| {
        ev.prevent_default();

        match AddItem::from_event(&ev) {
            Ok(input) => match validate(input) {
                Ok(input) => {
                    add_item.dispatch(input);
                }
                Err(InputError::Item(ItemError::Empty)) => {
                    log!("Input cannot be empty");
                }
            },
            Err(_) => {
                log!("Error submitting form");
            }
        }
    };

    let show_items = move |items: Vec<Item>| {
        view! { cx,
            <ul>
                <For
                    each=move || items.clone()
                    key=|item| item.id
                    view=move |_, item| view! { cx,
                        <li>
                            {item.text}
                        </li>
                    }
                />
            </ul>
        }
        .into_view(cx)
    };

    view! { cx,
        <Transition
            fallback=move || view! { cx, <p>"Loading"</p> }
        >
            {move || {
                items.with(cx, move |items| match items {
                    Ok(items) => show_items(items.clone()),
                    Err(_) => view! { cx, <p>"Error"</p> }.into_view(cx),
                })
            }}
        </Transition>
        <ActionForm action=add_item on:submit=on_add_item>
            <input
                type="text"
                name="item"
                on:input=move |ev| set_item_input.set(event_target_value(&ev))
                prop:value=item_input
            />
            <button type="submit">"Submit"</button>
        </ActionForm>
    }
}

#[server(AddItem, "/api")]
async fn add_item(cx: Scope, item: String) -> Result<(), ServerFnError> {
    use crate::ItemStore;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Mutex;

    let res = extract(cx, |items: Data<Mutex<ItemStore>>| async move {
        let mut items = items.lock()?;
        items.add(item);
        Ok(())
    })
    .await;

    match res {
        Ok(res) => res,
        Err(err) => Err(err),
    }
}

#[server(GetItems, "/api")]
async fn get_items(cx: Scope) -> Result<Vec<Item>, ServerFnError> {
    use crate::ItemStore;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use std::sync::Mutex;

    let res = extract(cx, |items: Data<Mutex<ItemStore>>| async move {
        let items = items.lock()?;
        Ok(items.items().to_owned())
    })
    .await;

    match res {
        Ok(res) => res,
        Err(err) => Err(err),
    }
}
