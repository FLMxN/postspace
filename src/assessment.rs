use crate::axis::Axis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pole {
    Positive,
    Negative,
}

impl Pole {
    pub const fn multiplier(self) -> i32 {
        match self {
            Self::Positive => 1,
            Self::Negative => -1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Question {
    pub text: &'static str,
    pub axis: Axis,
    pub pole: Pole,
}

struct QuestionGroup {
    axis: Axis,
    pole: Pole,
    prompts: &'static [&'static str],
}

const QUESTION_GROUPS: &[QuestionGroup] = &[
    QuestionGroup {
        axis: Axis::Market,
        pole: Pole::Positive,
        prompts: &[
            "Should private companies be allowed to set prices without government caps?",
            "Should adults be able to start a business without obtaining a special government license?",
            "Should private companies be allowed to own and operate major utilities?",
            "Should employers and workers be free to negotiate wages without a government-mandated minimum?",
            "Should public services be opened to competition from private providers?",
            "Should the government avoid using taxpayer money to rescue failing companies?",
            "Should all government-owned enterprises be transferred to private ownership?",
            "Should all taxes on business profits and investment be abolished?",
        ],
    },
    QuestionGroup {
        axis: Axis::Market,
        pole: Pole::Negative,
        prompts: &[
            "Should the government set price limits on essential goods in normal economic conditions?",
            "Should the state own major industries such as energy and rail?",
            "Should healthcare be guaranteed through publicly funded providers?",
            "Should higher taxes on large fortunes fund public services?",
            "Should governments provide essential utilities directly instead of relying on private firms?",
            "Should governments subsidize key industries to protect domestic jobs?",
            "Should the state own all major industries and banks?",
            "Should the government set all wages and prices across the economy?",
        ],
    },
    QuestionGroup {
        axis: Axis::Liberty,
        pole: Pole::Positive,
        prompts: &[
            "Should people be allowed to criticize their government without legal punishment?",
            "Should parents be allowed to educate their children at home?",
            "Should peaceful demonstrations be allowed without advance government approval?",
            "Should terminally ill adults be allowed to make end-of-life medical decisions with a physician?",
            "Should police need a court warrant to search a person's private digital messages?",
            "Should people be allowed to publish opinions that most citizens find offensive?",
            "Should adults be free to use any psychoactive substance without criminal penalty?",
            "Should the government be barred from restricting any peaceful activity or voluntary exchange among adults?",
        ],
    },
    QuestionGroup {
        axis: Axis::Liberty,
        pole: Pole::Negative,
        prompts: &[
            "Should police be allowed to search a home without a warrant when they suspect a serious crime?",
            "Should the government be allowed to ban public demonstrations during periods of unrest?",
            "Should the government block websites it considers harmful to public order?",
            "Should governments monitor private communications without individual warrants to prevent terrorism?",
            "Should people be jailed for publicly insulting elected officials?",
            "Should authorities impose curfews on adults during peaceful periods to reduce crime?",
            "Should the government be allowed to detain people indefinitely without charge during a declared emergency?",
            "Should the state require citizens to obtain permission before publishing any political opinion?",
        ],
    },
    QuestionGroup {
        axis: Axis::Progressivism,
        pole: Pole::Positive,
        prompts: &[
            "Should public schools teach that a person's gender identity may differ from their sex assigned at birth?",
            "Should employers be required to report pay differences by gender and race?",
            "Should universities be allowed to consider race in admissions to address historic discrimination?",
            "Should both parents receive equal paid leave after the birth or adoption of a child?",
            "Should anti-discrimination laws explicitly protect people based on gender identity?",
            "Should abortion remain legally available during the first trimester?",
            "Should reparations be paid to descendants of enslaved and colonized peoples?",
            "Should gender-affirming care be publicly funded and available to minors without parental consent?",
        ],
    },
    QuestionGroup {
        axis: Axis::Progressivism,
        pole: Pole::Negative,
        prompts: &[
            "Should marriage laws give preference to traditional married-parent households?",
            "Should public schools teach that gender is determined by sex assigned at birth?",
            "Should abortion be legal only in cases of rape, incest, or serious risk to the pregnant person's life?",
            "Should schools need parental consent before teaching about sexuality and gender identity?",
            "Should public-school materials present traditional gender roles as the social norm?",
            "Should same-sex couples be excluded from adopting children?",
            "Should divorce be legally restricted to cases of abuse or adultery?",
            "Should public schools be required to teach that traditional gender roles are morally superior?",
        ],
    },
    QuestionGroup {
        axis: Axis::Transhumanism,
        pole: Pole::Positive,
        prompts: &[
            "Should clinical trials test brain implants that improve memory beyond typical levels?",
            "Should governments fund research aimed at substantially extending healthy human lifespan?",
            "Should military personnel be allowed to use neural implants that improve reaction time?",
            "Should parents be allowed to select embryos for traits such as higher predicted intelligence?",
            "Should healthy adults be allowed to replace a healthy limb with a robotic one that increases strength?",
            "Should healthy adults be able to get brain-computer implants for faster digital interaction?",
            "Should healthy adults be allowed to edit their genes for cosmetic or performance traits without medical justification?",
            "Should brain-computer implants be made available to children for cognitive enhancement with parental consent?",
        ],
    },
    QuestionGroup {
        axis: Axis::Transhumanism,
        pole: Pole::Negative,
        prompts: &[
            "Should genetic edits intended to enhance embryos be prohibited?",
            "Should brain implants be limited to treating diagnosed medical conditions, not boosting normal abilities?",
            "Should parents be prohibited from selecting embryos for traits such as height or intelligence?",
            "Should healthy limbs not be replaced with robotic limbs solely to improve physical performance?",
            "Should cognitive-enhancement drugs be prohibited for healthy adults outside clinical trials?",
            "Should brain-computer implants be restricted to people with a diagnosed medical need?",
            "Should any medical procedure intended to enhance normal human abilities be prohibited by law?",
            "Should international treaties ban all germline genetic engineering and human enhancement research?",
        ],
    },
    QuestionGroup {
        axis: Axis::Democracy,
        pole: Pole::Positive,
        prompts: &[
            "Should all adult citizens have the right to vote in free and fair elections?",
            "Should political leaders be chosen through competitive elections rather than appointment?",
            "Should governments be required to hold regular elections with multiple political parties?",
            "Should citizens be able to remove elected officials through recall elections or referendums?",
            "Should political power be distributed among multiple branches of government to prevent abuse?",
            "Should civil society organizations operate independently of government control?",
            "Should the media be free to criticize government policies without censorship?",
            "Should all citizens have equal voting rights regardless of wealth, status, or background?",
        ],
    },
    QuestionGroup {
        axis: Axis::Democracy,
        pole: Pole::Negative,
        prompts: &[
            "Should a strong leader be able to make decisions without being constrained by elections?",
            "Should the government be able to restrict political opposition during times of crisis?",
            "Should voting rights be limited to those with education or property qualifications?",
            "Should political parties that oppose the current government be banned?",
            "Should the ruling party be allowed to control all branches of government?",
            "Should the government be able to censor media that criticizes its policies?",
            "Should elections be suspended if they threaten national stability?",
            "Should political power be concentrated in a single leader or party without checks?",
        ],
    },
];

pub fn all_questions() -> Vec<Question> {
    QUESTION_GROUPS
        .iter()
        .flat_map(|group| {
            group.prompts.iter().map(move |text| Question {
                text: *text,
                axis: group.axis,
                pole: group.pole,
            })
        })
        .collect()
}