use leptos::component;
use leptos::leptos_dom::logging::console_log;
use leptos::{svg::*, *};

#[component]
pub fn YAxis(ticks: Memo<Vec<(f64, String)>>) -> impl IntoView {
    let svg_ref = create_node_ref::<Svg>();
    view! {
        <svg y="0%" height="100%" _ref=svg_ref>
            <line
                x1="9.8%"
                y1="10%"
                x2="9.8%"
                y2="90%"
                stroke="black"
                stroke-width="1px"
                vector-effect="non-scaling-stroke"
            ></line>
            {move || {
                ticks
                    .get()
                    .into_iter()
                    .map(|(t, s)| {
                        let node_ref = create_node_ref::<Text>();
                        create_effect(move |_| {
                            if let Some(parent) = svg_ref.get() {
                                if let Some(elem) = node_ref.get() {
                                    request_animation_frame(move || {
                                        let parent_width = parent
                                            .get_bounding_client_rect()
                                            .width();
                                        let target_width = parent_width * 0.069;
                                        let text_size = elem.get_bounding_client_rect();
                                        console_log(
                                            format!(
                                                "{},{},{}", text_size.width(), target_width, parent_width
                                            )
                                                .as_str(),
                                        );
                                        if text_size.width() > target_width {
                                            let factor = target_width / text_size.width();
                                            console_log(format!("{:?}", factor).as_str());
                                            elem.set_attribute(
                                                    "font-size",
                                                    format!("{:.2?}em", factor).as_str(),
                                                )
                                                .expect("the fontsize to be changed");
                                            elem.set_attribute(
                                                    "dy",
                                                    format!("{:.2?}em", factor / 2.0).as_str(),
                                                )
                                                .expect("to set dy");
                                        }
                                    });
                                }
                            }
                        });
                        let t = 10.0 + t * 0.8;
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
                                _ref=node_ref
                                x="6.9%"
                                y=format!("{}%", t)
                                font-size="1em"
                                dy="0.5em"
                                text-anchor="end"
                                vector-effect="non-scaling-stroke"
                                lengthAdjust="spacing"
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
