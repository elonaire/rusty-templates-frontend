use yew::*;
use yew_icons::{IconId, Icon};

use crate::components::card::Card;

#[derive(Properties, PartialEq)]
pub struct StatsCardProps {
    pub title: String,
    pub subtitle: String,
    pub value: i32,
    pub icon: IconId,
    // #[prop_or_default]
    // pub children: Children,
    // pub theme: String,
    pub upward_trend: bool,
    pub percentage: u32,
}

#[function_component]
pub fn StatsCard(props: &StatsCardProps) -> Html {
    let trend_class = if props.upward_trend {
        "text-theme-green"
    } else {
        "text-theme-red"
    };

    html! {
        <Card>
            <div class="flex flex-row items-center gap-2">
                <div class="flex flex-col">
                    <div class="p-4 rounded-lg bg-blue-200 text-blue-500">
                        <Icon width={"1.5em".to_owned()} height={"1.5em".to_owned()} icon_id={props.icon}/>
                    </div>
                    <div class={classes!("text-xs", "mt-1", trend_class)}>{ format!("{}{}% ", if props.upward_trend { "+" } else { "-" }, props.percentage) }<Icon width={"1em".to_owned()} height={"1em".to_owned()} icon_id={ if props.upward_trend { IconId::LucideTrendingUp } else { IconId::LucideTrendingDown } }/></div>
                </div>
                <div>
                    <p class="text-md font-semibold text-gray-600 md:min-h-12">{ &props.title }</p>
                    <p class="text-xs text-gray-500">{ &props.subtitle }</p>
                    <p class="text-3xl font-bold text-gray-800">{ format!("${}", &props.value) }</p>
                </div>
            </div>
        </Card>
    }
}
