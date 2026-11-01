#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Axis {
    Market,
    Liberty,
    Progressivism,
    Transhumanism,
    Democracy,
}

impl Axis {
    pub const ALL: [Self; 5] = [
        Self::Market,
        Self::Liberty,
        Self::Progressivism,
        Self::Transhumanism,
        Self::Democracy,
    ];

    pub const fn index(self) -> usize {
        match self {
            Self::Market => 0,
            Self::Liberty => 1,
            Self::Progressivism => 2,
            Self::Transhumanism => 3,
            Self::Democracy => 4,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Market => "Market",
            Self::Liberty => "Liberty",
            Self::Progressivism => "Progressivism",
            Self::Transhumanism => "Transhumanism",
            Self::Democracy => "Democracy",
        }
    }

    pub const fn positive_pole(self) -> &'static str {
        match self {
            Self::Market => "free-market",
            Self::Liberty => "individual liberty",
            Self::Progressivism => "progressive",
            Self::Transhumanism => "enhancement",
            Self::Democracy => "democracy",
        }
    }

    pub const fn negative_pole(self) -> &'static str {
        match self {
            Self::Market => "state-directed",
            Self::Liberty => "authority",
            Self::Progressivism => "traditionalist",
            Self::Transhumanism => "naturalist",
            Self::Democracy => "autocracy",
        }
    }
}
