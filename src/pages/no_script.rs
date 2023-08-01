use leptos::*;
use leptos_router::*;

#[component]
pub fn NoScript(cx: Scope) -> impl IntoView {
    let path = use_route(cx).path();
    let query = use_query_map(cx);

    create_effect(cx, move |_| {
        log!("hi {}", query.get().to_query_string());
    });

    let redirect_to = move || {
        format!(
            "static/{path}{}",
            if query.get().0.len() > 0 {
                query.get().to_query_string()
            } else {
                String::new()
            }
        )
    };

    view! { cx,
        <noscript>
            <p>
                "Looks like JavaScript is not supported/enabled on your device."
                <br />
                "Click "
                <A href=redirect_to>"here"</A>
                " to continue to a compatible version of this page."
            </p>
        </noscript>
        <Outlet />
    }
}
