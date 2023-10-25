# Leptos Charts

A charting library built with Rust and the Leptos framework. Renders to pure SVG which is responsive, no code needed to adjust for view changes.

Currently supports Bar and Pie charts.

## Examples

### Bar Chart
![Bar](https://github.com/Panaetius/leptos_charts/blob/main/doc/Bar.png?raw=true)

Simple bar chart that is reactive and responsive, can take plain Vecs or a Leptos signal with a Vec.

Usage:

```rust
use leptos_charts::*;

let data: Vec<f64> = vec![2.0, 3.0, 1.5, 7.0, 1.0, 2.5, 9.9];
let options = Box::new(BarChartOptions {
    max_ticks: 4,
    color: Box::new(Palette(CATPPUCCIN_COLORS.clone())),
});
[...]
<BarChart
values=data.into()
options=options
attr:style="margin-top:5px"
attr:preserveAspectRatio="none"
attr:width="300"
attr:height="200"
/>
```

### Pie Chart
![Pie](https://github.com/Panaetius/leptos_charts/blob/main/doc/Pie.png?raw=true)

Usage:

```rust
use leptos_charts::*;

let data: Vec<f64> = vec![2.0, 3.0, 1.5, 7.0, 1.0, 2.5, 9.9];
let options = Box::new(PieChartOptions {
    color: Box::new(Palette(CATPPUCCIN_COLORS.clone())),
});

// In view!{}
<PieChart
values=data.into()
options=options
attr:width="300"
attr:height="200"
/>
```
