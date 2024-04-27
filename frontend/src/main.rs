mod api;

use leptos::*;
use api::{summarize, Dish, SummarizeError};

fn main() {
    mount_to_body(|| {
        view! { <App/> }
    })
}

#[component]
fn App() -> impl IntoView {
    // On dispatch, call backend endpoint
    let summarize_action = create_action(|video_id: &String| {
        let video_id = video_id.to_owned();
        async move { summarize(&video_id).await }
    });

    // On input submission, dispatch video URL
    let input_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();

        let input = input_element().unwrap();
        summarize_action.dispatch(input.value());
    };

    view! {
        <h1>"CuickCook 🍽️"</h1>
        <form on:submit=on_submit>
            <input
                type="text"
                placeholder="Enter YouTube Link 📺"
                node_ref=input_element
                disabled=summarize_action.pending()
            />
        </form>
        <SummaryField summarize_action=summarize_action/>
    }
}

#[component]
fn SummaryField(
    summarize_action: Action<String, Result<Vec<Dish>, SummarizeError>>
) -> impl IntoView {
    let dishes = move || summarize_action.value().get();
    let pending = move || summarize_action.pending().get();

    view! {
        <div class="summary-field">
            {move || {
                // Display loading message if waiting for endpoint response
                if pending() {
                    view! { <h1>"Summarizing..."</h1> }.into_view()
                // Endpoint has responded
                } else if let Some(dishes) = dishes() {
                    match dishes {
                        // Display dishes if successful summarization
                        Ok(dishes) => {
                            dishes.into_iter().enumerate().map(|(idx ,dish)| {
                                    view! {<Dish dish=dish delay=idx/>}
                            }).collect_view()
                        }
                        // Display error if failed to summarize
                        Err(err) => match err {
                            SummarizeError::InvalidUrl =>
                                view! {<h1>"Invalid URL"</h1>}.into_view(),
                            SummarizeError::BadResponse =>
                                view! {<h1>"Failed to summarize dishes"</h1>}.into_view(),
                        }
                    }
                // Display nothing endpoint hasn't been called
                } else {
                    view! {}.into_view()
                }
            }}
        </div>
    }
}

#[component]
fn Dish(dish: Dish, #[prop(optional)] delay: usize) -> impl IntoView {
    view! {
        <div class="dish" style=format!("animation-delay:{}s", delay as f32 * 0.1)>
            <h2>{dish.name}</h2>
            <h3>"🛍️ Ingredients"</h3>
            <ul>
                {dish.ingredients.into_iter()
                    .map(|ingr| view! { <li>{ingr}</li> })
                    .collect_view()}
            </ul>
            <h3>"📝 Instructions"</h3>
            <ol>
                {dish.instructions.into_iter()
                    .map(|instr| view! { <li>{instr}</li> })
                    .collect_view()}
            </ol>
        </div>
    }
}
