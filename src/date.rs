use anyhow::bail;
use rand::random_range;
use serde::{Deserialize, Serialize};

enum MonthName {
    Nuzyael,
    Peonu,
    Kelen,
    Nolus,
    Larane,
    Agrazhar,
    Azura,
    Halane,
    Savor,
    Ilvin,
    Navek,
    Morgat,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: 1,
            month: 1,
            day: 1,
        }
    }
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> anyhow::Result<Self> {
        if month <= 12 && month > 0 && day <= 30 && day > 0 {
            Ok(Date { month, day, year })
        } else {
            bail!("bad date input")
        }
    }

    fn as_ordinal(&self) -> u16 {
        (self.month as u16 - 1) * 30 + self.day as u16
    }
}

#[derive(Debug)]
pub enum Sunsign {
    Ulandus,
    Aralius,
    Feneri,
    Ahnu,
    Angberelius,
    Nadai,
    Hirin,
    Tarael,
    Tai,
    Skorus,
    Masara,
    Lado,
}

impl Sunsign {
    pub fn from_date(date: &Date) -> (Self, Option<Self>) {
        match date.as_ordinal() {
            2..6 => (Self::Lado, Some(Self::Ulandus)),
            6..32 => (Self::Ulandus, None),
            32..36 => (Self::Ulandus, Some(Self::Aralius)),
            36..61 => (Self::Aralius, None),
            61..65 => (Self::Aralius, Some(Self::Feneri)),
            65..92 => (Self::Feneri, None),
            92..96 => (Self::Feneri, Some(Self::Ahnu)),
            96..123 => (Self::Ahnu, None),
            123..127 => (Self::Ahnu, Some(Self::Angberelius)),
            127..155 => (Self::Angberelius, None),
            155..159 => (Self::Angberelius, Some(Self::Nadai)),
            159..184 => (Self::Nadai, None),
            184..188 => (Self::Nadai, Some(Self::Hirin)),
            188..213 => (Self::Hirin, None),
            213..217 => (Self::Hirin, Some(Self::Tarael)),
            217..242 => (Self::Tarael, None),
            242..246 => (Self::Tarael, Some(Self::Tai)),
            246..271 => (Self::Tai, None),
            271..275 => (Self::Tai, Some(Self::Skorus)),
            275..301 => (Self::Skorus, None),
            301..305 => (Self::Skorus, Some(Self::Masara)),
            305..330 => (Self::Masara, None),
            330..334 => (Self::Masara, Some(Self::Lado)),
            334..361 | 1..2 => (Self::Lado, None),
            _ => unreachable!(),
        }
    }

    pub fn gen_trait(&self) -> &str {
        let n = random_range(0..6);
        match self {
            Sunsign::Ulandus => match n {
                0 => "patient",
                1 => "confident",
                2 => "calm",
                3 => "aloof",
                4 => "inflexible",
                5 => "stubborn",
                _ => unreachable!(),
            },
            Sunsign::Aralius => match n {
                0 => "energetic",
                1 => "optimistic",
                2 => "enthusiastic",
                3 => "impulsive",
                4 => "demanding",
                5 => "volatile",
                _ => unreachable!(),
            },
            Sunsign::Feneri => match n {
                0 => "hard-working",
                1 => "productive",
                2 => "restless",
                3 => "impatient",
                4 => "argumentative",
                5 => "obsessive",
                _ => unreachable!(),
            },
            Sunsign::Ahnu => match n {
                0 => "ambitious",
                1 => "experimental",
                2 => "idealistic",
                3 => "perfectionist",
                4 => "critical",
                5 => "quick-tempered",
                _ => unreachable!(),
            },
            Sunsign::Angberelius => match n {
                0 => "lively",
                1 => "impassioned",
                2 => "persistent",
                3 => "blunt",
                4 => "aggressive",
                5 => "destructive",
                _ => unreachable!(),
            },
            Sunsign::Nadai => match n {
                0 => "exuberant",
                1 => "imaginative",
                2 => "spontaneous",
                3 => "unfocused",
                4 => "evasive",
                5 => "wasteful",
                _ => unreachable!(),
            },
            Sunsign::Hirin => match n {
                0 => "free-spirited",
                1 => "instinctive",
                2 => "opportunistic",
                3 => "rebellious",
                4 => "inscrutable",
                5 => "merciless",
                _ => unreachable!(),
            },
            Sunsign::Tarael => match n {
                0 => "dedicated",
                1 => "contemplative",
                2 => "indirect",
                3 => "enigmatic",
                4 => "indecisive",
                5 => "obedient",
                _ => unreachable!(),
            },
            Sunsign::Tai => match n {
                0 => "philosophical",
                1 => "truthful",
                2 => "restrained",
                3 => "unemotional",
                4 => "inconsiderate",
                5 => "naive",
                _ => unreachable!(),
            },
            Sunsign::Skorus => match n {
                0 => "personable",
                1 => "curious",
                2 => "inspiring",
                3 => "avid",
                4 => "intrusive",
                5 => "dictatorial",
                _ => unreachable!(),
            },
            Sunsign::Masara => match n {
                0 => "affectionate",
                1 => "tenacious",
                2 => "emotional",
                3 => "temperamental",
                4 => "resentful",
                5 => "overbearing",
                _ => unreachable!(),
            },
            Sunsign::Lado => match n {
                0 => "composed",
                1 => "tolerant",
                2 => "adaptive",
                3 => "yielding",
                4 => "indulgent",
                5 => "nonchalant",
                _ => unreachable!(),
            },
        }
    }
}
