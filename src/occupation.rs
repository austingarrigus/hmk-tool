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

