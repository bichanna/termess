use std::{io::stdout, thread, time::Duration};

use crossterm::{
    style::{Color, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
};
use once_cell::sync::Lazy;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

static RAINBOW_COLORS: Lazy<Vec<Color>> = Lazy::new(|| {
    vec![
        Color::Red,
        Color::Yellow,
        Color::Green,
        Color::Blue,
        // Indigo
        Color::Rgb {
            r: 75,
            g: 0,
            b: 130,
        },
        // Violet
        Color::Rgb {
            r: 127,
            g: 0,
            b: 255,
        },
        // Orange,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
    ]
});

fn get_random_color(rng: &mut ThreadRng) -> Color {
    RAINBOW_COLORS.choose(rng).unwrap().clone()
}

fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    loop {
        let foreground_color = get_random_color(&mut rng);
        let background_color = get_random_color(&mut rng);

        stdout()
            .execute(SetForegroundColor(foreground_color))?
            .execute(SetBackgroundColor(background_color))?;

        thread::sleep(Duration::from_millis(rng.gen_range(100..1500)));
    }
}
