use crate::web::views::todo::Todo;

// use include_dir::{include_dir, Dir};
// use markdown::{ParseOptions, Constructs, Options, to_html_with_options};
use leptos::*;
use leptos_router::*;
use leptos_meta::*;

// const CONTENT_DIRECTORY: Dir = include_dir!("./content");

// fn markdown_options() -> Options {
//     Options {
//         parse: ParseOptions {
//             constructs: Constructs {
//                 frontmatter: true,
//                 ..Constructs::default()
//             },
//             ..ParseOptions::default()
//         },
//         ..Options::default()
//     }
// }

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Jim Hill"/>
        <PrimaryRouter />
    }
}

#[component]
pub fn PrimaryRouter() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>   
                    <StaticRoute path="/" view=|| view! { <Home/> } static_params=move || Box::pin(async move {
                        StaticParamsMap::default()
                    }) />
                    <StaticRoute path="/todo" view=|| view! { <Todo/> } static_params=move || Box::pin(async move {
                        StaticParamsMap::default()
                    }) />
                    <StaticRoute path="/bookmarks" view=|| view! { <Bookmark/> } static_params=move || Box::pin(async move {
                        StaticParamsMap::default()
                    }) />
                    <StaticRoute path="/resume" view=|| view! { <Resume/> } static_params=move || Box::pin(async move {
                        StaticParamsMap::default()
                    }) />
                </Routes>
            </main>
        </Router>
    }
}

struct Card {
    title: String,
    description: String,
    image: String,
    image_alt: String,
    link: Option<String>
}

struct Section {
    title: String,
    cards: Vec<Card>
}

#[component]
fn Home() -> impl IntoView {
    // let md_options: Options = markdown_options();
    let sections = [
        Section {
            title: "My Passion".to_string(),
            cards: vec![
                Card {
                    title: "Responsible Computing".to_string(),
                    description: "We are in the midst of a data revolution and much like the industrial revolution, software engineers have a responsibility for how it affects our society.".to_string(),
                    image: "images/responsible-computing.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Health and Wellness".to_string(),
                    description: "Health is a fundamental human right and health data should help individuals not increase the profits of large corporations.".to_string(),
                    image: "images/health-and-wellness.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Climate Impact".to_string(),
                    description: "Climate change is the existential crisis of our generation.".to_string(),
                    image: "images/environmental-impact.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Education Reform".to_string(),
                    description: "Education is not personalized and technology can increase the availability and accessibility of learning to everyone.".to_string(),
                    image: "images/educational-reform.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Financial Freedom".to_string(),
                    description: "Not everyone is an entrepreneur. Creators and builders should be unencumbered by money to enhance their impact on the world!".to_string(),
                    image: "images/financial-freedom.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                }
        ]},
        Section {
            title: "My Projects".to_string(),
            cards: vec![
                Card {
                    title: "Luggage".to_string(),
                    description: "Data does not belong to corporations. It belongs to us. This project hopes to provide an open-source interface for making your data safe, secure and portable.".to_string(),
                    image: "images/luggage.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Gumby".to_string(),
                    description: "Data-driven health and fitness for everyone.".to_string(),
                    image: "images/gumby.svg".to_string(),
                    image_alt: "".to_string(),
                    link: Some("https://www.6umby.com".to_string())
                },
                Card {
                    title: "Passion Fruit".to_string(),
                    description: "When encumbered by too many dreams and goals, a platform of tools to accelerate many ideas at once is needed.".to_string(),
                    image: "images/passion-fruit.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Blue Eel".to_string(),
                    description: "Literacy is the single most important outcome in a child's life. Let's not forget about those who struggle.".to_string(),
                    image: "images/blue-eel.webp".to_string(),
                    image_alt: "".to_string(),
                    link: Some("https://blue.eel.education".to_string())
                },
                Card {
                    title: "Fire".to_string(),
                    description: "Financial planning tools for everyone so they can better understand the consequences of their decisions.".to_string(),
                    image: "images/fire.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                }
        ]},
        Section {
            title: "My Hobbies".to_string(),
            cards: vec![
                Card {
                    title: "Space Exploration".to_string(),
                    description: "I have always been interested in space and I believe that becoming multi-planetary is important for humanity. I just don't know yet how I can help or if it is our most pressing concern.".to_string(),
                    image: "images/space-exploration.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Fitness".to_string(),
                    description: "A sound body is key to a sound mind. Through running, swimming, and climbing I find my peace in a healthy life.".to_string(),
                    image: "images/fitness.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Natural Beauty".to_string(),
                    description: "I long for the beautiful vistas and natural places in this world. I want to preserve and sustain them for all generations!".to_string(),
                    image: "images/natural-beauty.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "Physics and Math".to_string(),
                    description: "Physics and math have always intrigued me. I would love to just spend all day working on fundamental unsolved problems in this space, but I am neither smart enough or focused enough. Instead, I am working on empowered those who are with financial freedom!".to_string(),
                    image: "images/physics-and-math.webp".to_string(),
                    image_alt: "".to_string(),
                    link: None
                },
                Card {
                    title: "FIRST Robotics".to_string(),
                    description: "Mentoring the next generation into being well rounded, thoughtful engineers brings me endless joy. ".to_string(),
                    image: "images/robotics-and-automation.webp".to_string(),
                    image_alt: "".to_string(),
                    link: Some("http://wildraccoons8891.org/".to_string())
                }
        ]}
    ];
    view! {
        <div class="bg-[url('images/better-world.webp')] h-screen bg-cover bg-center shadow-lg rounded-b-3xl">
            <h1 class="text-8xl text-blue-700 text-right font-bold py-20 pr-10"> "Jim Hill" </h1>
            <p class="text-6xl text-blue-400 text-right p-10"> "Be better, build better." </p>
        </div>
        <div class="h-10"></div>
        <div class="container m-auto grid grid-cols-1 sm:grid-cols-3 md:grid-cols-5 gap-4 p-4">
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
                                    <a target="_blank" href={card.link}>
                                    <img class="m-auto h-36 w-36 rounded-2xl" src={card.image} alt={card.image_alt}/>
                                    
                                    <div>
                                        <h3 class="text-2xl text-blue-400 text-center p-2 pt-6">{card.title}</h3>
                                        <p class="text-center p-2">
                                            {card.description}
                                        </p>
                                    </div>
                                    </a>
                                </div>
                            }).collect::<Vec<_>>()
                        }
                    })
                    .collect::<Vec<_>>()
            }
        </div>
        <footer class="min-h-40 bg-blue-400 mt-10 p-10">
            <div class="container m-auto grid grid-cols-1 gap-4">
                <div class="text-right text-white">
                    <p> Join me in building a better world. </p>
                </div>
                <div class="text-right">
                    <a href="https://www.github.com/jim-hill-r" class="text-bold text-white"> Github </a>
                </div>
                <div class="text-right">
                    <a href="https://www.linkedin.com/in/jimhillr/" class="text-bold text-white"> LinkedIn </a>
                </div>
                <div class="text-right">
                    <a href="/resume" class="text-bold text-white"> Resume </a>
                </div>
            </div>
        </footer>
    }
}

enum Focus {
    ENGINEER,
    EDUCATOR,
    ENTREPRENEUR
}

struct Experience {
    focus: Focus,
    title: String,
    company: String,
    timeframe: String,
    descriptions: Vec<String>
}

#[component]
fn Resume() -> impl IntoView {
    let engineerExperiences = [
        Experience {
            focus: Focus::ENGINEER,
            title: "Infrastructure Team Lead".to_string(),
            company: "JuliaHub".to_string(),
            timeframe: "2023-Present".to_string(),
            descriptions: vec![
                "Assumed role of people manager for the core platform infrastructure team while continuing cloud engineer role".to_string(),
                "Maintained backlog and groomed support tickets and issues for team".to_string(),
                "Point of contact for sales engineering, developers, and SOC2 compliance ensuring reliable architecture and coordinated upgrades".to_string()
            ]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Cloud Engineer".to_string(),
            company: "JuliaHub".to_string(),
            timeframe: "2022-Present".to_string(),
            descriptions: vec![
                "Responsible for site reliability for public, multi-tenant, single-tenant, and on-premise platform installations via Grafana alerting, observability and on-call rotations".to_string(),
                "Implemented infrastructure improvements including CIS best practices; CICD for terraform, ansible, kustomize and custom Julia scripts; custom fluentbit logging solution; golden AMIs with Packer; and Stripe billing integrations".to_string(),
                "Owned AWS security (SSO/IAM) access while improving developer experience with tools such as Tilt and Github Actions.".to_string()
            ]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Principal Software Engineer".to_string(),
            company: "NavAbility".to_string(),
            timeframe: "2020-2022".to_string(),
            descriptions: vec![
                "Designed, developed and operationalized progressive web app on AWS".to_string(),
                "Architected cloud system in collaboration with core Caesar.jl team including CICD".to_string(),
                "Implemented and launched over 10 microservices on AWS ECS entirely with CloudFormation".to_string(),
                "Produced marketing website using Elementor on Wordpress".to_string()
            ]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Enterprise Architect".to_string(),
            company: "Intelligent Medical Objects".to_string(),
            timeframe: "2020-2021".to_string(),
            descriptions: vec![
                "Architected the evolutionary architecture for the future of core search and normalize".to_string(),
                "Evangelized a shift to cross-team microservices architectures".to_string(),
                "Mentored multiple teams in api design while shifting away from 3-tiered architectures".to_string()
            ]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Staff Software Engineer".to_string(),
            company: "Intelligent Medical Objects".to_string(),
            timeframe: "2019-2020".to_string(),
            descriptions: vec![
                "Provisioned and operated high availability infrastructure for core search product on AWS".to_string(),
                "Responsible for technology upgrade from .NET to ElasticSearch and AWS Lambda".to_string(),
                "Supported and mentored engineers to ensure rapid delivery of quality software leveraging agile scrum".to_string()
            ]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Senior Software Engineer".to_string(),
            company: "GE Digital".to_string(),
            timeframe: "2015-2019".to_string(),
            descriptions: vec![
                "Develop dotnet and web apps improving efficiency and scalability of the GE Digital managed services
organization".to_string(),
                "Primary project manager, architect, designer and developer for products built on Predix cloud and WinForms".to_string(),
                "Responsible for delivering agile sprint stories involving C#, NodeJS, Angular2, Spring Boot, Postgres, OAuth2, and
                Cloud Foundry".to_string()
            ]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Customer Reliability Engineer".to_string(),
            company: "GE Digital".to_string(),
            timeframe: "2013-2015".to_string(),
            descriptions: vec![]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Applications Engineer".to_string(),
            company: "Renishaw".to_string(),
            timeframe: "2010-2013".to_string(),
            descriptions: vec![]
        },
        Experience {
            focus: Focus::ENGINEER,
            title: "Design Engineer".to_string(),
            company: "Caterpillar".to_string(),
            timeframe: "2006-2009".to_string(),
            descriptions: vec![]
        }
    ];
    let educatorExperiences = [
        Experience {
            focus: Focus::EDUCATOR,
            title: "Volunteer Mentor & Team Leader".to_string(),
            company: "FIRST Robotics".to_string(),
            timeframe: "2008-2009, 2022-Present".to_string(),
            descriptions: vec![]
        },
        Experience {
            focus: Focus::EDUCATOR,
            title: "Adjunct Professor".to_string(),
            company: "Aurora University".to_string(),
            timeframe: "2017".to_string(),
            descriptions: vec![]
        },
        Experience {
            focus: Focus::EDUCATOR,
            title: "Graduate Assistant".to_string(),
            company: "Northern Illinois University".to_string(),
            timeframe: "2009-2010".to_string(),
            descriptions: vec![]
        },
        Experience {
            focus: Focus::EDUCATOR,
            title: "Peer Tutor".to_string(),
            company: "Northern Illinois University".to_string(),
            timeframe: "2003-2005".to_string(),
            descriptions: vec![]
        }
    ];
    let entrepreneurExperiences = [
        Experience {
            focus: Focus::ENTREPRENEUR,
            title: "Co-founder".to_string(),
            company: "NavAbility".to_string(),
            timeframe: "2020-2022".to_string(),
            descriptions: vec![
                "Managed operations, cost, project planning and uptime of cloud product".to_string(),
                "Collaborated on submissions for grants and venture capital funding".to_string()
            ]
        },    
        Experience {
            focus: Focus::ENTREPRENEUR,
            title: "Product Manager".to_string(),
            company: "GE Digital".to_string(),
            timeframe: "2017-2019".to_string(),
            descriptions: vec![
                "Developed the global strategy for providing expertise-based, continuous consulting services for GE customers".to_string(),
                "Provided support for marketing, sales, and new product introductions".to_string(),
                "Managed two scrums teams developing cloud software for industrial customers".to_string()
            ]
        },
        Experience {
            focus: Focus::ENTREPRENEUR,
            title: "Founder".to_string(),
            company: "Spark Hill, LLC".to_string(),
            timeframe: "2014-2015".to_string(),
            descriptions: vec![]
        },
        Experience {
            focus: Focus::ENTREPRENEUR,
            title: "Volunteer Event Chairman".to_string(),
            company: "American Cancer Society".to_string(),
            timeframe: "2008-2010".to_string(),
            descriptions: vec![]
        }
    ];
    let skills = skills();
    view! {
        <div class="bg-amber-50 min-h-screen m-0 p-0 text-slate-600">
            <div class="print:hidden pt-12"></div>
            <div class="print:border-0 print:shadow-none container min-h-screen m-auto grid grid-cols-12 border-2 shadow-lg">
                <div class="bg-blue-600 col-span-3 p-2 text-white">
                    <div class="container grid grid-cols-4">
                        <div class="col-span-4 text-center p-5">
                            <img class="rounded-full bg-center border-4 border-white" src="images/headshot_2023.webp"/>
                        </div>
                        <div class="print:text-[12px] col-span-4 text-center font-medium">
                            "1.630.212.9727" <br/>
                            "jim.hill.r@gmail.com" <br/>
                            "Github: jim-hill-r" <br/>
                            "LinkedIn: jimhillr" <br/>
                            <br/>
                            "Be better, build better." <br/>
                        </div>
                        <div class="print:text-[10px] mt-2 col-span-4 text-left text-sm">
                            <h2 class="text-2xl pl-2 font-semibold"> "Education" </h2>
                            <hr/>
                        </div>
                        <div class="print:text-[10px] print:leading-tight p-2 col-span-4 text-sm">
                            <p class="float-left"> "Masters of Science" </p>
                            <p class="float-right"> "3.9/4.0 GPA" </p>
                            <p class="float-left"> "Computer Science" </p>
                            <p class="float-right"> "2016" </p>
                            <p class="float-left"> "Georgia Institute of Technology" </p>
                            <p class="float-left"> "Machine Learning Specialization" </p>
                        </div>
                        <div class="print:text-[10px] print:leading-tight p-2 col-span-4 text-sm">
                            <p class="float-left"> "Masters of Science" </p>
                            <p class="float-right"> "3.9/4.0 GPA" </p>
                            <p class="float-left"> "Mechanical Engineering" </p>
                            <p class="float-right"> "2014" </p>
                            <p class="float-left"> "Northern Illinois University" </p>
                        </div>
                        <div class="print:text-[10px] print:leading-tight p-2 col-span-4 text-sm">
                            <p class="float-left"> "Bachelors of Science" </p>
                            <p class="float-right"> "3.9/4.0 GPA" </p>
                            <p class="float-left"> "Mechanical Engineering" </p>
                            <p class="float-right"> "2006" </p>
                            <p class="float-left"> "Northern Illinois University" </p>
                        </div>
                        <div class="print:text-[10px] mt-2 col-span-4 text-left text-sm">
                            <h2 class="text-2xl pl-2 font-semibold"> "Skills" </h2>
                            <hr/>
                        </div>
                        <div class="col-span-4">
                            {skills.into_iter()
                                .map(|group| view! {
                                    <div class="print:leading-tight p-2 pb-0 flex flex-wrap">
                                    {
                                        group.into_iter()
                                            .map(|skill| view! {
                                                {if skill.emphasis {
                                                    view! {<div class="print:leading-tight print:text-[10px] text-sm pr-2 font-semibold"> {skill.keyword} </div>}
                                                } else {
                                                    view! {<div class="print:leading-tight print:text-[10px] text-sm pr-2"> {skill.keyword} </div>}
                                                }}
                                                
                                            }).collect::<Vec<_>>()
                                    }
                                    </div>
                                }).collect::<Vec<_>>()}
                        </div>
                        
                    </div>
                </div>
                <div class="bg-white p-2 col-span-9">
                    <div class="container grid grid-cols-4 items-end p-2">
                        <div class="print:leading-tight print:text-[32px] col-span-1 text-left font-semibold text-5xl text-blue-600">
                            "Jim Hill"
                        </div>
                        <div class="print:leading-tight print:text-[18px] col-span-1 text-left font-medium text-2xl text-blue-600">
                            "Engineer"
                        </div>
                        <div class="print:leading-tight print:text-[18px] col-span-1 text-left font-medium text-2xl text-blue-600">
                            "Educator"
                        </div>
                        <div class="print:leading-tight print:text-[18px] col-span-1 text-left font-medium text-2xl text-blue-600">
                            "Entrepreneur"
                        </div>
                        <div class="print:leading-tight print:text-[10px] mt-2 col-span-4 text-left text-sm">
                            <h2 class="print:leading-tight text-2xl font-semibold text-slate-600"> "Summary" </h2>
                            <hr/>
                            <p class="print:leading-tight">
                            "I have more than 15 years of experience in software, mechanical, and manufacturing engineering in industries including software tech, healthcare, education, automotive, mining, aviation and power generation. "
                            "During my career, I have held roles as a design engineer, applications engineer, software developer, and software engineer at Caterpillar, Renishaw, GE Digital, and IMO respectively. Recently I have been a principal software engineer and infrastructure team lead at two startups, NavAbility and JuliaHub. Combining this experience with a pair of Master's Degrees in Mechanical and Software Engineering allows me to provide a comprehensive view of how digital technology can help solve the real challenges faced by a modern internet."
                            </p>
                        </div>
                        <div class="print:leading-tight print:text-[10px] print:leading-tight mt-2 col-span-4 text-left text-sm">
                            <h2 class="print:leading-tight text-2xl font-semibold text-slate-600"> "Experience as an Engineer" </h2>
                            <hr/>
                            {engineerExperiences.into_iter()
                                .map(|experience| view! {
                                    <div class="print:leading-tight mt-0 col-span-4">
                                        <div class="print:leading-tight flow-root">  
                                            <h3 class="print:leading-tight print:text-[12px] text-base font-medium float-left">{experience.company}" | "{experience.title} </h3>
                                            <p class="print:leading-tight text-right pr-2"> {experience.timeframe} </p>
                                        </div>
                                        <ul>
                                        {experience.descriptions.into_iter()
                                            .map(|description| view! {
                                                <li class="print:leading-tight print:text-[10px] text-sm pl-3"> {description} </li>
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>
                                }).collect::<Vec<_>>()}
                        </div>
                        <div class="print:leading-tight print:text-[10px] mt-2 col-span-4 text-left text-sm">
                            <h2 class="print:leading-tight text-2xl font-semibold text-slate-600"> "Experience as an Educator" </h2>
                            <hr/>
                            {educatorExperiences.into_iter()
                                .map(|experience| view! {
                                    <div class="print:leading-tight col-span-4">
                                        <div class="print:leading-tight flow-root">  
                                            <h3 class="print:leading-tight print:text-[12px] text-base font-medium float-left">{experience.company}" | "{experience.title} </h3>
                                            <p class="print:leading-tight text-right pr-2"> {experience.timeframe} </p>
                                        </div>
                                        <ul>
                                        {
                                            experience.descriptions.into_iter()
                                                .map(|description| view! {
                                                    <li class="print:leading-tight print:text-[10px] text-sm pl-3"> {description} </li>
                                                }).collect::<Vec<_>>()
                                        }
                                        </ul>
                                    </div>
                                }).collect::<Vec<_>>()}
                        </div>
                        <div class="print:leading-tight print:text-[10px] mt-2 col-span-4 text-left text-sm">
                            <h2 class="print:leading-tight text-2xl font-semibold text-slate-600"> "Experience as an Entrepreneur" </h2>
                            <hr/>
                            {entrepreneurExperiences.into_iter()
                                .map(|experience| view! {
                                    <div class="print:leading-tight mt-0 col-span-4">
                                        <div class="print:leading-tight flow-root">  
                                            <h3 class="print:leading-tight print:text-[12px] text-base font-medium float-left">{experience.company}" | "{experience.title} </h3>
                                            <p class="print:leading-tight text-right pr-2"> {experience.timeframe} </p>
                                        </div>
                                        <ul>
                                        {
                                            experience.descriptions.into_iter()
                                                .map(|description| view! {
                                                    <li class="print:leading-tight print:text-[10px] text-sm pl-3"> {description} </li>
                                                }).collect::<Vec<_>>()
                                        }
                                        </ul>
                                    </div>
                                }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

struct Skill {
    keyword: String,
    emphasis: bool
}

fn skills() -> Vec<Vec<Skill>> {
vec![
  vec![
    Skill{keyword:"HTML".to_string(), emphasis:true},
    Skill{keyword:"CSS".to_string(), emphasis:true},
    Skill{keyword:"Javascript".to_string(), emphasis:false},
    Skill{keyword:"Typescript".to_string(), emphasis:true},
    Skill{keyword:"Node".to_string(), emphasis:true},
    Skill{keyword:"OAuth2".to_string(), emphasis:false},
    Skill{keyword:"Angular".to_string(), emphasis:false},
    Skill{keyword:"VueJS".to_string(), emphasis:false},
    Skill{keyword:"React".to_string(), emphasis:true},
    Skill{keyword:"GraphQL".to_string(), emphasis:true},
    Skill{keyword:"Gatsby".to_string(), emphasis:false},
    Skill{keyword:"Astro".to_string(), emphasis:true},
    Skill{keyword:"Leptos".to_string(), emphasis:true},
  ],
  vec![
    Skill{keyword:"AWS".to_string(), emphasis:true},
    Skill{keyword:"Linux".to_string(), emphasis:false},
    Skill{keyword:"Docker".to_string(), emphasis:false},
    Skill{keyword:"Kubernetes".to_string(), emphasis:true},
    Skill{keyword:"Kustomize".to_string(), emphasis:true},
    Skill{keyword:"Bash".to_string(), emphasis:false},
    Skill{keyword:"DevOps".to_string(), emphasis:false},
    Skill{keyword:"Github Actions".to_string(), emphasis:false},
    Skill{keyword:"Terraform".to_string(), emphasis:true},
    Skill{keyword:"Packer".to_string(), emphasis:true},
    Skill{keyword:"Fluentbit".to_string(), emphasis:false},
    Skill{keyword:"Ansible".to_string(), emphasis:true},
    Skill{keyword:"Infra as Code".to_string(), emphasis:false},
    Skill{keyword:"SOC2".to_string(), emphasis:false},
    Skill{keyword:"Cloud Foundry".to_string(), emphasis:false}
  ],
  vec![
    Skill{keyword:"SQL".to_string(), emphasis:false},
    Skill{keyword:"Neo4j".to_string(), emphasis:false},
    Skill{keyword:"Redis".to_string(), emphasis:false},
    Skill{keyword:"DynamoDB".to_string(), emphasis:false},
    Skill{keyword:"ElasticSearch".to_string(), emphasis:false},
    Skill{keyword:"Postgres".to_string(), emphasis:true},
    Skill{keyword:"Redpanda".to_string(), emphasis:false},
    Skill{keyword:"EFS".to_string(), emphasis:false},
    Skill{keyword:"S3".to_string(), emphasis:true}
  ],
  vec![
    Skill{keyword:"Julia".to_string(), emphasis:true},
    Skill{keyword:"Python".to_string(), emphasis:false},
    Skill{keyword:"Java".to_string(), emphasis:false},
    Skill{keyword:"R".to_string(), emphasis:false},
    Skill{keyword:"Matlab".to_string(), emphasis:false},
    Skill{keyword:"Machine Learning".to_string(), emphasis:false},
    Skill{keyword:"Spark".to_string(), emphasis:false},
    Skill{keyword:"Rust".to_string(),emphasis:true}
  ],
  vec![
    Skill{keyword:"Microsoft".to_string(), emphasis:false},
    Skill{keyword:"C#".to_string(), emphasis:false},
    Skill{keyword:".NET Core".to_string(), emphasis:false},
    Skill{keyword:"Azure".to_string(), emphasis:false},
    Skill{keyword:"Windows".to_string(), emphasis:false},
    Skill{keyword:"Powershell".to_string(), emphasis:false}
  ],
  vec![
    Skill{keyword:"Dynamics".to_string(), emphasis:false},
    Skill{keyword:"Vibrations".to_string(), emphasis:false},
    Skill{keyword:"Controls".to_string(), emphasis:false},
    Skill{keyword:"FMEA".to_string(), emphasis:false},
    Skill{keyword:"Powertrain".to_string(), emphasis:false},
    Skill{keyword:"GD&T".to_string(), emphasis:false}
  ],
  vec![
    Skill{keyword:"Power Generation".to_string(), emphasis:false},
    Skill{keyword:"Turbines".to_string(), emphasis:false},
    Skill{keyword:"Reciprocating Engines".to_string(), emphasis:false},
    Skill{keyword:"HRSG".to_string(), emphasis:false}
  ],
  vec![
    Skill{keyword:"UChicago New Venture Challenge".to_string(), emphasis:false},
    Skill{keyword:"Leadership Training".to_string(), emphasis:false},
    Skill{keyword:"Certified Tutor".to_string(), emphasis:false},
    Skill{keyword:"Eagle Scout".to_string(), emphasis:true}
  ]
]
}

#[component]
fn Bookmark() -> impl IntoView {
    view! {
        <div class="bg-amber-50 min-h-screen m-0 p-0 text-slate-600">
            "My Stuff"
            <ul>
                <li> <a href="https://dataluggage.com" target="_blank"> "Future Luggage Home" </a> </li>
                <li> <a href="https://www.6umby.com" target="_blank"> "Gumby Home" </a> </li>
                <li> <a href="https://passionfruit.dev" target="_blank"> "Future Passion Fruit Home" </a> </li>
                <li> <a href="https://blue.eel.education" target="_blank"> "Blue Eel Legacy Home" </a> </li>
                <li> <a href="https://preview.eel.education" target="_blank"> "Blue Eel Next Generation Home" </a> </li>

                <li> <a href="https://bananapeelsocks.com" target="_blank"> "Our family domain" </a> </li>
                <li> <a href="https://oobicoin.com" target="_blank"> "Future Oobicoin Home" </a> </li>
            </ul>
            <br/>
            "My Customers"
            <ul>
                <li> <a href="https://wildraccoons8891.org" target="_blank"> "Wild Raccoons Home" </a> </li>
                <li> <a href="https://sparkhill.xyz" target="_blank"> "Spark Hill Home" </a> </li>
            </ul>
            <br/>
            "Social media and personal brand"
            <ul>
            <li> <a href="https://www.jimhillr.com" target="_blank"> "My Landing Page" </a> </li>
                <li> <a href="https://github.com/jim-hill-r" target="_blank"> "Github Profile" </a> </li>
                <li> <a href="https://www.linkedin.com/in/jimhillr/" target="_blank"> "LinkedIn Profile" </a> </li>
            </ul>
            "Architecture links"
            <ul>
                <li> <a href="https://github.com/orgs/surrealdb/discussions/1275" target="_blank"> "Use surreal in the client" </a> </li>
            </ul>
        </div>
    }
}
