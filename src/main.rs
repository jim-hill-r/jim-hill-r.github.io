use include_dir::{include_dir, Dir};
use markdown::{ParseOptions, Constructs, Options, to_html_with_options};
use passionfruitdev::website::Website;
use leptos::*;

const CONTENT_DIRECTORY: Dir = include_dir!("./content");

fn markdown_options() -> Options {
    Options {
        parse: ParseOptions {
            constructs: Constructs {
                frontmatter: true,
                ..Constructs::default()
            },
            ..ParseOptions::default()
        },
        ..Options::default()
    }
}

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
    let md_options: Options = markdown_options();
    let sections = [
        Section {
            title: "My Passion".to_string(),
            cards: vec![
                Card {
                    title: "Responsible Computing".to_string(),
                    description: "We are in the midst of a data revolution and much like the industrial revolution, software engineers have a responsibility for how it affects our society.".to_string(),
                    image: "images/responsible-computing.jpg".to_string()
                },
                Card {
                    title: "Health and Wellness".to_string(),
                    description: "Health is a fundamental human right and health data should help individuals not increase the profits of large corporations.".to_string(),
                    image: "images/health-and-wellness.jpg".to_string()
                },
                Card {
                    title: "Environmental Impact".to_string(),
                    description: "Climate change is the existential crisis of our generation.".to_string(),
                    image: "images/environmental-impact.jpg".to_string()
                },
                Card {
                    title: "Education Reform".to_string(),
                    description: "Education is not personalized and technology can increase the availability and accessibility of learning to everyone.".to_string(),
                    image: "images/educational-reform.jpg".to_string()
                },
                Card {
                    title: "Financial Freedom".to_string(),
                    description: "Not everyone is an entrepreneur. Creators and builders should be unencumbered by money to enhance their impact on the world!".to_string(),
                    image: "images/financial-freedom.jpg".to_string()
                }
        ]},
        Section {
            title: "My Projects".to_string(),
            cards: vec![
                Card {
                    title: "Luggage".to_string(),
                    description: "Data does not belong to corporations. It belongs to us. This project hopes to provide an open-source interface for making your data safe, secure and portable.".to_string(),
                    image: "images/luggage.jpg".to_string()
                },
                Card {
                    title: "Gumby".to_string(),
                    description: "Data-driven health and fitness for everyone.".to_string(),
                    image: "images/gumby.svg".to_string()
                },
                Card {
                    title: "Passion Fruit".to_string(),
                    description: "When encumbered by too many dreams and goals, a platform of tools to accelerate many ideas at once is needed.".to_string(),
                    image: "images/passion-fruit.png".to_string()
                },
                Card {
                    title: "Blue Eel".to_string(),
                    description: "Literacy is the single most important outcome in a child's life. Let's not forget about those who struggle.".to_string(),
                    image: "images/blue-eel.jpg".to_string()
                },
                Card {
                    title: "Fire".to_string(),
                    description: "Financial planning tools for everyone so they can better understand the consequences of their decisions.".to_string(),
                    image: "images/fire.jpg".to_string()
                }
        ]},
        Section {
            title: "My Hobbies".to_string(),
            cards: vec![
                Card {
                    title: "Space Exploration".to_string(),
                    description: "I have always been interested in space and I believe that becoming multi-planetary is important for humanity. I just don't know yet how I can help or if it is our most pressing concern.".to_string(),
                    image: "images/space-exploration.jpg".to_string()
                },
                Card {
                    title: "Fitness".to_string(),
                    description: "A sound body is key to a sound mind. Through running, swimming, and climbing I find my peace in a healthy life.".to_string(),
                    image: "images/fitness.jpg".to_string()
                },
                Card {
                    title: "Natural Beauty".to_string(),
                    description: "I long for the beautiful vistas and natural places in this world. I want to preserve and sustain them for all generations!".to_string(),
                    image: "images/natural-beauty.jpg".to_string()
                },
                Card {
                    title: "Physics and Math".to_string(),
                    description: "Physics and math have always intrigued me. I would love to just spend all day working on fundamental unsolved problems in this space, but I am neither smart enough or focused enough. Instead, I am working on empowered those who are with financial freedom!".to_string(),
                    image: "images/physics-and-math.jpg".to_string()
                },
                Card {
                    title: "FIRST Robotics".to_string(),
                    description: "Mentoring the next generation into being well rounded, thoughtful engineers brings me endless joy. ".to_string(),
                    image: "images/robotics-and-automation.jpg".to_string()
                }
        ]}
    ];
    view! {
        <div class="bg-[url('images/better-world.jpg')] h-screen bg-cover shadow-lg rounded-b-3xl">
            <h1 class="text-8xl text-blue-700 text-right font-bold py-20 pr-10"> "Jim Hill" </h1>
            <p class="text-6xl text-blue-400 text-right p-10"> "Be better, build better." </p>
        </div>
        <div class="h-10"></div>
        <div class="container m-auto grid grid-cols-5 gap-4">
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
                                    <div class=move || {format!("h-36 bg-cover rounded-2xl bg-[url('{}')]",card.image)}>
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
        <footer class="h-40 bg-blue-400 mt-10 p-10">
            <div class="container m-auto grid grid-cols-1 gap-4">
                <div class="text-right text-white">
                    <p> Join me in building a better world. </p>
                </div>
                <div class="text-right">
                    <a href="https://www.github.com/jim-hill-r" class="text-bold text-white"> Github </a>
                </div>
            </div>
        </footer>
    }
}