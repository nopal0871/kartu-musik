use crate::types::CardData;
use crate::views::dashboard::Dashboard;
use crate::views::player::Player;
use leptos::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Page {
    Dashboard,
    Player,
}

#[component]
pub fn App() -> impl IntoView {
    let (card_data, set_card_data) = create_signal(CardData::default());
    let (current_page, set_page) = create_signal(Page::Dashboard);

    view! {
        <main class="min-h-screen">
            {move || match current_page.get() {
                Page::Dashboard => view! {
                    <Dashboard 
                        card_data=card_data 
                        set_card_data=set_card_data 
                        on_generate=move || set_page.set(Page::Player)
                    />
                }.into_view(),
                Page::Player => view! {
                    <Player 
                        card_data=card_data 
                        on_back=move || set_page.set(Page::Dashboard)
                    />
                }.into_view(),
            }}
        </main>
    }
}
