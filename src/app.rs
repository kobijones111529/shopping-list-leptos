use crate::pages::{home::*, no_script::*, not_found::*};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css" />

        <Title text="Welcome to Leptos" />

        <Router>
            <main>
                <Routes>
                    <Route path="/static" view=Outlet ssr=SsrMode::PartiallyBlocked>
                        <Route path="" view=Home ssr=SsrMode::PartiallyBlocked />
                        <Route path="*any" view=NotFound />
                    </Route>
                    <Route path="/" view=NoScript>
                            <Route path="" view=Home />
                            <Route path="*any" view=NotFound />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
