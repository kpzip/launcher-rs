#[macro_export]
macro_rules! gradient {
    ($ang:expr; ($r1:expr, $g1:expr, $b1:expr), ($r2:expr, $g2:expr, $b2:expr)) => {
        Gradient::Linear(Linear::new($ang).add_stop(0.0, color!($r1, $g1, $b1)).add_stop(1.0, color!($r2, $g2, $b2)))
    };
}

#[macro_export]
macro_rules! button_text {
    ($text:literal, $predicate:expr) => {
        if $predicate {
            concat!(">", $text, "<")
        } else {
            $text
        }
    };
}

pub use button_text;
pub use gradient;
