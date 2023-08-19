use leptos::*;
use leptos_router::*;
use super::model::AuthContext;

use super::backend::token_request;
// TODO maybe make this customizable for the consuming application
#[component]
pub fn AuthCallback() -> impl IntoView {
    let query = move || use_query_map().get();
    let code = query().get("code").unwrap().to_owned();
    let token_resource =
        create_blocking_resource(|| (), move |_| token_request(code.clone()));

    // token_resource.with(|token| {
    //     log!(token.to_string());
    // });
    view! {
        <h1>"Auth Callback"</h1>
        <Suspense fallback=|| ()>
            {move || {
                token_resource
                    .with(|token| {
                            let Ok(auth_complete) = token else {
                            return view! { <div>"Nothing"</div> }.into_view();
                        };
                            view! {
                                <div>"Token Received: " {format!("{:?}", auth_complete)}</div>
                            }
                                .into_view()
                        },
                    )
            }}
        </Suspense>
    }
}

// #[component]
// pub fn AuthWall(
//     not_authenticated_component: fn() -> dyn IntoView,
//     authenticated_component: fn() -> dyn IntoView,
// ) -> impl IntoView {
//     let auth_context = use_context::<ReadSignal<AuthContext>>();

//     view! {
//         <Show when=move || auth_context.is_some()
//             fallback=not_authenticated_component
//             >
//             {authenticated_component()}
//         </Show>
//     }
// }
