// ideas.rs

pub fn assess(vector: [f64; 5]) {
    let [m, l, p, t, d] = vector;

    let ideas: Vec<(&str, bool)> = vec![
        // ── Neoreaction / Dark Enlightenment ──
        ("neoreactionary (NRx)",                 m >  0.20 && l < -0.65 && p < -0.65 && d < -0.65),
        ("Moldbuggian neocameralist",            m >  0.40 && l < -0.80 && p < -0.80 && d < -0.80),
        ("Landian accelerationist",              m >  0.65 && l < -0.60 && p < -0.40 && t > 0.65 && d < -0.65),
        ("Dark Enlightenment techno-monarchist", m >  0.55 && l < -0.85 && t > 0.75 && d < -0.85),
        ("Bronze Age mindset (BAPist)",          m.abs() < 0.2 && l < -0.90 && p < -0.85 && t < 0.1 && d < -0.85),
        ("Hoppean",                              m >  0.85 && l < -0.50 && p < -0.60 && d < -0.60),
        ("reactionary conservative",             m <  0.35 && l < -0.70 && p < -0.75 && d < -0.65),

        // ── Libertarian / crypto / exit ──
        ("anarcho-capitalist",                   m >  0.75 && l >  0.75 && p <  0.4 && d > 0.60),
        ("voluntaryist",                         m >  0.40 && l >  0.05 && p > -0.45 && p < 0.4 && d > 0.35),
        ("crypto-anarchist",                     m >  0.65 && l >  0.75 && t >  0.35 && d > 0.50),
        ("cyber-libertarian",                    m >  0.55 && l >  0.65 && t >  0.45 && d > 0.50),
        ("seasteader / exit advocate",           m >  0.55 && l >  0.65 && t >  0.35 && p > -0.05 && d > 0.55),
        ("paleolibertarian",                     m >  0.75 && l >  0.15 && p < -0.65 && d > 0.40),
        ("mutualist",                            m < -0.15 && l >  0.75 && p >  0.15 && d > 0.60),
        ("Georgist",                             m.abs() < 0.2 && l > 0.55 && p > 0.25 && d > 0.55),
        ("egoist / Stirnerite",                  l >  0.85 && m.abs() < 0.55 && d > 0.55),
        ("techno-authoritarian libertarian",     m >  0.75 && l > -0.60 && t >  0.80 && d <  0.30),
        ("classical liberal conservative",       m >  0.35 && l > -0.20 && p < -0.55 && d > 0.10),
        ("traditional liberal",                  m >  0.70 && l >  0.30 && p < -0.50 && d >  0.10),

        // ── Left-anarchist / post-left ──
        ("anarcho-communist",                    m <= -0.75 && l > 0.65 && p > 0.65 && d > 0.75),
        ("anarcho-syndicalist",                  m <= -0.60 && l > 0.65 && p > 0.45 && t < 0.3 && d > 0.65),
        ("council communist",                    m < -0.75 && l > 0.45 && p > 0.55 && d > 0.70),
        ("platformist",                          m < -0.60 && l > 0.55 && p > 0.45 && d > 0.65),
        ("insurrectionist anarchist",            m < -0.45 && l > 0.85 && p > 0.35 && d > 0.70),
        ("situationist / post-leftist",          m < -0.45 && l > 0.65 && p > 0.35 && d > 0.65),
        ("democratic confederalist (Öcalan-style)", m < -0.45 && l > 0.65 && p > 0.55 && d > 0.80),
        ("communalist (Bookchin-style)",         m < -0.50 && l > 0.65 && p > 0.55 && d > 0.75),
        ("social ecologist",                     m < -0.40 && l > 0.65 && p > 0.45 && t < 0.3 && d > 0.65),
        ("anti-work / post-work advocate",       m < -0.40 && l > 0.55 && p > 0.55 && t > 0.45 && d > 0.65),

        // ── Anarcho-primitivism / anti-civ / deep green ──
        ("anarcho-primitivist",                  m < -0.55 && l > 0.65 && t < -0.75 && d > 0.55),
        ("anti-civ rewilding advocate",          m < -0.45 && l > 0.55 && t < -0.75 && d > 0.50),
        ("neo-Luddite",                          l >  0.45 && t < -0.85 && d > 0.45),
        ("deep ecologist / Earth First!-style",  m < -0.50 && l > 0.55 && p > 0.35 && t < -0.65 && d > 0.55),

        // ── Accelerationism / futures ──
        ("left accelerationist",                 m < -0.50 && l > 0.40 && p > 0.65 && t > 0.60 && d > 0.55),
        ("FALGSC (fully automated luxury gay space communist)", m < -0.60 && l > 0.40 && p > 0.80 && t > 0.60 && d > 0.70),
        ("post-scarcity communist",              m < -0.60 && l > 0.10 && p > 0.50 && t > 0.60 && d > 0.65),
        ("techno-socialist",                     m < -0.50 && l > 0.10 && p > 0.50 && t > 0.60 && d > 0.60),
        ("singularitarian",                      t >  0.60 && m > -0.25 && l > -0.25 && d > 0.10),
        ("anarcho-transhumanist",                l >  0.80 && t >  0.80 && d > 0.75),
        ("technocratic transhumanist",           l < -0.30 && t >  0.80 && d < -0.35),
        ("cyberocracy advocate / AI-state",      l < -0.40 && p >  0.10 && t > 0.60 && d < -0.65),
        ("techno-feudalist",                     m >  0.55 && l < -0.55 && t > 0.40 && d < -0.60),
        ("neocameralist",                        m >  0.50 && l < -0.50 && t > -0.10 && d < -0.55),
        ("xenofeminist",                         l >  0.10 && p >  0.70 && t > 0.70 && d > 0.65),
        ("cyberfeminist",                        l >  0.10 && p >  0.60 && t > 0.60 && d > 0.60),
        ("post-genderist",                       p >  0.80 && t >  0.50 && d > 0.55),

        // ── Green-tech / governance futures ──
        ("ecomodernist",                         m >  0.10 && l > 0.10 && p > 0.30 && t > 0.60 && d > 0.45),
        ("solarpunk",                            m < -0.30 && l > 0.10 && p > 0.50 && t > 0.50 && d > 0.70),
        ("lunarpunk",                            m < -0.30 && l > 0.30 && p > 0.40 && t > 0.40 && t < 0.75 && d > 0.65),
        ("degrowth advocate",                    m < -0.40 && p > 0.40 && t < 0.15 && d > 0.50),
        ("longtermist",                          m.abs() < 0.45 && l > 0.40 && p > 0.40 && t > 0.60 && d > 0.50),
        ("effective altruist",                   m >  0.30 && l > 0.40 && p > 0.50 && t > 0.40 && d > 0.55),
        ("futarchist",                           m >  0.60 && l > 0.40 && d > 0.40),
        ("lottocratic / sortitionist",           m <  0.25 && l > 0.30 && p > 0.30 && d > 0.80),
        ("liquid democracy advocate",            l >  0.50 && p > 0.35 && d > 0.65),

        // ── Russian cosmism / God-building ──
        ("Russian cosmist",                      m < -0.40 && p > 0.30 && t > 0.60 && d > 0.25),
        ("God-builder (bogostroitel)",           m < -0.40 && p > 0.30 && t > 0.45 && t < 0.70 && d > 0.20),

        // ── Bioconservatism / anti-enhancement ──
        ("bioconservative",                      t < -0.60 && p < -0.40 && d < -0.25),
        ("religious bioconservative",            t < -0.80 && p < -0.60 && d < -0.40),
        ("left bioconservative",                 m < -0.40 && p > 0.50 && t < -0.40 && d > 0.35),

        // ── Traditionalist / authoritarian right ──
        ("Evolian traditionalist",               m < -0.60 && l < -0.80 && p < -0.80 && d < -0.80),
        ("Guénonian perennialist",               m < -0.60 && l < -0.70 && p < -0.80 && d < -0.75),
        ("Catholic integralist",                 m.abs() < 0.45 && l < -0.60 && p < -0.80 && d < -0.70),
        ("Christian Reconstructionist / dominionist", l < -0.70 && p < -0.85 && d < -0.75),
        ("eco-fascist",                          m < -0.40 && l < -0.60 && p < -0.40 && t < -0.10 && d < -0.70),
        ("environmental authoritarian",          m < -0.30 && l < -0.70 && t < -0.10 && d < -0.75),
        ("national-anarchist",                   m < -0.10 && l > 0.40 && p < -0.70 && d < -0.45),
        ("national-bolshevik (Nazbol)",          m < -0.70 && l < -0.80 && p < -0.50 && d < -0.75),
        ("Eurasianist / Duginist",               m < -0.70 && l < -0.80 && p < -0.80 && d < -0.85),
        ("Strasserist",                          m < -0.60 && l < -0.80 && p < -0.60 && d < -0.80),
        ("Falangist",                            m < -0.40 && l < -0.85 && p < -0.80 && d < -0.80),
        ("paleoconservative",                    m >  0.40 && l < -0.40 && p < -0.80 && d < -0.45),
        ("MAGA populist",                        m >  0.20 && l < -0.50 && p < -0.70 && d < -0.40),
        ("esoteric Hitlerist (Savitri Devi-style)", l < -0.90 && p < -0.90 && t < -0.40 && d < -0.85),
        ("folkish pagan / Wotanist",             l < -0.60 && p < -0.80 && d < -0.65),
        ("race realist / HBD-aware",             l < -0.50 && p < -0.70 && d < -0.50),
        ("conservative anti-woke",               m >  0.20 && l < -0.20 && p < -0.75 && d < -0.40),
        ("identitarian",                         l < -0.50 && p <= -0.75 && d < -0.50),
        ("ethno-pluralist",                      m.abs() < 0.45 && l < -0.30 && p < -0.70 && d < -0.25),
        ("authoritarian state capitalist",       m < 0.1 && m > -0.5 && l < -0.85 && p < -0.50 && d < -0.85),

        // ── Marxist-Leninist / 20th-century left ──
        ("Marxist-Leninist (vanguardist)",       m < -0.85 && l < -0.80 && p > -0.20 && d < -0.65),
        ("Trotskyist",                           m < -0.85 && l < -0.45 && p > 0.50 && d > -0.90),
        ("Maoist",                               m < -0.85 && l < -0.80 && p < 0.25 && d < -0.75),
        ("Hoxhaist",                             m < -0.90 && l < -0.85 && p < 0.15 && d < -0.80),
        ("Juche",                                m < -0.70 && l < -0.80 && p < -0.40 && d < -0.90),
        ("Ba'athist",                            m < -0.60 && l < -0.80 && p > -0.10 && p < 0.35 && d < -0.75),
        ("Chavista / Bolivarian",                m < -0.60 && l < -0.60 && p > 0.20 && d > 0.15),
        ("Peronist",                             m < -0.40 && l < -0.50 && p > -0.15 && p < 0.35 && d > 0.05),

        // ── Religious authoritarian ──
        ("Islamic theocrat",                     l < -0.80 && p < -0.80 && d < -0.85),
        ("Salafi-jihadist",                      l < -0.90 && p < -0.85 && d < -0.90),
    ];

    println!("\nBased on your answers, you may be ideologically similar to:");
    for (name, matched) in ideas {
        if matched {
            println!("{name}");
        }
    }
}