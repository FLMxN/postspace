use crate::axis::Axis;

#[derive(Clone, Copy, Debug)]
pub struct Figure {
    pub name: &'static str,
    pub vector: [f64; Axis::ALL.len()],
}

const fn profile(
    name: &'static str,
    market: f64,
    liberty: f64,
    progressivism: f64,
    transhumanism: f64,
    democracy: f64,
    delusionality: f64,
    revolutionism: f64,
) -> Figure {
    let mut vector = [0.0; Axis::ALL.len()];
    vector[Axis::Market.index()] = market;
    vector[Axis::Liberty.index()] = liberty;
    vector[Axis::Progressivism.index()] = progressivism;
    vector[Axis::Transhumanism.index()] = transhumanism;
    vector[Axis::Democracy.index()] = democracy;
    vector[Axis::Delusionality.index()] = delusionality;
    vector[Axis::Revolutionism.index()] = revolutionism;
    Figure { name, vector }
}

pub const FIGURES: &[Figure] = &[
    // ── Neoreaction / Dark Enlightenment ────────────────────────────────
    profile(
        "Curtis Yarvin (Mencius Moldbug)",
        0.65,
        -0.45,
        -0.90,
        0.10,
        -0.95,
        0.7,
        0.8,
    ),
    profile("Nick Land", 0.90, 0.25, -0.45, 0.95, -0.90, 0.9, 0.75),
    profile(
        "Hans-Hermann Hoppe",
        0.95,
        -0.70,
        -0.80,
        0.05,
        -0.80,
        0.2,
        0.3,
    ),
    profile(
        "Michael Anissimov",
        0.45,
        0.25,
        -0.85,
        0.35,
        -0.90,
        0.2,
        0.1,
    ),
    profile("Spandrell", 0.25, -0.90, -0.90, 0.00, -0.90, 0.4, 0.1),
    profile("Steve Sailer", 0.20, -0.75, -0.90, -0.05, -0.70, -0.6, -0.2),
    // ── Post‑left / anarchist & primitivist ────────────────────────────
    profile(
        "Vaush (Ian Kochinski)",
        -0.35,
        0.75,
        0.85,
        0.30,
        0.85,
        -0.6,
        -0.4,
    ),
    profile("Hasan Piker", -0.70, 0.50, 0.90, 0.15, 0.80, -0.5, -0.2),
    profile("Noam Chomsky", -0.75, 0.90, 0.80, 0.20, 0.90, -0.2, 0.1),
    profile("Hakim Bey", -0.60, 0.95, 0.40, -0.30, 0.85, 0.7, 0.8),
    profile("Bob Black", -0.70, 0.90, 0.20, -0.40, 0.80, -0.2, 0.7),
    profile("John Zerzan", -0.85, 0.70, -0.10, -0.95, 0.60, -0.2, 0.9),
    // ── Left accelerationism / Post-capitalism ─────────────────────────
    profile("Nick Srnicek", -0.50, 0.55, 0.75, 0.65, 0.75, 0.2, 0.5),
    profile("Alex Williams", -0.55, 0.55, 0.75, 0.65, 0.75, 0.2, 0.5),
    profile("Mark Fisher", -0.65, 0.70, 0.60, 0.35, 0.80, 0.2, 0.2),
    profile("Benjamin Noys", -0.55, 0.65, 0.65, 0.20, 0.70, 0.2, 0.0),
    // ── Alt-right / Internet-native right ──────────────────
    profile("Jack Donovan", -0.15, -0.70, -0.80, -0.10, -0.70, 0.1, 0.1),
    profile("Michael Malice", 0.30, 0.80, 0.15, 0.10, 0.50, -0.4, 0.2),
    profile("Kantbot", 0.10, -0.75, -0.80, 0.15, -0.75, 0.6, 0.1),
    // ── Post‑right / identitarian ──────────────────────────
    profile(
        "Bronze Age Pervert (Costin Alamariu)",
        0.00,
        -0.95,
        -0.90,
        -0.20,
        -0.90,
        0.8,
        0.4,
    ),
    profile("Nick Fuentes", -0.60, -0.90, -0.95, -0.10, -0.90, -0.3, 0.3),
    profile("Guillaume Faye", -0.50, -0.80, -0.90, 0.15, -0.85, 0.4, 0.8),
    profile(
        "Alain de Benoist",
        -0.60,
        -0.50,
        -0.85,
        -0.10,
        -0.75,
        0.6,
        0.4,
    ),
    profile("Jason Jorjani", 0.20, -0.60, -0.70, 0.80, -0.40, 0.8, 0.4),
    // ── Speculative realism / Posthumanism ──────────────────────────────
    profile("Reza Negarestani", -0.10, 0.30, 0.45, 0.85, 0.25, 0.9, 0.1),
    profile("Ray Brassier", -0.10, 0.40, 0.25, 0.65, 0.20, 0.7, 0.1),
    // ── Classical liberal / libertarian / techno‑optimist ──────────────
    profile("Ayn Rand", 0.95, 0.90, -0.40, 0.35, 0.15, -0.6, -0.4),
    profile("Peter Thiel", 0.90, 0.75, -0.55, 0.95, -0.55, -0.2, -0.1),
    // ── Traditionalist / authoritarian right ───────────────────────────
    profile("Julius Evola", -0.80, -0.95, -0.95, -0.80, -0.95, 0.9, 0.7),
    profile(
        "Alexander Dugin",
        -0.85,
        -0.90,
        -0.90,
        -0.40,
        -0.95,
        0.9,
        0.9,
    ),
    // ── Internet‑famous political commentators (Anglosphere) ──────────
    profile(
        "Sargon of Akkad (Carl Benjamin)",
        0.50,
        0.10,
        -0.70,
        0.10,
        0.40,
        -0.5,
        -0.3,
    ),
    profile(
        "Destiny (Steven Bonnell II)",
        0.55,
        0.65,
        0.70,
        0.30,
        0.80,
        -0.7,
        -0.6,
    ),
    profile(
        "Jordan Peterson",
        0.80,
        0.40,
        -0.60,
        -0.10,
        0.20,
        -0.4,
        -0.3,
    ),
    profile("Andrew Tate", -0.20, -0.85, -0.90, -0.30, -0.80, -0.5, 0.0),
    profile(
        "Candace Owens",
        0.30,
        -0.30,
        -0.85,
        -0.15,
        -0.50,
        -0.4,
        -0.3,
    ),
    profile(
        "Sneako (Nicolas Kenn De Balinthazy)",
        -0.10,
        -0.75,
        -0.80,
        -0.20,
        -0.70,
        -0.4,
        0.0,
    ),
    profile(
        "xQc (Félix Lengyel)",
        0.00,
        0.20,
        0.05,
        0.00,
        0.10,
        -0.7,
        0.0,
    ),
    profile(
        "Asmongold (Zack Hoyt)",
        0.10,
        0.15,
        -0.30,
        0.00,
        0.10,
        -0.5,
        0.0,
    ),
    profile(
        "Markiplier (Mark Fischbach)",
        0.10,
        0.35,
        0.55,
        0.05,
        0.25,
        -0.6,
        0.0,
    ),
    // ── Political leaders / statespeople ─────────────────────────────
    profile("Javier Milei", 0.85, 0.65, -0.35, 0.15, 0.55, 0.2, -0.5),
    profile("Donald Trump", 0.05, -0.50, -0.80, 0.10, -0.80, -0.1, 0.2),
    profile("Bernie Sanders", -0.65, 0.70, 0.90, 0.20, 0.85, -0.6, -0.7),
    profile("Alexei Navalny", 0.35, 0.75, 0.45, 0.10, 0.80, -0.6, -0.6),
    profile("Deng Xiaoping", 0.30, -0.85, -0.15, 0.15, -0.90, -0.6, -0.6),
    profile("Xi Jinping", -0.45, -0.95, -0.65, 0.35, -0.95, -0.4, -0.5),
    // ── Russian / CIS information space ────────────────────────────────
    profile("Eduard Limonov", -0.80, -0.90, -0.60, 0.00, -0.80, 0.3, 0.8),
    profile(
        "Dmitry Puchkov (Goblin)",
        0.20,
        -0.50,
        -0.80,
        -0.10,
        -0.50,
        -0.5,
        -0.1,
    ),
    profile("Maxim Katz", 0.80, 0.90, 0.70, 0.20, 0.85, -0.6, -0.7),
    profile(
        "Alexander Shtefanov",
        0.30,
        0.70,
        0.40,
        0.10,
        0.70,
        -0.4,
        -0.3,
    ),
    profile("Stas Vasiliev", 0.10, -0.30, -0.70, -0.10, -0.30, -0.5, 0.0),
    // ── 20th-century revolutionaries / communist leaders ────────────────
    profile("Vladimir Lenin", -0.90, -0.95, 0.30, 0.00, -0.95, -0.2, 1.0),
    profile("Leon Trotsky", -0.95, -0.85, 0.65, 0.15, -0.90, 0.2, 1.0),
    profile(
        "Joseph Stalin",
        -0.95,
        -1.00,
        -0.15,
        -0.10,
        -0.98,
        -0.5,
        0.8,
    ),
    profile("Mao Zedong", -0.95, -1.00, 0.10, 0.00, -0.98, -0.2, 1.0),
    profile("Fidel Castro", -0.95, -0.95, 0.05, -0.10, -0.95, -0.4, 0.9),
    profile("Che Guevara", -0.95, -0.70, 0.50, 0.10, -0.75, -0.3, 1.0),
    // ── Fascism / authoritarian conservatism ────────────────────────────
    profile(
        "Benito Mussolini",
        -0.40,
        -1.00,
        -0.90,
        -0.10,
        -0.95,
        -0.2,
        0.8,
    ),
    profile("Adolf Hitler", -0.20, -1.00, -1.00, -0.20, -0.98, -0.3, 0.8),
];

pub fn find_figure(name: &str) -> Option<&'static Figure> {
    FIGURES.iter().find(|figure| figure.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_table_is_non_empty() {
        assert!(!FIGURES.is_empty());
    }

    #[test]
    fn all_vectors_are_within_unit_interval() {
        for figure in FIGURES {
            for &value in &figure.vector {
                assert!(
                    (-1.0..=1.0).contains(&value),
                    "{} has out‑of‑range value {}",
                    figure.name,
                    value
                );
            }
        }
    }

    #[test]
    fn figure_scores_match_representative_calibrations() {
        let lenin = find_figure("Vladimir Lenin").expect("Lenin profile should exist");
        let negarestani = find_figure("Reza Negarestani").expect("Negarestani profile should exist");
        let sanders = find_figure("Bernie Sanders").expect("Sanders profile should exist");
        let milei = find_figure("Javier Milei").expect("Milei profile should exist");
        let xi = find_figure("Xi Jinping").expect("Xi profile should exist");
        let trotsky = find_figure("Leon Trotsky").expect("Trotsky profile should exist");

        assert_eq!(lenin.vector[Axis::Revolutionism.index()], 1.0);
        assert_eq!(negarestani.vector[Axis::Delusionality.index()], 0.9);
        assert_eq!(sanders.vector[Axis::Revolutionism.index()], -0.7);
        assert_eq!(milei.vector[Axis::Market.index()], 0.85);
        assert_eq!(xi.vector[Axis::Market.index()], -0.45);
        assert_eq!(trotsky.vector[Axis::Liberty.index()], -0.85);
        assert_eq!(trotsky.vector[Axis::Democracy.index()], -0.90);
    }
}
