use itertools::Itertools;
use leptos::{svg::*, *};
use leptos_use::*;
use num_traits::ToPrimitive;
use std::{f64::consts::TAU, iter};

use crate::{ChartColor, Palette, CATPPUCCIN_COLORS};

pub struct PieChartOptions {
    pub max_ticks: u8,
    pub color: Box<dyn ChartColor>,
}

impl Default for PieChartOptions {
    fn default() -> Self {
        Self {
            max_ticks: 5u8,
            color: Box::new(Palette(CATPPUCCIN_COLORS.clone())),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PieSegment {
    from: (f64, f64),
    to: (f64, f64),
    value: f64,
}
enum SegmentSize {
    LessThanHalf,
    Half,
    MoreThanHalf,
}
impl PieSegment {
    fn angle(&self) -> SegmentSize {
        let zcross = self.from.0 * self.to.1 - self.to.0 * self.from.1;
        if zcross == 0.0 {
            SegmentSize::Half
        } else if zcross > 0.0 {
            SegmentSize::LessThanHalf
        } else {
            SegmentSize::MoreThanHalf
        }
    }
    fn get_arc_path(&self) -> String {
        let angle = self.angle();

        let large_arc_flag = match angle {
            SegmentSize::LessThanHalf | SegmentSize::Half => 0,
            SegmentSize::MoreThanHalf => 1,
        };

        format!(
            "M0 0 {from_x} {from_y} A100 100 0 {arc_flag} 1 {to_x} {to_y}Z",
            from_x = self.from.0,
            from_y = self.from.1,
            to_x = self.to.0,
            to_y = self.to.1,
            arc_flag = large_arc_flag
        )
    }

    // Gets a middle vector for two vectors in a circle segment
    // This points in the direction of a circle segment's center
    // even if the angle of the segment is >= 180°
    // uses the cross product to figure out the angle and flips the vector
    // if it's larger than 180°. For the 180° case, it creates a new vector
    // 90° clockwise perpendicular to the from vector.
    fn get_center_unit_vector(&self) -> (f64, f64) {
        match self.angle() {
            SegmentSize::Half => {
                let magnitude = f64::sqrt(self.from.0.powi(2) + self.from.1.powi(2));
                (self.from.1 / magnitude, -self.from.0 / magnitude)
            }
            SegmentSize::LessThanHalf => {
                let new_x = (self.from.0 + self.to.0) / 2.0;
                let new_y = (self.from.1 + self.to.1) / 2.0;
                let magnitude = f64::sqrt(new_x.powi(2) + new_y.powi(2));

                (new_x / magnitude, new_y / magnitude)
            }
            SegmentSize::MoreThanHalf => {
                let new_x = (self.from.0 + self.to.0) / 2.0;
                let new_y = (self.from.1 + self.to.1) / 2.0;
                let magnitude = f64::sqrt(new_x.powi(2) + new_y.powi(2));

                (-new_x / magnitude, -new_y / magnitude)
            }
        }
    }
}

#[component]
pub fn PieChart<T>(
    values: MaybeSignal<Vec<T>>,
    options: Box<PieChartOptions>,
    // colors: Option<&'chart [&'chart str]>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: ToPrimitive + Clone + PartialOrd + 'static,
{
    let values = create_memo(move |_| {
        values
            .get()
            .iter()
            .map(|v| v.to_f64().unwrap())
            .collect::<Vec<f64>>()
    });
    let num_pies = create_memo(move |_| values.get().len());
    let sum = create_memo(move |_| values.get().iter().sum::<f64>());
    let sorted_values = create_memo(move |_| {
        iter::once((0.0, 99.0, 0.0))
            .chain(
                values
                    .get()
                    .into_iter()
                    .sorted_by(|a, b| f64::partial_cmp(a, b).unwrap())
                    .map(|f| (f, f / sum.get()))
                    .scan((0.0, 0.0), |state, v| {
                        *state = (v.0, state.1 + v.1);
                        Some(*state)
                    })
                    .map(|(f, v)| (f, (v * TAU).cos() * 99.0, (v * TAU).sin() * 99.0)),
            )
            .map_windows(|[from, to]| PieSegment {
                from: (from.1, from.2),
                to: (to.1, to.2),
                value: to.0,
            })
            .collect::<Vec<PieSegment>>()
    });

    view! {
        <svg {..attrs}>
            {move || {
                sorted_values
                    .get()
                    .into_iter()
                    .enumerate()
                    .map(|(i, segment)| {
                        let el = create_node_ref::<Path>();
                        let is_hovered = use_element_hover(el);
                        let label_pos = segment.get_center_unit_vector();
                        let color = String::from(options.color.color_for_index(i, num_pies.get()));
                        view! {
                            <svg viewBox="0 0 200 200">
                                <g transform="translate(100,100)" stroke="#000" stroke-width="1">
                                    <mask id=format!("cut-path-{}", i)>
                                        <path
                                            d=segment.get_arc_path()
                                            fill="white"
                                            stroke="black"
                                            stroke-width="2"
                                            vector-effect="non-scaling-stroke"
                                        ></path>
                                    </mask>
                                    <path
                                        node_ref=el
                                        d=segment.get_arc_path()
                                        fill=color.clone()
                                        fill-opacity=0.6
                                        stroke=color
                                        stroke-width="2"
                                        vector-effect="non-scaling-stroke"
                                        mask=move || {
                                            if is_hovered.get() {
                                                "none".to_string()
                                            } else {
                                                format!("url(#cut-path-{})", i)
                                            }
                                        }
                                    >
                                    </path>
                                    <Show when=move || is_hovered.get() fallback=|| ()>
                                        <text
                                            font-size="15px"
                                            vector-effect="non-scaling-stroke"
                                            x=label_pos.0 * 85.0
                                            y=label_pos.1 * 85.0
                                        >
                                            <tspan
                                                text-anchor="middle"
                                                dominant-baseline="middle"
                                                color="#000"
                                            >
                                                {segment.value}
                                            </tspan>
                                        </text>
                                    </Show>
                                </g>
                            </svg>
                        }
                    })
                    .collect_view()
            }}

        </svg>
    }
}
