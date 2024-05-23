#![allow(dead_code)]
//https://rosepinetheme.com/palette/ingredients/
use druid::Color;
pub struct Theme {
    pub base: Color,
    pub surface: Color,
    pub overlay: Color,
    pub muted: Color,
    pub subtle: Color,
    pub text: Color
}
impl Theme {
    pub const ROSE_PINE: Theme = Theme {
        base: Color::rgb8(25, 23, 36),
        surface: Color::rgb8(31, 29, 46),
        overlay: Color::rgb8(38, 35, 58),
        muted: Color::rgb8(110, 106, 134),
        subtle: Color::rgb8(144, 140, 170),
        text: Color::rgb8(224, 222, 244),
    };
    pub const ROSE_PINE_MOON: Theme = Theme {
        base: Color::rgb8(35, 33, 54),
        surface: Color::rgb8(42, 39, 63),
        overlay: Color::rgb8(57, 53, 82),
        muted: Color::rgb8(110, 106, 134),
        subtle: Color::rgb8(144, 140, 170),
        text: Color::rgb8(224, 222, 244),
    };
    pub const ROSE_PINE_DAWN: Theme = Theme {
        base: Color::rgb8(250, 244, 237),
        surface: Color::rgb8(255, 250, 243),
        overlay: Color::rgb8(242, 233, 222),
        muted: Color::rgb8(152, 147, 165),
        subtle: Color::rgb8(121, 117, 147),
        text:   Color::rgb8(87, 82, 121),
    };
}
