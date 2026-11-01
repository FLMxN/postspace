mod assessment;
mod axis;
mod vectors;
mod ideas;

use assessment::{Question, all_questions};
use crate::ideas::assess;
use axis::Axis;
use rand::seq::SliceRandom;
use std::io::{self, Write};

fn euclidean_distance_sq(a: &[f64; Axis::ALL.len()], b: &[f64; Axis::ALL.len()]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
}

fn similarity_percentage(dist_sq: f64) -> f64 {
    const MAX_DIST_SQ: f64 = 20.0; // 5 dimensions, each ranging from -1 to 1, so max diff is 2 per dimension
    ((1.0 - dist_sq / MAX_DIST_SQ) * 100.0).round()
}

fn top_figures(vector: &[f64; Axis::ALL.len()], n: usize) -> Vec<(&'static vectors::Figure, f64)> {
    let mut figures: Vec<_> = vectors::FIGURES
        .iter()
        .map(|fig| {
            let dist_sq = euclidean_distance_sq(&fig.vector, vector);
            (fig, dist_sq)
        })
        .collect();
    figures.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    figures
        .into_iter()
        .take(n)
        .map(|(fig, dist_sq)| (fig, similarity_percentage(dist_sq)))
        .collect()
}

fn main() {
    let id = rand::random::<u32>();
    let mut questions = all_questions();
    questions.shuffle(&mut rand::rng());

    let mut scores = [0; Axis::ALL.len()];
    let mut answered = [0; Axis::ALL.len()];

    println!("Answer with y/n/x or 1/-1/0. y and 1 mean yes; n and -1 mean no; x and 0 mean neutral.");

    'questions: for (index, question) in questions.iter().enumerate() {
        loop {
            print!(
                "\nQuestion {}/{}: {}\n[y/n/x/1/-1/0] > ",
                index + 1,
                questions.len(),
                question.text
            );
            if io::stdout().flush().is_err() {
                eprintln!("Could not write the question prompt.");
                return;
            }

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    println!("\nInput closed. Showing partial results.");
                    break 'questions;
                }
                Ok(_) => match parse_answer(&input) {
                    Some(response) => {
                        let axis_index = question.axis.index();
                        scores[axis_index] += score_for(*question, response);
                        answered[axis_index] += 1;
                        break;
                    }
                    None => println!("Please enter y, n, x, 1, -1, or 0."),
                },
                Err(error) => {
                    eprintln!("Could not read your answer: {error}");
                    return;
                }
            }
        }
    }

    let vector: [f64; Axis::ALL.len()] =
        std::array::from_fn(|index| normalize_score(scores[index], answered[index]));

    println!("\nResults:");
    println!(
        "Profile {id}\n[{:+.2}, {:+.2}, {:+.2}, {:+.2}, {:+.2}]",
        vector[0], vector[1], vector[2], vector[3], vector[4]
    );
    for axis in Axis::ALL {
        let score = vector[axis.index()];
        let pole = if score > 0.0 {
            axis.positive_pole()
        } else if score < 0.0 {
            axis.negative_pole()
        } else {
            "balanced"
        };
        println!("{}: {score:+.2} ({pole})", axis.name());
    }
    assess(vector);
    
    let top_3 = top_figures(&vector, 3);
    println!("\nTop 3 closest figures:");
    for (i, (fig, similarity)) in top_3.iter().enumerate() {
        println!("{}. {} ({:.0}%)", i + 1, fig.name, similarity);
    }
}

fn parse_answer(input: &str) -> Option<i32> {
    match input.trim().to_ascii_lowercase().as_str() {
        "y" | "1" => Some(1),
        "n" | "-1" => Some(-1),
        "x" | "0" => Some(0),
        _ => None,
    }
}

fn score_for(question: Question, response: i32) -> i32 {
    response * question.pole.multiplier()
}

fn normalize_score(score: i32, answered: usize) -> f64 {
    if answered == 0 {
        0.0
    } else {
        (score as f64 / answered as f64).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_score, parse_answer, score_for};
    use crate::assessment::{Pole, Question, all_questions};
    use crate::axis::Axis;

    #[test]
    fn parses_all_supported_answers() {
        assert_eq!(parse_answer("Y"), Some(1));
        assert_eq!(parse_answer("1"), Some(1));
        assert_eq!(parse_answer("n"), Some(-1));
        assert_eq!(parse_answer("-1"), Some(-1));
        assert_eq!(parse_answer("x"), Some(0));
        assert_eq!(parse_answer("0"), Some(0));
        assert_eq!(parse_answer("maybe"), None);
    }

    #[test]
    fn answers_score_toward_the_question_pole() {
        let question = Question {
            text: "",
            axis: Axis::Market,
            pole: Pole::Negative,
        };

        assert_eq!(score_for(question, 1), -1);
        assert_eq!(score_for(question, -1), 1);
        assert_eq!(score_for(question, 0), 0);
    }

    #[test]
    fn question_bank_has_eighty_questions() {
        assert_eq!(all_questions().len(), 80);
    }

    #[test]
    fn normalizes_scores_to_the_unit_interval() {
        assert_eq!(normalize_score(8, 8), 1.0);
        assert_eq!(normalize_score(-8, 8), -1.0);
        assert_eq!(normalize_score(2, 4), 0.5);
        assert_eq!(normalize_score(0, 0), 0.0);
    }
}
