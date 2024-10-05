use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1 class="p-6 text-4xl">"Welcome to Leptos!"</h1>
        <button class="bg-[#1da1f2] text-white p-2" on:click=on_click>"Click Me: " {count}</button>
    }
}