use crate::templates::TemplateRegistry;
use crate::types::CardData;
use leptos::*;

#[component]
pub fn Dashboard(
    card_data: ReadSignal<CardData>,
    set_card_data: WriteSignal<CardData>,
    on_generate: impl Fn() + 'static + Copy,
) -> impl IntoView {
    let registry = TemplateRegistry::new();
    let templates = registry.list_all();

    let update_title = move |ev| {
        let val = event_target_value(&ev);
        set_card_data.update(|d| d.title = val);
    };

    let update_artist = move |ev| {
        let val = event_target_value(&ev);
        set_card_data.update(|d| d.artist = val);
    };

    view! {
        <div class="max-w-4xl mx-auto p-6 space-y-6">
            <header class="border-b border-gray-800 pb-4">
                <h1 class="text-3xl font-bold bg-gradient-to-r from-blue-400 to-indigo-500 bg-clip-text text-transparent">
                    "Kartu Musik Generator"
                </h1>
                <p class="text-sm text-gray-400 mt-1">"Pilih template dan kustomisasi konten kartu musik Anda."</p>
            </header>

            <section class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div class="space-y-2">
                    <label class="block text-xs font-semibold text-gray-400">"Judul Lagu"</label>
                    <input 
                        type="text" 
                        value=move || card_data.get().title
                        on:input=update_title
                        class="w-full bg-gray-900 border border-gray-800 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-blue-500"
                    />
                </div>
                <div class="space-y-2">
                    <label class="block text-xs font-semibold text-gray-400">"Nama Artis"</label>
                    <input 
                        type="text" 
                        value=move || card_data.get().artist
                        on:input=update_artist
                        class="w-full bg-gray-900 border border-gray-800 rounded-lg px-3 py-2 text-sm text-white focus:outline-none focus:border-blue-500"
                    />
                </div>
            </section>

            <section>
                <h2 class="text-sm font-semibold text-gray-400 mb-3">"Pilih Template Pemutar"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {templates.into_iter().map(|tpl| {
                        let id = tpl.id();
                        let is_selected = move || card_data.get().template_id == id;
                        let id_str = id.to_string();

                        view! {
                            <div 
                                class=move || format!(
                                    "border rounded-xl p-4 cursor-pointer transition-all {}",
                                    if is_selected() { "border-blue-500 bg-blue-950/20" } else { "border-gray-800 hover:border-gray-700 bg-gray-900/40" }
                                )
                                on:click=move |_| set_card_data.update(|d| d.template_id = id_str.clone())
                            >
                                <h3 class="font-bold text-sm text-white">{tpl.name()}</h3>
                                <p class="text-xs text-gray-400 mt-1">{tpl.description()}</p>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <div class="pt-4">
                <button 
                    on:click=move |_| on_generate()
                    class="w-full bg-blue-600 hover:bg-blue-500 font-bold py-3 rounded-xl shadow-lg transition-all"
                >
                    "Buka Studio Player & Ekspor"
                </button>
            </div>
        </div>
    }
}
