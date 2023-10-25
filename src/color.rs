use once_cell::sync::Lazy;
pub static CATPPUCCIN_COLORS: Lazy<Vec<Color>> = Lazy::new(|| {
    vec![
        Color::Hex("#dc8a78"), //rosewater
        Color::Hex("#8839ef"), //Mauve
        Color::Hex("#fe640b"), //Peach
        Color::Hex("#40a02b"), //green
        Color::Hex("#04a5e5"), //Sky
        Color::Hex("#ea76cb"), //Pink
        Color::Hex("#1e66f5"), //Blue
        Color::Hex("#d20f39"), //Red
        Color::Hex("#df8e1d"), //yellow
        Color::Hex("#209fb5"), //Sapphire
        Color::Hex("#7287fd"), //lavender
        Color::Hex("#e64553"), //maroon
    ]
});

#[derive(Debug, Clone)]
pub enum Color<'a> {
    Hex(&'a str),
    RGB(u8, u8, u8),
}
impl From<Color<'_>> for String {
    fn from(color: Color) -> String {
        match color {
            Color::Hex(s) => s.to_string(),
            Color::RGB(r, g, b) => format!("#{:02x?}{:02x?}{:02x?}", r, g, b),
        }
    }
}
impl From<Color<'_>> for (u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8) {
        match color {
            Color::Hex(hex) => {
                assert!(hex.len() == 7);
                (
                    u8::from_str_radix(&hex[1..3], 16)
                        .expect("Couldn't convert hex string to u8 for Color"),
                    u8::from_str_radix(&hex[3..5], 16)
                        .expect("Couldn't convert hex string to u8 for Color"),
                    u8::from_str_radix(&hex[5..7], 16)
                        .expect("Couldn't convert hex string to u8 for Color"),
                )
            }
            Color::RGB(r, g, b) => (r, g, b),
        }
    }
}
pub struct Palette<'a>(pub Vec<Color<'a>>);
pub struct Gradient<'a> {
    pub from: Color<'a>,
    pub to: Color<'a>,
}
pub struct CalculatedColor<'a, F>
where
    F: Fn(usize, usize) -> Color<'a>,
{
    func: F,
}

pub trait ChartColor {
    fn color_for_index(&self, i: usize, total: usize) -> Color;
}
impl ChartColor for Palette<'_> {
    fn color_for_index(&self, i: usize, _total: usize) -> Color {
        self.0[i % self.0.len()].clone()
    }
}
impl ChartColor for Gradient<'_> {
    fn color_for_index(&self, i: usize, total: usize) -> Color {
        let from_color: (u8, u8, u8) = self.from.clone().into();
        let to_color: (u8, u8, u8) = self.to.clone().into();
        Color::RGB(
            ((to_color.0 as i64 - from_color.0 as i64) * i as i64 / total as i64
                + from_color.0 as i64) as u8,
            ((to_color.1 as i64 - from_color.1 as i64) * i as i64 / total as i64
                + from_color.1 as i64) as u8,
            ((to_color.2 as i64 - from_color.2 as i64) * i as i64 / total as i64
                + from_color.2 as i64) as u8,
        )
    }
}
impl<'a, F> ChartColor for CalculatedColor<'a, F>
where
    F: Fn(usize, usize) -> Color<'a>,
{
    fn color_for_index(&self, i: usize, total: usize) -> Color<'a> {
        (self.func)(i, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_color_to_string() {
        let color = Color::Hex("#3489bc");
        let str: String = color.into();
        assert_eq!(str, "#3489bc");
    }
    #[test]
    fn rgb_color_to_string() {
        let color = Color::RGB(128, 200, 7);
        let str: String = color.into();
        assert_eq!(str, "#80c807");
    }
    #[test]
    fn test_palette() {
        let palette = Palette(CATPPUCCIN_COLORS.clone());
        assert_eq!(String::from(palette.color_for_index(0, 100)), "#dc8a78");
        assert_eq!(String::from(palette.color_for_index(5, 100)), "#ea76cb");
        assert_eq!(String::from(palette.color_for_index(11, 100)), "#e64553");
        assert_eq!(String::from(palette.color_for_index(12, 100)), "#dc8a78");
    }
    #[test]
    fn test_gradient() {
        let gradient = Gradient {
            from: Color::RGB(0, 0, 0),
            to: Color::RGB(255, 255, 255),
        };
        assert_eq!(
            <(u8, u8, u8)>::from(gradient.color_for_index(0, 255)),
            (0, 0, 0)
        );
        assert_eq!(
            <(u8, u8, u8)>::from(gradient.color_for_index(128, 255)),
            (128, 128, 128)
        );
        assert_eq!(
            <(u8, u8, u8)>::from(gradient.color_for_index(255, 255)),
            (255, 255, 255)
        );

        let gradient = Gradient {
            from: Color::RGB(0, 100, 200),
            to: Color::RGB(4, 96, 204),
        };
        assert_eq!(
            <(u8, u8, u8)>::from(gradient.color_for_index(0, 4)),
            (0, 100, 200)
        );
        assert_eq!(
            <(u8, u8, u8)>::from(gradient.color_for_index(4, 4)),
            (4, 96, 204)
        );
        assert_eq!(
            <(u8, u8, u8)>::from(gradient.color_for_index(2, 4)),
            (2, 98, 202)
        );
    }
}
