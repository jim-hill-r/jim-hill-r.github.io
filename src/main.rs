use passionfruitdev::website::Website;
use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <Website> <Body/> </Website> })
}

struct Card {
    title: String,
    description: String,
    image: String
}

struct Section {
    title: String,
    cards: Vec<Card>
}

#[component]
fn Body() -> impl IntoView {
    let sections = [
        Section {
            title: "My Passion".to_string(),
            cards: vec![
                Card {
                    title: "Responsible Computing".to_string(),
                    description: "We are in the midst of a data revolution and much like the industrial revolution, software engineers have a responsibility for how it affects our society.".to_string(),
                    image: "images/responsible-computing.jpg".to_string()
                }
        ]}
    ];
    view! {
        <div class="bg-[url('images/better-world.jpg')] h-screen bg-cover shadow-lg rounded-b-3xl">
            <h1 class="text-8xl text-blue-700 text-right font-bold py-20 pr-10"> "Jim Hill" </h1>
            <p class="text-6xl text-blue-400 text-right p-10"> "Be better, build better." </p>
        </div>
        <div class="container m-auto grid grid-cols-3 gap-4">
            {
                sections.into_iter()
                    .map(|section| view! { 
                        <div class="col-span-full">
                            <h2 class="text-4xl text-blue-700 py-5 text-center">{section.title}</h2>
                        </div>
                        {
                            section.cards.into_iter()
                            .map(|card| view! {
                                <div class="">
                                    <div class="h-36 bg-[url('images/responsible-computing.jpg')] bg-cover rounded-2xl">
                                    </div>
                                    <div>
                                        <h3 class="text-2xl text-blue-400 text-center p-2 pt-6">{card.title}</h3>
                                        <p class="text-center p-2">
                                            {card.description}
                                        </p>
                                    </div>
                                </div>
                            }).collect::<Vec<_>>()
                        }
                    })
                    .collect::<Vec<_>>()
            }
        </div>
        <div class="h-40 bg-blue-400 mt-10">
        </div>
    }
}