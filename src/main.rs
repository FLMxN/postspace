mod assessment;
mod axis;
mod vectors;
mod ideas;
mod card;

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
    let args: Vec<String> = std::env::args().skip(1).collect();
    if matches!(args.as_slice(), [flag] if flag == "-h" || flag == "--help") {
        print_usage();
        return;
    }

    let vector = match parse_vector_args(&args) {
        Ok(Some(vector)) => vector,
        Ok(None) => run_questionnaire(),
        Err(error) => {
            eprintln!("{error}");
            print_usage();
            std::process::exit(2);
        }
    };
    let id = rand::random::<u32>();

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
    let matched_ideas = assess(vector);

    let top_3 = top_figures(&vector, 3);
    println!("\nTop 3 closest figures:");
    for (i, (fig, similarity)) in top_3.iter().enumerate() {
        println!("{}. {} ({:.0}%)", i + 1, fig.name, similarity);
    }

    match card::render(&format!("№ {id}"), &vector, &matched_ideas, &top_3) {
        Ok(path) => println!("\nResult card saved to {}", path.display()),
        Err(error) => eprintln!("Could not render result card: {error}"),
    }
}

fn run_questionnaire() -> [f64; Axis::ALL.len()] {
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
                std::process::exit(1);
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
                    std::process::exit(1);
                }
            }
        }
    }

    std::array::from_fn(|index| normalize_score(scores[index], answered[index]))
}

fn parse_vector_args(args: &[String]) -> Result<Option<[f64; Axis::ALL.len()]>, String> {
    if args.is_empty() {
        return Ok(None);
    }
    if args[0] != "--vector" {
        return Err(format!("Unknown argument: {}", args[0]));
    }
    if args.len() != Axis::ALL.len() + 1 {
        return Err(format!(
            "--vector requires exactly {} coordinates",
            Axis::ALL.len()
        ));
    }

    let mut vector = [0.0; Axis::ALL.len()];
    for (index, value) in args[1..].iter().enumerate() {
        let coordinate = value
            .parse::<f64>()
            .map_err(|_| format!("Coordinate {} is not a number: {value}", index + 1))?;
        if !coordinate.is_finite() || !(-1.0..=1.0).contains(&coordinate) {
            return Err(format!(
                "Coordinate {} must be finite and between -1 and 1: {value}",
                index + 1
            ));
        }
        vector[index] = coordinate;
    }
    Ok(Some(vector))
}

fn print_usage() {
    println!(
        "Usage: postspace [--vector <market> <liberty> <progressivism> <transhumanism> <democracy>]"
    );
    println!("Without arguments, start the interactive quiz. Use --help for this message.");
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
    use super::{normalize_score, parse_answer, parse_vector_args, score_for};
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

    #[test]
    fn parses_vector_arguments_and_rejects_invalid_coordinates() {
        let args = ["--vector", "0.2", "-0.4", "0", "0.8", "-1"]
            .map(String::from);
        assert_eq!(
            parse_vector_args(&args),
            Ok(Some([0.2, -0.4, 0.0, 0.8, -1.0]))
        );
        assert_eq!(parse_vector_args(&[]), Ok(None));

        let out_of_range = ["--vector", "0", "0", "1.1", "0", "0"].map(String::from);
        assert!(parse_vector_args(&out_of_range).is_err());

        let wrong_count = ["--vector", "0", "0"].map(String::from);
        assert!(parse_vector_args(&wrong_count).is_err());
    }
}
