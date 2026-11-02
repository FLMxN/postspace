use std::{error::Error, fs, path::PathBuf};

use plotters::{prelude::*, style::FontStyle};

use crate::{axis::Axis, vectors::Figure};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 1600;
const BACKGROUND: RGBColor = RGBColor(8, 7, 13);
const PANEL: RGBColor = RGBColor(35, 25, 66);
const TEXT: RGBColor = RGBColor(255, 255, 255);
const MUTED: RGBColor = RGBColor(183, 170, 211);
const POSITIVE: RGBColor = RGBColor(255, 105, 180);
const NEGATIVE: RGBColor = RGBColor(160, 115, 255);

pub fn render(
    token: &str,
    vector: &[f64; Axis::ALL.len()],
    matched_ideas: &[&str],
    top_figures: &[(&'static Figure, f64)],
) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all("cards")?;
    let path = PathBuf::from("cards").join(format!("{}_card.png", safe_filename(token)));
    {
        let root = BitMapBackend::new(&path, (WIDTH, HEIGHT)).into_drawing_area();
        root.fill(&BACKGROUND)?;

        root.draw(&Text::new(
            "POSTSPACE  /  POLITICAL PROFILE",
            (72, 76),
            ("sans-serif", 30)
                .into_font()
                .style(FontStyle::Bold)
                .color(&MUTED),
        ))?;
        root.draw(&Text::new(
            token,
            (72, 160),
            ("sans-serif", 64)
                .into_font()
                .style(FontStyle::Bold)
                .color(&TEXT),
        ))?;
        root.draw(&Text::new(
            "A snapshot from your answers",
            (76, 220),
            ("sans-serif", 28).into_font().color(&MUTED),
        ))?;
        root.draw(&PathElement::new(
            vec![(72, 270), (1128, 270)],
            MUTED.mix(0.35),
        ))?;
        let track_left = 120;
        let track_right = 1080;
        let center = (track_left + track_right) / 2;
        for (index, axis) in Axis::ALL.iter().enumerate() {
            let top = 326 + index as i32 * 88;
            let value = vector[axis.index()].clamp(-1.0, 1.0);
            let color = if value >= 0.0 { POSITIVE } else { NEGATIVE };
            root.draw(&Text::new(
                axis.name(),
                (122, top - 20),
                ("sans-serif", 32)
                    .into_font()
                    .style(FontStyle::Bold)
                    .color(&TEXT),
            ))?;
            root.draw(&Text::new(
                format!("{value:+.2}"),
                (1070, top - 20),
                ("sans-serif", 28).into_font().color(&color),
            ))?;
            root.draw(&Rectangle::new(
                [(track_left, top + 20), (track_right, top + 38)],
                PANEL.filled(),
            ))?;
            root.draw(&PathElement::new(
                vec![(center, top + 16), (center, top + 42)],
                MUTED.mix(0.7),
            ))?;
            let offset = (value.abs() * ((track_right - track_left) / 2) as f64).round() as i32;
            let (left, right) = if value >= 0.0 {
                (center, center + offset)
            } else {
                (center - offset, center)
            };
            if offset > 0 {
                root.draw(&Rectangle::new(
                    [(left, top + 20), (right, top + 38)],
                    color.filled(),
                ))?;
            }
            root.draw(&Text::new(
                axis.negative_pole(),
                (122, top + 50),
                ("sans-serif", 24).into_font().color(&MUTED),
            ))?;
            root.draw(&Text::new(
                axis.positive_pole(),
                (1000, top + 50),
                ("sans-serif", 24).into_font().color(&MUTED),
            ))?;
        }

        root.draw(&PathElement::new(
            vec![(72, 950), (1128, 950)],
            MUTED.mix(0.35),
        ))?;
        root.draw(&Text::new(
            "POSSIBLE IDEOLOGICAL OVERLAPS",
            (72, 1002),
            ("sans-serif", 48)
                .into_font()
                .style(FontStyle::Bold)
                .color(&TEXT),
        ))?;
        if matched_ideas.is_empty() {
            root.draw(&Text::new(
                "No exact labels matched this profile.",
                (76, 1060),
                ("sans-serif", 24).into_font().color(&MUTED),
            ))?;
        } else {
            for (index, idea) in matched_ideas.iter().take(5).enumerate() {
                root.draw(&Text::new(
                    *idea,
                    (76, 1060 + index as i32 * 38),
                    ("sans-serif", 32).into_font().color(&TEXT),
                ))?;
            }
            if matched_ideas.len() > 5 {
                root.draw(&Text::new(
                    format!("+ {} more", matched_ideas.len() - 5),
                    (76, 1060 + 5 * 38),
                    ("sans-serif", 21).into_font().color(&MUTED),
                ))?;
            }
        }

        root.draw(&PathElement::new(
            vec![(72, 1305), (1128, 1305)],
            MUTED.mix(0.35),
        ))?;
        root.draw(&Text::new(
            "CLOSEST FIGURES",
            (72, 1358),
            ("sans-serif", 30)
                .into_font()
                .style(FontStyle::Bold)
                .color(&TEXT),
        ))?;
        for (index, (figure, similarity)) in top_figures.iter().take(3).enumerate() {
            let y = 1422 + index as i32 * 48;
            root.draw(&Text::new(
                format!("{}. {}", index + 1, figure.name),
                (76, y),
                ("sans-serif", 32).into_font().color(&TEXT),
            ))?;
            root.draw(&Text::new(
                format!("{similarity:.0}%"),
                (1040, y),
                ("sans-serif", 32).into_font().color(&POSITIVE),
            ))?;
        }
        root.draw(&Text::new(
            "A playful comparison, not a scientific assessment.",
            (76, 1570),
            ("sans-serif", 19).into_font().color(&MUTED),
        ))?;
        root.present()?;
    }
    Ok(path)
}

fn safe_filename(token: &str) -> String {
    let name: String = token
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if name.is_empty() {
        "profile".into()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_portrait_result_card() {
        let token = format!("card-test-{}", std::process::id());
        let path = render(&token, &[0.2, -0.4, 0.0, 0.8, -1.0, 0.3, -0.6], &[], &[])
            .expect("result card should render");
        let png = fs::read(&path).expect("rendered card should exist");
        assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert_eq!(u32::from_be_bytes(png[16..20].try_into().unwrap()), WIDTH);
        assert_eq!(u32::from_be_bytes(png[20..24].try_into().unwrap()), HEIGHT);
        assert_eq!(WIDTH * 4, HEIGHT * 3);
        fs::remove_file(path).expect("test card should be removable");
    }
}
