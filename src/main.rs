use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Epic {
    All,
    Iliad,
    Odyssey,
}

#[derive(Clone, Copy)]
struct Lesson {
    numeral: &'static str,
    title: &'static str,
    epic: Epic,
    epic_label: &'static str,
    figure: &'static str,
    quote: &'static str,
    insight: &'static str,
    practice: &'static str,
}

const LESSONS: [Lesson; 10] = [
    Lesson {
        numeral: "I",
        title: "Govern Your Wrath",
        epic: Epic::Iliad,
        epic_label: "Iliad",
        figure: "Achilles",
        quote: "Sing, goddess, the anger of Peleus’ son Achilles and its devastation.",
        insight: "Achilles’ menis nearly wrecks the Achaean campaign. Private grievance, nursed in the tent while the line breaks, is not honor—it is desertion of the common work. A leader who makes the army pay for a wounded ego will keep the prize and lose the war.",
        practice: "Separate the insult from the objective. Cool the blood before you set policy. If you must withdraw, name the principle—never make silence your revenge.",
    },
    Lesson {
        numeral: "II",
        title: "Authority Is Stewardship",
        epic: Epic::Iliad,
        epic_label: "Iliad",
        figure: "Agamemnon",
        quote: "I will be there in person at your tent to take Briseis in all her beauty, your own prize.",
        insight: "The scepter is not a license to seize what your people have earned. Agamemnon asserts rank by taking Achilles’ prize and shatters the coalition. Rank without restraint is theft with a title. Followers forgive hard orders; they do not forgive petty extraction.",
        practice: "Never take credit, spoils, or safety from your people to soothe your status. Publicly protect their due, especially when you have the power not to.",
    },
    Lesson {
        numeral: "III",
        title: "Stand the Unchosen Watch",
        epic: Epic::Iliad,
        epic_label: "Iliad",
        figure: "Hector",
        quote: "I have learned to be valiant always and to fight in the forefront of the Trojans.",
        insight: "Hector fights for the city, not the song. He knows the likely end—Andromache’s widowhood, the child dashed from the wall—and still meets Achilles at the Scaean gates. Leadership is showing up for the duty you did not romanticize.",
        practice: "Put your body where the risk is. Your people read courage from your calendar, not your speeches. Do the grim watch first.",
    },
    Lesson {
        numeral: "IV",
        title: "Keep Counsel Close",
        epic: Epic::Iliad,
        epic_label: "Iliad",
        figure: "Nestor",
        quote: "I am older than you; therefore I will speak out and tell you everything.",
        insight: "The old horseman’s gift is memory: what held at Pylos, what failed before Troy. Youth brings force; age brings pattern. Campaigns die when leaders confuse volume with wisdom and leave the veterans outside the tent.",
        practice: "Institutional memory is a weapon. Invite the people who have already paid for the lesson before you order the next charge.",
    },
    Lesson {
        numeral: "V",
        title: "Cunning Is a Form of Courage",
        epic: Epic::Odyssey,
        epic_label: "Odyssey",
        figure: "Odysseus",
        quote: "Tell me, Muse, of the man of many ways, who was driven far journeys.",
        insight: "Metis—cunning, timing, the fitting word—beats raw force. The horse at Troy, the name ‘Nobody,’ the beggar’s rags, the bow none else can string. Strength that cannot change shape is brittle. The polytropos leader asks what the hour requires, not what would make them look mighty.",
        practice: "Before you push, ask whether a sidestep wins cleaner. Reward ingenuity in your ranks the way you reward endurance.",
    },
    Lesson {
        numeral: "VI",
        title: "Lash Yourself to the Mast",
        epic: Epic::Odyssey,
        epic_label: "Odyssey",
        figure: "Odysseus & the Sirens",
        quote: "You must bind me with tight-fastening bonds, so that I stay unmoved where I am.",
        insight: "He wants the song and not the wreck. Willpower is a poor night watchman; systems hold when appetite wakes. Lotus, Circe’s hall, the cattle of Helios: the voyage home is a sequence of beautiful traps.",
        practice: "Name your sirens. Bind the process—dual control, cooling-off, a trusted crew with wax in their ears—so a moment of appetite cannot sink the ship.",
    },
    Lesson {
        numeral: "VII",
        title: "Grow Into the Chair",
        epic: Epic::Odyssey,
        epic_label: "Odyssey",
        figure: "Telemachus",
        quote: "You should no longer cling to your childhood; you are no longer of an age to do that.",
        insight: "Telemachus does not inherit Ithaca by sitting in the hall. He sails, asks Nestor and Menelaus, learns the world’s measure, then stands with his father. Leadership is apprenticed. Titles announced too early are costumes.",
        practice: "Seek mentors on purpose. Take the voyage—hard assignment, strange court, honest feedback—that makes you large enough for the seat you want.",
    },
    Lesson {
        numeral: "VIII",
        title: "Strategic Patience",
        epic: Epic::Odyssey,
        epic_label: "Odyssey",
        figure: "Penelope",
        quote: "By day she wove at the great web, but at night she would unravel it.",
        insight: "Without an army she keeps the household from being carved up. The shroud, the tests of the beggar, the bow contest: delay is a strategy when delay preserves the realm. Integrity under siege looks like quiet, repetitive work.",
        practice: "Do not confuse speed with seriousness. Hold the line until the right moment. Build tests that reveal character before you hand over the keys.",
    },
    Lesson {
        numeral: "IX",
        title: "Know When to Be Seen",
        epic: Epic::Odyssey,
        epic_label: "Odyssey",
        figure: "Odysseus in Ithaca",
        quote: "I will stay here by the swine, and you go first to the city.",
        insight: "He surveys his own hall as a beggar before he claims it. Premature revelation is suicide; so is endless hiding. Reconnaissance, allies (Eumaeus, Telemachus), then the moment the suitors cannot restring the bow. Power shown too soon is power wasted.",
        practice: "Gather truth quietly. Map the room. Reveal authority when the work is prepared—and you are.",
    },
    Lesson {
        numeral: "X",
        title: "See the Human in the Enemy",
        epic: Epic::Iliad,
        epic_label: "Iliad",
        figure: "Priam & Achilles",
        quote: "I have gone through what no other mortal on earth has gone through; I put my lips to the hands of the man who has killed my children.",
        insight: "The killer and the father weep in the same tent. Shared grief restores a measure of order: Hector’s body goes home. Victory that cannot make peace is only a pause between slaughters. After the fight, dignity is the first reconstruction project.",
        practice: "When the clash is over, restore face. Tomorrow’s alliance is built from today’s mercy. Speak of the dead as you would have your own spoken of.",
    },
];

#[component]
fn App() -> impl IntoView {
    let (filter, set_filter) = signal(Epic::All);
    let (open, set_open) = signal(None::<usize>);

    view! {
        <div class="min-h-screen overflow-x-hidden hero-wash font-sans text-marble">
            <Nav />
            <Hero />
            <Intro />
            <section id="lessons" class="relative mx-auto max-w-6xl px-4 pb-16 pt-4 sm:px-5 md:px-8 md:pb-24">
                <div class="mb-8 flex flex-col gap-5 md:mb-10 md:flex-row md:items-end md:justify-between">
                    <div class="min-w-0">
                        <p class="font-display text-xs tracking-[0.22em] text-bronze uppercase sm:text-sm sm:tracking-[0.35em]">
                            "The catalogue"
                        </p>
                        <h2 class="mt-2 font-display text-2xl leading-snug text-gold sm:text-4xl md:text-5xl">
                            "Ten commands from the poems"
                        </h2>
                    </div>
                    <div class="flex w-full min-w-0 flex-wrap gap-2 sm:w-auto">
                        <FilterChip
                            label="All"
                            active=Signal::derive(move || filter.get() == Epic::All)
                            on_click=move |_| set_filter.set(Epic::All)
                        />
                        <FilterChip
                            label="Iliad"
                            active=Signal::derive(move || filter.get() == Epic::Iliad)
                            on_click=move |_| set_filter.set(Epic::Iliad)
                        />
                        <FilterChip
                            label="Odyssey"
                            active=Signal::derive(move || filter.get() == Epic::Odyssey)
                            on_click=move |_| set_filter.set(Epic::Odyssey)
                        />
                    </div>
                </div>

                <div class="grid gap-5 md:grid-cols-2">
                    {LESSONS
                        .into_iter()
                        .enumerate()
                        .map(|(i, lesson)| {
                            view! {
                                <LessonCard
                                    lesson=lesson
                                    index=i
                                    filter=filter
                                    open=open
                                    set_open=set_open
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </section>
            <Coda />
            <Footer />
        </div>
    }
}

#[component]
fn Nav() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-30 border-b border-bronze/20 bg-navy/80 backdrop-blur-md">
            <div class="mx-auto flex max-w-6xl items-center justify-between gap-3 px-4 py-3 sm:px-5 sm:py-4 md:px-8">
                <a href="#top" class="flex min-w-0 items-baseline gap-2 no-underline sm:gap-3">
                    <span class="font-display text-lg tracking-[0.12em] text-gold sm:text-2xl sm:tracking-widest">
                        "ΟΜΗΡΟΣ"
                    </span>
                    <span class="hidden truncate font-serif text-lg italic text-sand sm:inline md:text-xl">
                        "the homeric command"
                    </span>
                </a>
                <nav class="flex shrink-0 gap-4 font-display text-[11px] tracking-[0.14em] text-sand uppercase sm:gap-6 sm:text-sm sm:tracking-[0.22em]">
                    <a class="transition hover:text-gold" href="#lessons">
                        "Lessons"
                    </a>
                    <a class="transition hover:text-gold" href="#coda">
                        "Nostos"
                    </a>
                </nav>
            </div>
            <div class="meander h-1.5 w-full"></div>
        </header>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section id="top" class="relative overflow-hidden px-4 pb-12 pt-10 sm:px-5 sm:pb-16 sm:pt-16 md:px-8 md:pb-20 md:pt-24">
            <div class="pointer-events-none absolute -right-24 top-10 hidden h-[420px] w-40 border-x border-bronze/15 md:block"></div>
            <div class="mx-auto grid max-w-6xl items-end gap-8 sm:gap-12 lg:grid-cols-12">
                <div class="min-w-0 lg:col-span-7">
                    <p class="font-display text-[11px] leading-relaxed tracking-[0.16em] text-bronze uppercase sm:text-base sm:tracking-[0.4em]">
                        "From the wine-dark sea to the Scaean gates"
                    </p>
                    <h1 class="mt-4 font-display text-[2.15rem] leading-[1.12] text-marble sm:mt-5 sm:text-6xl md:text-7xl md:leading-[1.08]">
                        "Leadership,"
                        <span class="block italic text-gold">"sung in hexameter"</span>
                    </h1>
                    <p class="mt-5 max-w-xl font-serif text-lg leading-relaxed text-sand sm:mt-6 sm:text-2xl md:text-3xl">
                        "Ten lessons from Homer’s "
                        <em>"Iliad"</em>
                        " and "
                        <em>"Odyssey"</em>
                        " for anyone who must hold a people together under pressure, pride, and the long way home."
                    </p>
                    <div class="mt-7 flex flex-col items-stretch gap-3 sm:mt-10 sm:flex-row sm:flex-wrap sm:items-center sm:gap-4">
                        <a
                            href="#lessons"
                            class="inline-flex items-center justify-center gap-2 bg-wine px-5 py-3.5 text-center font-display text-xs tracking-[0.18em] text-marble uppercase transition hover:bg-wine/80 sm:px-6 sm:text-sm sm:tracking-[0.25em]"
                        >
                            "Open the catalogue"
                        </a>
                        <p class="text-center font-serif text-base italic text-sand/80 sm:text-left sm:text-lg">
                            "Wrath. Cunning. Duty. Return."
                        </p>
                    </div>
                </div>
                <aside class="column-shadow relative min-w-0 bg-ink/70 p-5 sm:p-8 lg:col-span-5">
                    <div class="meander mb-5 h-2 w-full sm:mb-6"></div>
                    <p class="font-display text-xs tracking-[0.22em] text-bronze uppercase sm:text-sm sm:tracking-[0.3em]">
                        "Invocation"
                    </p>
                    <blockquote class="mt-4 font-serif text-xl leading-snug text-marble italic sm:text-3xl">
                        "A leader is not the loudest in the assembly, nor the first to claim the spoil—but the one who still knows the way to Ithaca when the crew has forgotten the stars."
                    </blockquote>
                    <p class="mt-5 font-display text-xs tracking-[0.18em] text-gold sm:mt-6 sm:text-sm sm:tracking-widest">
                        "— after Homer"
                    </p>
                    <dl class="mt-6 grid grid-cols-3 gap-2 border-t border-bronze/20 pt-5 text-center sm:mt-8 sm:gap-3 sm:pt-6">
                        <div class="min-w-0">
                            <dt class="font-display text-2xl text-gold sm:text-4xl">"10"</dt>
                            <dd class="mt-1 font-serif text-sm text-sand sm:text-lg">"lessons"</dd>
                        </div>
                        <div class="min-w-0">
                            <dt class="font-display text-2xl text-gold sm:text-4xl">"2"</dt>
                            <dd class="mt-1 font-serif text-sm text-sand sm:text-lg">"epics"</dd>
                        </div>
                        <div class="min-w-0">
                            <dt class="font-display text-2xl text-gold sm:text-4xl">"1"</dt>
                            <dd class="mt-1 font-serif text-sm text-sand sm:text-lg">"homecoming"</dd>
                        </div>
                    </dl>
                </aside>
            </div>
        </section>
    }
}

#[component]
fn Intro() -> impl IntoView {
    view! {
        <section class="mx-auto max-w-6xl px-4 pb-6 sm:px-5 md:px-8 md:pb-8">
            <div class="grid gap-8 border-y border-bronze/20 py-8 sm:gap-6 sm:py-12 md:grid-cols-3">
                <IntroPill
                    kicker="Iliad"
                    title="The cost of command"
                    body="Nine years at Troy teach what rage, rank, and duty do to a coalition. The poem is a staff college in grief."
                />
                <IntroPill
                    kicker="Odyssey"
                    title="The long return"
                    body="Winning is not arriving. Odysseus must outthink monsters, temptation, and his own legend to restore a household."
                />
                <IntroPill
                    kicker="Both"
                    title="People, not trophies"
                    body="Armor shines; cities burn. Homer’s leaders are measured by whether anyone still has a home when the singing stops."
                />
            </div>
        </section>
    }
}

#[component]
fn IntroPill(kicker: &'static str, title: &'static str, body: &'static str) -> impl IntoView {
    view! {
        <div class="min-w-0 px-0 sm:px-1">
            <p class="font-display text-xs tracking-[0.22em] text-bronze uppercase sm:text-sm sm:tracking-[0.3em]">{kicker}</p>
            <h3 class="mt-2 font-display text-xl text-gold sm:text-2xl">{title}</h3>
            <p class="mt-3 font-serif text-lg leading-relaxed text-sand sm:text-xl">{body}</p>
        </div>
    }
}

#[component]
fn FilterChip<F>(label: &'static str, active: Signal<bool>, on_click: F) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + Copy + 'static,
{
    view! {
        <button
            type="button"
            on:click=on_click
            class=move || {
                if active.get() {
                    "min-h-11 flex-1 border border-gold bg-gold/15 px-3 py-2.5 font-display text-xs tracking-[0.16em] text-gold uppercase sm:flex-none sm:px-5 sm:text-sm sm:tracking-[0.25em]"
                } else {
                    "min-h-11 flex-1 border border-bronze/30 px-3 py-2.5 font-display text-xs tracking-[0.16em] text-sand uppercase transition hover:border-gold/50 hover:text-gold sm:flex-none sm:px-5 sm:text-sm sm:tracking-[0.25em]"
                }
            }
        >
            {label}
        </button>
    }
}

#[component]
fn LessonCard(
    lesson: Lesson,
    index: usize,
    filter: ReadSignal<Epic>,
    open: ReadSignal<Option<usize>>,
    set_open: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let visible = move || {
        let f = filter.get();
        f == Epic::All || f == lesson.epic
    };
    let is_open = move || open.get() == Some(index);

    view! {
        <article
            class=move || {
                let hidden = if visible() { "" } else { "hidden" };
                let glow = if is_open() { " card-open" } else { "" };
                format!(
                    "{hidden}{glow} relative min-w-0 overflow-hidden border border-bronze/20 bg-ink/55 p-5 transition duration-300 hover:border-bronze/50 sm:p-7",
                )
            }
        >
            <span class="pointer-events-none absolute -right-1 -top-3 font-display text-6xl text-bronze/10 select-none sm:text-7xl md:text-8xl">
                {lesson.numeral}
            </span>
            <div class="relative flex items-start justify-between gap-3">
                <div class="min-w-0">
                    <p class="font-display text-[11px] leading-relaxed tracking-[0.16em] text-bronze uppercase sm:text-sm sm:tracking-[0.3em]">
                        {lesson.epic_label}
                        " · "
                        {lesson.figure}
                    </p>
                    <h3 class="mt-2 font-display text-2xl leading-snug text-marble sm:text-3xl">
                        {lesson.title}
                    </h3>
                </div>
                <span class="shrink-0 font-display text-base text-gold sm:text-lg">{lesson.numeral}</span>
            </div>
            <blockquote class="mt-4 border-l-2 border-wine pl-3 font-serif text-lg leading-snug text-sand italic sm:mt-5 sm:pl-4 sm:text-xl md:text-2xl">
                {lesson.quote}
            </blockquote>
            <p class="mt-4 font-serif text-lg leading-relaxed text-marble/90 sm:text-xl">{lesson.insight}</p>
            <Show when=is_open fallback=move || view! { <></> }>
                <div class="mt-5 border-t border-bronze/20 pt-5">
                    <p class="font-display text-xs tracking-[0.22em] text-gold uppercase sm:text-sm sm:tracking-[0.3em]">
                        "In the field"
                    </p>
                    <p class="mt-2 font-serif text-lg leading-relaxed text-foam sm:text-xl">
                        {lesson.practice}
                    </p>
                </div>
            </Show>
            <button
                type="button"
                class="mt-5 min-h-11 font-display text-xs tracking-[0.16em] text-bronze uppercase transition hover:text-gold sm:mt-6 sm:text-sm sm:tracking-[0.25em]"
                on:click=move |_| {
                    set_open
                        .update(|cur| {
                            *cur = if *cur == Some(index) { None } else { Some(index) };
                        });
                }
            >
                {move || {
                    if is_open() { "Close the practice —" } else { "How to lead from this —" }
                }}
            </button>
        </article>
    }
}

#[component]
fn Coda() -> impl IntoView {
    view! {
        <section id="coda" class="relative px-4 py-16 sm:px-5 sm:py-24 md:px-8">
            <div class="meander absolute top-0 left-0 h-2 w-full"></div>
            <div class="mx-auto max-w-3xl text-center">
                <p class="font-display text-xs tracking-[0.22em] text-bronze uppercase sm:text-base sm:tracking-[0.35em]">
                    "Nostos"
                </p>
                <h2 class="mt-3 font-display text-3xl leading-tight text-gold sm:mt-4 sm:text-4xl md:text-6xl">
                    "Bring them home"
                </h2>
                <p class="mt-5 font-serif text-lg leading-relaxed text-sand sm:mt-6 sm:text-2xl md:text-3xl">
                    "The Iliad ends in a funeral; the Odyssey ends in a marriage restored. Homer’s last word on command is not the body count. It is whether the people you led still have a hearth, a name, and a future. Win the day if you must. Measure yourself by the return."
                </p>
            </div>
        </section>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-bronze/20 px-4 py-6 sm:px-5 sm:py-8 md:px-8">
            <div class="mx-auto flex max-w-6xl flex-col items-center justify-between gap-2 text-center font-serif text-base text-sand/70 sm:gap-3 sm:text-lg md:flex-row md:text-left">
                <p class="max-w-prose">"The Homeric Command · leadership from the Iliad & Odyssey"</p>
                <p class="font-display text-[11px] tracking-[0.16em] uppercase sm:text-sm sm:tracking-[0.25em]">
                    "Leptos · WASM · Tailwind"
                </p>
            </div>
        </footer>
    }
}
