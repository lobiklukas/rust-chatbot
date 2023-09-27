use leptos::{ *, html::Div };

use crate::model::conversation::Conversation;

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>, is_loading: ReadSignal<bool>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();

    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto p-5" node_ref=chat_div_ref>
        {move || conversation.get().messages.iter().map(move |message| {
            let msg_class =  if message.user {"bg-blue-500 max-w-md p-4 mb-5 rounded-lg self-end"} else {"bg-slate-600 max-w-md p-4 mb-5 rounded-lg self-start"};
            view! {
              <div class={msg_class}>
                {message.text.clone()}
              </div>
            }
          }).collect::<Vec<_>>()
        }
      </div>
  }
}
