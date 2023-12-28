use leptos::component;
use leptos::{svg::*, *};

#[component]
pub fn YAxis(ticks: Memo<Vec<(f64, String)>>) -> impl IntoView {
    view! {
        <svg y="10%" height="80%" overflow="visible">
            <line
                x1="9.8%"
                y1="0%"
                x2="9.8%"
                y2="100%"
                stroke="black"
                stroke-width="1px"
                vector-effect="non-scaling-stroke"
            ></line>
            {move || {
                ticks
                    .get()
                    .into_iter()
                    .map(|(t, s)| {
                        view! {
                            <line
                                x1="7%"
                                y1=format!("{}%", t)
                                x2="9.8%"
                                y2=format!("{}%", t)
                                stroke="black"
                                strocke-width="1px"
                                vector-effect="non-scaling-stroke"
                            ></line>
                            <text
                                x="6.9%"
                                y=format!("{}%", t)
                                font-size="20px"
                                dy="5px"
                                text-anchor="end"
                                vector-effect="non-scaling-stroke"
                            >
                                {s}
                            </text>
                        }
                    })
                    .collect_view()
            }}

        </svg>
    }
}
