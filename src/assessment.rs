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
            "Should private companies be allowed to own and operate major utilities?",
            "Should adults be able to start a business without obtaining a special government license?",
            "Should all government-owned enterprises be transferred to private ownership?",
            "Should all taxes on business profits and investment be abolished?",
            "Should essential public services be privatized and run without regulatory oversight?",
            "Should the state stay entirely out of setting prices and allocating basic goods?",
        ],
    },
    QuestionGroup {
        axis: Axis::Market,
        pole: Pole::Negative,
        prompts: &[
            "Should the state own all major industries and banks?",
            "Should the government set all wages and prices across the economy?",
            "Should healthcare be guaranteed through publicly funded providers?",
            "Should higher taxes on large fortunes fund public services?",
            "Should the government nationalize all strategic sectors and direct production?",
            "Should every major utility and transport network be excluded from private ownership?",
        ],
    },
    QuestionGroup {
        axis: Axis::Liberty,
        pole: Pole::Positive,
        prompts: &[
            "Should people be allowed to criticize their government without legal punishment?",
            "Should peaceful demonstrations be allowed without advance government approval?",
            "Should adults be free to use any psychoactive substance without criminal penalty?",
            "Should the government be barred from restricting any peaceful activity or voluntary exchange among adults?",
            "Should adults be free to form any nonviolent private association, including armed defense groups, without government permission?",
            "Should the state be forbidden from criminalizing essentially all consensual nonviolent acts between adults?",
        ],
    },
    QuestionGroup {
        axis: Axis::Liberty,
        pole: Pole::Negative,
        prompts: &[
            "Should police be allowed to search a home without a warrant when they suspect a serious crime?",
            "Should the government be allowed to detain people indefinitely without charge during a declared emergency?",
            "Should the state require citizens to obtain permission before publishing any political opinion?",
            "Should the government be able to censor media that criticizes its policies?",
            "Should the state be empowered to suspend civil liberties and impose emergency rule without judicial review?",
            "Should the government be allowed to ban political opposition entirely if it claims the public order is at risk?",
        ],
    },
    QuestionGroup {
        axis: Axis::Progressivism,
        pole: Pole::Positive,
        prompts: &[
            "Should public schools teach that a person's gender identity may differ from their sex assigned at birth?",
            "Should universities be allowed to consider race in admissions to address historic discrimination?",
            "Should reparations be paid to descendants of enslaved and colonized peoples?",
            "Should gender-affirming care be publicly funded and available to minors without parental consent?",
            "Should the state require every institution to affirm each person's self-identified gender, even when it conflicts with biological sex?",
            "Should public policy treat sex and gender categories as fluid social constructs, not biological facts?",
        ],
    },
    QuestionGroup {
        axis: Axis::Progressivism,
        pole: Pole::Negative,
        prompts: &[
            "Should public schools teach that gender is determined by sex assigned at birth?",
            "Should abortion be legal only in cases of rape, incest, or serious risk to the pregnant person's life?",
            "Should same-sex couples be excluded from adopting children?",
            "Should public schools be required to teach that traditional gender roles are morally superior?",
            "Should biological sex be recognized as the sole legal basis for social policy and public accommodation?",
            "Should any school or institution teaching gender ideology be denied public funding?",
        ],
    },
    QuestionGroup {
        axis: Axis::Transhumanism,
        pole: Pole::Positive,
        prompts: &[
            "Should military personnel be allowed to use neural implants that improve reaction time?",
            "Should healthy adults be allowed to replace a healthy limb with a robotic one that increases strength?",
            "Should healthy adults be allowed to edit their genes for cosmetic or performance traits without medical justification?",
            "Should brain-computer implants be made available to children for cognitive enhancement with parental consent?",
            "Should citizens be allowed to merge their minds with digital systems and AI networks for enhanced cognition?",
            "Should the state subsidize radical bodily enhancement as a public good for citizens?",
        ],
    },
    QuestionGroup {
        axis: Axis::Transhumanism,
        pole: Pole::Negative,
        prompts: &[
            "Should parents be prohibited from selecting embryos for traits such as height or intelligence?",
            "Should any medical procedure intended to enhance normal human abilities be prohibited by law?",
            "Should international treaties ban all germline genetic engineering and human enhancement research?",
            "Should brain-computer implants be restricted to people with a diagnosed medical need?",
            "Should all non-therapeutic enhancement, including cosmetic gene editing, be illegal even for consenting adults?",
            "Should governments ban any research that could produce human enhancement beyond treatment of disease?",
        ],
    },
    QuestionGroup {
        axis: Axis::Democracy,
        pole: Pole::Positive,
        prompts: &[
            "Should political leaders be chosen through competitive elections rather than appointment?",
            "Should governments be required to hold regular elections with multiple political parties?",
            "Should political power be distributed among multiple branches of government to prevent abuse?",
            "Should the media be free to criticize government policies without censorship?",
            "Should citizens be able to directly propose and vote on binding laws through referendums?",
            "Should political power be shared through citizen assemblies, sortition, and direct democratic mechanisms alongside elections?",
        ],
    },
    QuestionGroup {
        axis: Axis::Democracy,
        pole: Pole::Negative,
        prompts: &[
            "Should a strong leader be able to make decisions without being constrained by elections?",
            "Should the ruling party be allowed to control all branches of government?",
            "Should elections be suspended if they threaten national stability?",
            "Should political power be concentrated in a single leader or party without checks?",
            "Should the state be allowed to suspend the constitution and rule by emergency decree indefinitely?",
            "Should political competition be permanently abolished in favor of a single-party order?",
        ],
    },
    QuestionGroup {
        axis: Axis::Delusionality,
        pole: Pole::Positive,
        prompts: &[
            "Is political theory more valuable when it deliberately breaks with ordinary language and familiar categories?",
            "Should political movements embrace ideas that seem impossible or incoherent by today's standards?",
            "Can contradictions be more politically illuminating than a consistent, practical program?",
            "Should political ideas be allowed to remain deliberately ambiguous rather than made accessible to everyone?",
            "Should political theory deliberately weaponize confusion to dismantle consensus reality?",
            "Should political discourse value incommunicability and radical opacity over any demand for public clarity?",
        ],
    },
    QuestionGroup {
        axis: Axis::Delusionality,
        pole: Pole::Negative,
        prompts: &[
            "Should political arguments use clear language that ordinary people can readily understand?",
            "Is it important for political movements to explain how their plans would be implemented?",
            "Should political leaders avoid promises that cannot be supported by evidence?",
            "Should political movements prioritize achievable improvements over abstract visions?",
            "Should all political proposals be judged primarily by measurable social outcomes rather than moral intuition?",
            "Should every policy be required to come with a concrete implementation plan before it is considered legitimate?",
        ],
    },
    QuestionGroup {
        axis: Axis::Revolutionism,
        pole: Pole::Positive,
        prompts: &[
            "Should people organize to replace the existing political and economic order rather than gradually reform it?",
            "Can revolutionary change be justified when institutions block meaningful progress?",
            "Should a movement reject compromise if it would preserve the existing system?",
            "Is dismantling deeply unjust institutions preferable to trying to improve them incrementally?",
            "Should a movement aim to abolish the existing state and replace it with an entirely new social order?",
            "Should a movement embrace temporary chaos if it makes the existing order impossible to restore?",
        ],
    },
    QuestionGroup {
        axis: Axis::Revolutionism,
        pole: Pole::Negative,
        prompts: &[
            "Should political change usually happen through gradual reforms within existing institutions?",
            "Are lasting improvements more likely when built through incremental policy changes?",
            "Are negotiated compromises generally preferable to attempts to overthrow the system?",
            "Should political movements focus on achievable policy changes rather than replacing the entire order?",
            "Should movement strategy prioritize stability and institutional continuity over any revolutionary rupture?",
            "Should activists reject revolutionary tactics as irresponsible except when all incremental options are exhausted?",
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
