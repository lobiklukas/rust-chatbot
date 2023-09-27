use leptos::{html::Input, *};

#[component]
pub fn TypeArea(
    send: Action<String, Result<(), ServerFnError>>,
    is_loading: ReadSignal<bool>,
) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();

    let v = move || is_loading.get();

    view! {
        <form class="flex w-full p-4" on:submit=move |ev| {
                ev.prevent_default();
                // if is_loading.get() == false {
                    let input = input_ref.get().expect("input to exist");
                    send.dispatch(input.value());
                    input.set_value("");
                // }
           }
           >
            <input
            node_ref=input_ref
            id="text-input"
            disable={is_loading.get()}
            class="form-input w-full bg-transparent border rounded border-slate-400"
            type="text"
            placeholder="Type here"
            />
            <button class="px-4 py-2 text-white" type="submit">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
            <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75" />
            </svg>
            </button>
        </form>
    }
}
