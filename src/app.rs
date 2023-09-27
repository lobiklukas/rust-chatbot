use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Welcome to Leptos"/>
        <div class="h-screen w-full flex flex-col justify-center items-center">
            <TextInput/>
        </div>
    }
}

#[component]
fn TextInput(cx: Scope) -> impl IntoView {
    view! { cx,
        <label class="text-left text-white" for="text-input">
            What image you want to generate?
        </label>
        <input
            id="text-input"
            class="form-input w-full max-w-lg bg-transparent border rounded border-slate-400"
            type="text"
            placeholder="What image you want you want to "
        />
    }
}
