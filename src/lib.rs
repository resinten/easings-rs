#![feature(clamp)]

use std::f32::consts::PI;

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.0;
const C4: f32 = (2.0 * PI) / 3.0;
const C5: f32 = (2.0 * PI) / 4.5;
const N1: f32 = 7.5625;
const D1: f32 = 2.75;

#[derive(Clone, Copy, Debug)]
pub enum Easing {
    Linear,
    SineIn,
    SineOut,
    SineInOut,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    BackIn,
    BackOut,
    BackInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

pub fn ease(easing: Easing, x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    match easing {
        Easing::Linear => x,

        // Sine
        Easing::SineIn => 1.0 - (x * PI / 2.0).cos(),
        Easing::SineOut => (x * PI / 2.0).sin(),
        Easing::SineInOut => -((x * PI).cos() - 1.0) / 2.0,

        // Quad
        Easing::QuadIn => x.powf(2.0),
        Easing::QuadOut => 1.0 - (1.0 - x).powf(2.0),
        Easing::QuadInOut => {
            if x < 0.5 {
                2.0 * x.powf(2.0)
            } else {
                1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0
            }
        }

        // Cubic
        Easing::CubicIn => x.powf(3.0),
        Easing::CubicOut => 1.0 - (1.0 - x).powf(3.0),
        Easing::CubicInOut => {
            if x < 0.5 {
                4.0 * x.powf(3.0)
            } else {
                1.0 - (-2.0 * x + 2.0).powf(3.0) / 2.0
            }
        }

        // Quart
        Easing::QuartIn => x.powf(4.0),
        Easing::QuartOut => 1.0 - (1.0 - x).powf(4.0),
        Easing::QuartInOut => {
            if x < 0.5 {
                8.0 * x.powf(4.0)
            } else {
                1.0 - (-2.0 * x + 2.0).powf(4.0) / 2.0
            }
        }

        // Quint
        Easing::QuintIn => x.powf(5.0),
        Easing::QuintOut => 1.0 - (1.0 - x).powf(5.0),
        Easing::QuintInOut => {
            if x < 0.5 {
                16.0 * x.powf(5.0)
            } else {
                1.0 - (-2.0 * x + 2.0).powf(5.0) / 2.0
            }
        }

        // Expo
        Easing::ExpoIn => {
            if x <= 0.0 {
                0.0
            } else {
                2.0_f32.powf(10.0 * x - 10.0)
            }
        }
        Easing::ExpoOut => {
            if x >= 1.0 {
                1.0
            } else {
                1.0 - 2.0_f32.powf(-10.0 * x)
            }
        }
        Easing::ExpoInOut => {
            if x <= 0.0 {
                0.0
            } else if x >= 1.0 {
                1.0
            } else if x < 0.5 {
                2.0_f32.powf(20.0 * x - 10.0) / 2.0
            } else {
                (2.0 - 2.0_f32.powf(-20.0 * x + 10.0)) / 2.0
            }
        }

        // Circ
        Easing::CircIn => 1.0 - (1.0 - x.powf(2.0)).sqrt(),
        Easing::CircOut => (1.0 - (x - 1.0).powf(2.0)).sqrt(),
        Easing::CircInOut => {
            if x < 0.5 {
                (1.0 - (1.0 - (2.0 * x).powf(2.0)).sqrt()) / 2.0
            } else {
                ((1.0 - (-2.0 * x + 2.0).powf(2.0)).sqrt() + 1.0) / 2.0
            }
        }

        // Back
        Easing::BackIn => C3 * x.powf(3.0) - C1 * x.powf(2.0),
        Easing::BackOut => 1.0 + C3 * (x - 1.0).powf(3.0) + C1 * (x - 1.0).powf(2.0),
        Easing::BackInOut => {
            if x < 0.5 {
                (2.0 * x).powf(2.0) * ((C2 + 1.0) * 2.0 * x - 2.0) / 2.0
            } else {
                ((2.0 * x - 2.0).powf(2.0) * ((C2 + 1.0) * (x * 2.0 - 2.0) + C2) + 2.0) / 2.0
            }
        }

        // Elastic
        Easing::ElasticIn => {
            if x <= 0.0 {
                0.0
            } else if x >= 1.0 {
                1.0
            } else {
                -(2.0_f32.powf(10.0 * x - 10.0)) * (C4 * (x * 10.0 - 10.75)).sin()
            }
        }
        Easing::ElasticOut => {
            if x <= 0.0 {
                0.0
            } else if x >= 1.0 {
                1.0
            } else {
                2.0_f32.powf(-10.0 * x) * (C4 * (x * 10.0 - 0.75)).sin() + 1.0
            }
        }
        Easing::ElasticInOut => {
            if x <= 0.0 {
                0.0
            } else if x >= 1.0 {
                1.0
            } else if x < 0.5 {
                -(2.0_f32.powf(20.0 * x - 10.0) * (C5 * (20.0 * x - 11.125)).sin()) / 2.0
            } else {
                2.0_f32.powf(-20.0 * x + 10.0) * (C5 * (20.0 * x - 11.125)).sin() / 2.0 + 1.0
            }
        }

        // Bounce
        Easing::BounceIn => 1.0 - bounce_out(1.0 - x),
        Easing::BounceOut => bounce_out(x),
        Easing::BounceInOut => {
            if x < 0.5 {
                (1.0 - bounce_out(1.0 - 2.0 * x)) / 2.0
            } else {
                (1.0 + bounce_out(2.0 * x - 1.0)) / 2.0
            }
        }
    }
}

fn bounce_out(x: f32) -> f32 {
    if x < 1.0 / D1 {
        N1 * x.powf(2.0)
    } else if x < 2.0 / D1 {
        let x = x - 1.5 / D1;
        N1 * x.powf(2.0) + 0.75
    } else if x < 2.5 / D1 {
        let x = x - 2.25 / D1;
        N1 * x.powf(2.0) + 0.9375
    } else {
        let x = x - 2.625 / D1;
        N1 * x.powf(2.0) + 0.984_375
    }
}
