use crate::components::forms::input::{InputField, InputFieldType};
use chrono::{Datelike, Local, NaiveDate, Weekday};
use yew::{prelude::*, virtual_dom::VNode};
use yew_icons::{Icon, IconId};

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct DatePickerProps {
    pub label: String,
    pub name: String,
    #[prop_or(false)]
    pub required: bool,
    #[prop_or(Local::now().date_naive())]
    pub initial_value: NaiveDate,
    #[prop_or(Callback::noop())]
    pub onchange: Callback<NaiveDate>,
}

#[function_component]
pub fn DatePicker(props: &DatePickerProps) -> Html {
    let DatePickerProps {
        label,
        name,
        required,
        initial_value,
        onchange,
    } = props;

    let show_calendar = use_state_eq(|| false);
    let selected_date =
        use_state_eq(|| initial_value.clone());

    // Toggle calendar visibility
    let toggle_calendar = {
        let show_calendar = show_calendar.clone();
        Callback::from(move |_| {
            show_calendar.set(!*show_calendar);
        })
    };

    // Select a date
    let select_date = {
        let onchange = onchange.clone();
        let show_calendar = show_calendar.clone();
        let selected_date = selected_date.clone();
        Callback::from(move |date: NaiveDate| {
            selected_date.set(date.clone());
            onchange.emit(date.clone());
            show_calendar.set(false);
        })
    };

    html! {
        <div class="mb-2">
            <label for={name.clone()} class="block text-gray-700 text-sm font-bold mb-2">
                {label}
                { if *required { html!{ <span class="text-red-500">{ "*" }</span> } } else { html!{} } }
            </label>
            <div class="relative">
                <InputField readonly=true onclick={toggle_calendar.clone()} initial_value={selected_date.format("%b %0e %Y").to_string()} name={name.clone()} field_type={InputFieldType::Text} />
                <div class="absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer" onclick={toggle_calendar.clone()}>
                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapCalendarDate}/>
                </div>
                {
                    if *show_calendar {
                        html! {
                            <div class="absolute bg-slate-50 border mt-1 rounded shadow-lg z-10 max-h-[400px] overflow-auto"> // Max height set here
                                <Calendar {select_date} />
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct CalendarProps {
    select_date: Callback<NaiveDate>,
}

#[function_component]
fn Calendar(props: &CalendarProps) -> Html {
    let today = chrono::Local::now().naive_local();
    let default_month = today.month();
    let default_year = today.year();
    let current_month = use_state_eq(|| default_month);
    let current_year = use_state_eq(|| default_year);
    let current_month_left = current_month.clone();
    let current_month_right = current_month.clone();
    let current_year_left = current_year.clone();
    let current_year_right = current_year.clone();
    let viewing_years = use_state_eq(|| false);

    let years_per_page = 16; // Number of years to display per page
    let year_page = use_state_eq(|| 0);

    let toggle_viewing_years = {
        let viewing_years = viewing_years.clone();
        Callback::from(move |_| viewing_years.set(!*viewing_years))
    };

    let change_year = {
        let viewing_years = viewing_years.clone();
        let current_year = current_year.clone();
        Callback::from(move |year: i32| {
            current_year.set(year);
            viewing_years.set(false);
        })
    };

    let render_years = || {
        let start_year = (*current_year - 60).max(1); // or any other way to determine start year
        let end_year = *current_year + 12;
        let total_years: Vec<i32> = (start_year..end_year).collect();
        let pages = total_years.chunks(years_per_page).collect::<Vec<&[i32]>>();

        if pages.is_empty() {
            return vec![html! {}];
        }

        let current_page = *year_page % pages.len();

        let mut years = vec![];
        for &year in pages[current_page] {
            let change_year = change_year.clone();
            years.push(html! {
                <button
                    class="h-8 px-2 border rounded m-1 hover:bg-blue-200"
                    onclick={Callback::from(move |_| change_year.emit(year))}
                >
                    {year}
                </button>
            });
        }
        years
    };

    let next_year_page = {
        let year_page = year_page.clone();
        Callback::from(move |_| year_page.set(*year_page + 1))
    };

    let prev_year_page = {
        let year_page = year_page.clone();
        Callback::from(move |_| {
            if *year_page > 0 {
                year_page.set(*year_page - 1)
            }
        })
    };

    let days_in_month = {
        let current_year = current_year.clone();
        let current_month = current_month.clone();
        move || {
            log::info!(
                "current month: {:?}, current year: {:?}",
                current_month.clone(),
                current_year.clone()
            );
            match NaiveDate::from_ymd_opt(*current_year, *current_month + 1, 1) {
                Some(date) => date
                    .signed_duration_since(
                        NaiveDate::from_ymd_opt(*current_year, *current_month, 1).unwrap(),
                    )
                    .num_days() as u32,
                None => 31,
            }
        }
    };

    let select_date = props.select_date.clone();
    let render_days = || {
        let mut days = vec![];
        let mut calendar_adjustment = 0;

        for day in 1..=days_in_month() {
            let date = NaiveDate::from_ymd_opt(*current_year, *current_month, day);
            if day == 1 {
                let weekday = date.unwrap().weekday();
                log::info!("Weekday: {:?}", weekday);
                calendar_adjustment = match weekday {
                    Weekday::Sun => 0,
                    Weekday::Mon => 1,
                    Weekday::Tue => 2,
                    Weekday::Wed => 3,
                    Weekday::Thu => 4,
                    Weekday::Fri => 5,
                    Weekday::Sat => 6,
                };
            }

            let select_date = select_date.clone();

            days.push(html! {
                <button
                    class="h-8 px-2 border rounded m-1 hover:bg-blue-200"
                    onclick={Callback::from(move |_| select_date.emit(date.unwrap()))}
                >
                    {day}
                </button>
            });
        }

        let blank_days_count = calendar_adjustment;
        let blank_days: Vec<VNode> = (0..blank_days_count)
            .map(|_| {
                html! {
                    <button
                        class="h-8 px-2 border-none rounded m-1"
                    >
                        {""}
                    </button>
                }
            })
            .collect();

        days.splice(0..0, blank_days); // Insert blank days at the beginning
        days
    };

    html! {
        <div class="p-4">
            {
                if *viewing_years {
                    html! {
                        <>
                            <div class="flex justify-between items-center mb-2">
                                <button onclick={prev_year_page}>
                                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronLeft}/>
                                </button>
                                <span class="cursor-pointer">{format!("Years")}</span>
                                <button onclick={next_year_page}>
                                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronRight}/>
                                </button>
                            </div>
                            <div class="grid grid-cols-4 gap-1">
                                { render_years().into_iter().collect::<Html>() }
                            </div>
                        </>
                    }
                } else {
                    let days_of_week = ["S", "M", "T", "W", "T", "F", "S"];
                    html! {
                        <>
                            <div class="flex justify-between items-center mb-2">
                                <button onclick={Callback::from(move |_| current_month_left.set(if *current_month_left == 1 { current_year_left.set(*current_year_left - 1); 12 } else { *current_month_left - 1 }))}>
                                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronLeft}/>
                                </button>
                                <span onclick={toggle_viewing_years} class="cursor-pointer">{format!("{:?} {:?}", *current_year, chrono::Month::try_from(u8::try_from(*current_month).unwrap()).unwrap())}</span>
                                <button onclick={Callback::from(move |_| current_month_right.set(if *current_month_right == 12 { current_year_right.set(*current_year_right + 1); 1 } else { *current_month_right + 1 }))}>
                                    <Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={IconId::BootstrapChevronRight}/>
                                </button>
                            </div>
                            <div class="grid grid-cols-7 gap-1 text-gray-500">
                                { days_of_week.iter().map(|&day| html! { <div class="font-bold text-center">{day}</div> }).collect::<Html>() }
                                { render_days().into_iter().collect::<Html>() }
                            </div>
                        </>
                    }
                }
            }
        </div>
    }
}
