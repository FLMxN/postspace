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
) -> Figure {
    let mut vector = [0.0; Axis::ALL.len()];
    vector[Axis::Market.index()] = market;
    vector[Axis::Liberty.index()] = liberty;
    vector[Axis::Progressivism.index()] = progressivism;
    vector[Axis::Transhumanism.index()] = transhumanism;
    vector[Axis::Democracy.index()] = democracy;
    Figure { name, vector }
}

pub const FIGURES: &[Figure] = &[
    // ── Neoreaction / Dark Enlightenment ────────────────────────────────
    profile("Curtis Yarvin (Mencius Moldbug)", 0.65, -0.45, -0.90, 0.10, -0.95),
    profile("Nick Land", 0.90, 0.25, -0.75, 0.95, -0.90),
    profile("Hans-Hermann Hoppe", 0.95, -0.70, -0.80, 0.05, -0.80),
    profile("Michael Anissimov", 0.45, 0.25, -0.85, 0.35, -0.90),
    profile("Spandrell", 0.25, -0.90, -0.90, 0.00, -0.90),
    profile("Steve Sailer", 0.20, -0.75, -0.90, -0.05, -0.70),

    // ── Post‑left / anarchist & primitivist ────────────────────────────
    profile("Vaush (Ian Kochinski)", -0.35, 0.75, 0.85, 0.30, 0.85),
    profile("Hasan Piker", -0.70, 0.50, 0.90, 0.15, 0.80),
    profile("Noam Chomsky", -0.75, 0.90, 0.80, 0.20, 0.90),
    profile("Hakim Bey", -0.60, 0.95, 0.40, -0.30, 0.85),
    profile("Bob Black", -0.70, 0.90, 0.20, -0.40, 0.80),
    profile("John Zerzan", -0.85, 0.70, -0.10, -0.95, 0.60),

    // ── Left accelerationism / Post-capitalism ─────────────────────────
    profile("Nick Srnicek", -0.50, 0.55, 0.75, 0.65, 0.75),
    profile("Alex Williams", -0.55, 0.55, 0.75, 0.65, 0.75),
    profile("Mark Fisher", -0.65, 0.70, 0.60, 0.35, 0.80),
    profile("Benjamin Noys", -0.55, 0.65, 0.65, 0.20, 0.70),

    // ── Alt-right / Internet-native right ──────────────────
    profile("Jack Donovan", -0.15, -0.70, -0.80, -0.10, -0.70),
    profile("Michael Malice", 0.30, 0.80, 0.15, 0.10, 0.50),
    profile("Kantbot", 0.10, -0.75, -0.80, 0.15, -0.75),

    // ── Post‑right / identitarian ──────────────────────────
    profile("Bronze Age Pervert (Costin Alamariu)", 0.00, -0.95, -0.90, -0.20, -0.90),
    profile("Nick Fuentes", -0.60, -0.90, -0.95, -0.10, -0.90),
    profile("Guillaume Faye", -0.50, -0.80, -0.90, 0.15, -0.85),
    profile("Alain de Benoist", -0.60, -0.50, -0.85, -0.10, -0.75),
    profile("Jason Jorjani", 0.20, -0.60, -0.70, 0.80, -0.40),

    // ── Speculative realism / Posthumanism ──────────────────────────────
    profile("Reza Negarestani", -0.10, 0.30, 0.45, 0.85, 0.25),
    profile("Ray Brassier", -0.10, 0.40, 0.25, 0.65, 0.20),

    // ── Classical liberal / libertarian / techno‑optimist ──────────────
    profile("Ayn Rand", 0.95, 0.90, -0.40, 0.35, 0.15),
    profile("Peter Thiel", 0.90, 0.75, -0.55, 0.95, -0.55),

    // ── Traditionalist / authoritarian right ───────────────────────────
    profile("Julius Evola", -0.80, -0.95, -0.95, -0.80, -0.95),
    profile("Alexander Dugin", -0.85, -0.90, -0.90, -0.40, -0.95),

    // ── Internet‑famous political commentators (Anglosphere) ──────────
    profile("Sargon of Akkad (Carl Benjamin)", 0.50, 0.10, -0.70, 0.10, 0.40),
    profile("Destiny (Steven Bonnell II)", 0.55, 0.65, 0.70, 0.30, 0.80),
    profile("Jordan Peterson", 0.80, 0.40, -0.60, -0.10, 0.20),
    profile("Andrew Tate", -0.20, -0.85, -0.90, -0.30, -0.80),
    profile("Candace Owens", 0.30, -0.30, -0.85, -0.15, -0.50),
    profile("Sneako (Nicolas Kenn De Balinthazy)", -0.10, -0.75, -0.80, -0.20, -0.70),
    profile("xQc (Félix Lengyel)", 0.00, 0.20, 0.05, 0.00, 0.10),
    profile("Asmongold (Zack Hoyt)", 0.10, 0.15, -0.30, 0.00, 0.10),
    profile("Markiplier (Mark Fischbach)", 0.10, 0.35, 0.55, 0.05, 0.25),

    // ── Political leaders / statespeople ─────────────────────────────
    profile("Javier Milei", 1.00, 0.65, -0.35, 0.15, 0.55),
    profile("Donald Trump", 0.05, -0.50, -0.80, 0.10, -0.80),
    profile("Bernie Sanders", -0.65, 0.70, 0.90, 0.20, 0.85),
    profile("Alexei Navalny", 0.35, 0.75, 0.45, 0.10, 0.80),
    profile("Deng Xiaoping", 0.30, -0.85, -0.15, 0.15, -0.90),
    profile("Xi Jinping", -0.25, -0.95, -0.65, 0.35, -0.95),

    // ── Russian / CIS information space ────────────────────────────────
    profile("Eduard Limonov", -0.80, -0.90, -0.60, 0.00, -0.80),
    profile("Dmitry Puchkov (Goblin)", 0.20, -0.50, -0.80, -0.10, -0.50),
    profile("Maxim Katz", 0.80, 0.90, 0.70, 0.20, 0.85),
    profile("Alexander Shtefanov", 0.30, 0.70, 0.40, 0.10, 0.70),
    profile("Stas Vasiliev", 0.10, -0.30, -0.70, -0.10, -0.30),

    // ── 20th-century revolutionaries / communist leaders ────────────────
    profile("Vladimir Lenin", -0.90, -0.95, 0.30, 0.00, -0.95),
    profile("Leon Trotsky", -0.95, -0.50, 0.65, 0.15, -0.80),
    profile("Joseph Stalin", -0.95, -1.00, -0.15, -0.10, -0.98),
    profile("Mao Zedong", -0.95, -1.00, 0.10, 0.00, -0.98),
    profile("Fidel Castro", -0.95, -0.95, 0.05, -0.10, -0.95),
    profile("Che Guevara", -0.95, -0.70, 0.50, 0.10, -0.75),

    // ── Fascism / authoritarian conservatism ────────────────────────────
    profile("Benito Mussolini", -0.40, -1.00, -0.90, -0.10, -0.95),
    profile("Adolf Hitler", -0.20, -1.00, -1.00, -0.20, -0.98),

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
}