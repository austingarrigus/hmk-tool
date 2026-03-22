enum Society {
    Feudal,
    Imperial,
    Tribal,
    Tributary,
    Kuzhan,
    Sinain,
}

struct Occupation {
    name: String,
    wealth_die: u8,
    skills: (String, u8),
    description: String,
}

const OCCUPATIONS: [Occupation; 79] = [
    Occupation{
        name: "alchemist",
        wealth_die: 4,

Alchemy 4 Discourse 2
Mathematics 3 Physician 1
Herblore 2 Native Language 1
Mineralogy 2 Script 4
Folklore 2
An arcanist who creates elixirs by
enchanting the Pvaric Principles
found in herbs and minerals.
];
