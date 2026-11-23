use crate::templates::TemplateRegistry;
use crate::types::CardData;
use leptos::*;

#[component]
pub fn Player(
    card_data: ReadSignal<CardData>,
    on_back: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let registry = TemplateRegistry::new();
    let current_template = move || registry.get(&card_data.get().template_id);

    view! {
        <div class="min-h-screen bg-black flex flex-col items-center justify-center p-4 relative">
            <button 
                on:click=move |_| on_back()
                class="absolute top-6 left-6 text-xs bg-gray-800 hover:bg-gray-700 text-white px-4 py-2 rounded-lg transition-all"
            >
                "← Kembali ke Dashboard"
            </button>

            <div class="my-auto">
                {move || match current_template() {
                    Some(tpl) => tpl.render_view(&card_data.get()),
                    None => view! { <div class="text-red-500">"Template tidak ditemukan."</div> }.into_view(),
                }}
            </div>
        </div>
    }
}
