use crate::item::{Item, ProjectileHead, ProjectileShaft};
use anyhow_serde::Result;
use std::{collections::HashMap, fs::File, io::Read};
use strum::IntoEnumIterator;

pub enum Suit {
    Clothing,
    HeavyClothing,
    QuiltedCoat,
    KurbulCuirass,
    ScaleByrnie,
    MailByrnie,
    Gambeson,
    ScaleHabergeon,
    MailHabergeon,
    MailHauberk,
    KurbulMail,
    PlateMail,
}

impl Suit {
    fn files(&self) -> Vec<String> {
        match self {
            Suit::Clothing => vec![
                "items/armor/cloth_vest.toml".to_string(),
                "items/armor/cloth_tunic.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/cloth_swaddle.toml".to_string(),
            ],
            Suit::HeavyClothing => vec![
                "items/armor/leather_cap.toml".to_string(),
                "items/armor/padded_shirt.toml".to_string(),
                "items/armor/cloth_tunic_sleeved.toml".to_string(),
                "items/armor/cloth_surcoat.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/cloth_swaddle.toml".to_string(),
                "items/armor/leather_shoes.toml".to_string(),
            ],
            Suit::QuiltedCoat => vec![
                "items/armor/kurbul_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/quilted_coat.toml".to_string(),
                "items/armor/cloth_tunic.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/cloth_leggings.toml".to_string(),
                "items/armor/leather_shoes.toml".to_string(),
            ],
            Suit::KurbulCuirass => vec![
                "items/armor/plate_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/kurbul_spaulders.toml".to_string(),
                "items/armor/kurbul_rerebraces.toml".to_string(),
                "items/armor/kurbul_cuirass.toml".to_string(),
                "items/armor/padded_tunic_sleeved.toml".to_string(),
                "items/armor/cloth_trousers.toml".to_string(),
                "items/armor/cloth_swaddle.toml".to_string(),
                "items/armor/leather_boots.toml".to_string(),
            ],
            Suit::ScaleByrnie => vec![
                "items/armor/plate_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/scale_byrnie.toml".to_string(),
                "items/armor/padded_coat.toml".to_string(),
                "items/armor/cloth_trousers.toml".to_string(),
                "items/armor/cloth_swaddle.toml".to_string(),
                "items/armor/leather_boots.toml".to_string(),
            ],
            Suit::MailByrnie => vec![
                "items/armor/plate_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/mail_byrnie.toml".to_string(),
                "items/armor/padded_tunic.toml".to_string(),
                "items/armor/cloth_trousers.toml".to_string(),
                "items/armor/cloth_swaddle.toml".to_string(),
                "items/armor/leather_boots.toml".to_string(),
            ],
            Suit::Gambeson => vec![
                "items/armor/plate_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/gambeson_coat.toml".to_string(),
                "items/armor/cloth_vest.toml".to_string(),
                "items/armor/kurbul_kneecops.toml".to_string(),
                "items/armor/padded_cuisses.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/cloth_swaddle.toml".to_string(),
                "items/armor/leather_boots.toml".to_string(),
            ],
            Suit::ScaleHabergeon => vec![
                "items/armor/plate_34_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/kurbul_spaulders.toml".to_string(),
                "items/armor/kurbul_coudes.toml".to_string(),
                "items/armor/leather_gauntlets.toml".to_string(),
                "items/armor/scale_habergeon.toml".to_string(),
                "items/armor/padded_coat.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/kurbul_kneecops.toml".to_string(),
                "items/armor/padded_leggings.toml".to_string(),
                "items/armor/leather_boots.toml".to_string(),
            ],
            Suit::MailHabergeon => vec![
                "items/armor/plate_helm.toml".to_string(),
                "items/armor/padded_cap.toml".to_string(),
                "items/armor/mail_habergeon.toml".to_string(),
                "items/armor/padded_coat.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/kurbul_kneecops.toml".to_string(),
                "items/armor/padded_leggings.toml".to_string(),
                "items/armor/leather_boots.toml".to_string(),
            ],
            Suit::MailHauberk => vec![
                "items/armor/mail_cowl.toml".to_string(),
                "items/armor/padded_cowl.toml".to_string(),
                "items/armor/plate_coudes.toml".to_string(),
                "items/armor/mail_mittens.toml".to_string(),
                "items/armor/padded_mittens.toml".to_string(),
                "items/armor/mail_hauberk.toml".to_string(),
                "items/armor/padded_coat.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/plate_kneecops.toml".to_string(),
                "items/armor/mail_leggings.toml".to_string(),
                "items/armor/padded_leggings.toml".to_string(),
            ],
            Suit::KurbulMail => vec![
                "items/armor/plate_34_helm.toml".to_string(),
                "items/armor/mail_cowl.toml".to_string(),
                "items/armor/padded_cowl.toml".to_string(),
                "items/armor/kurbul_spaulders.toml".to_string(),
                "items/armor/kurbul_rerebraces.toml".to_string(),
                "items/armor/kurbul_coudes.toml".to_string(),
                "items/armor/mail_mittens.toml".to_string(),
                "items/armor/padded_mittens.toml".to_string(),
                "items/armor/kurbul_cuirass.toml".to_string(),
                "items/armor/mail_hauberk.toml".to_string(),
                "items/armor/padded_coat.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/kurbul_kneecops.toml".to_string(),
                "items/armor/plate_greaves.toml".to_string(),
                "items/armor/mail_leggings.toml".to_string(),
                "items/armor/padded_leggings.toml".to_string(),
            ],
            Suit::PlateMail => vec![
                "items/armor/plate_great_helm.toml".to_string(),
                "items/armor/mail_cowl.toml".to_string(),
                "items/armor/padded_cowl.toml".to_string(),
                "items/armor/plate_spaulders.toml".to_string(),
                "items/armor/plate_rerebraces.toml".to_string(),
                "items/armor/plate_coudes.toml".to_string(),
                "items/armor/plate_vambraces.toml".to_string(),
                "items/armor/mail_mittens.toml".to_string(),
                "items/armor/padded_mittens.toml".to_string(),
                "items/armor/plate_cuirass.toml".to_string(),
                "items/armor/mail_hauberk.toml".to_string(),
                "items/armor/padded_coat.toml".to_string(),
                "items/armor/cloth_breeches.toml".to_string(),
                "items/armor/plate_kneecops.toml".to_string(),
                "items/armor/plate_greaves.toml".to_string(),
                "items/armor/mail_leggings.toml".to_string(),
                "items/armor/padded_leggings.toml".to_string(),
            ],
        }
    }

    pub fn load(&self) -> Vec<Item> {
        self.files()
            .iter()
            .map(|x| item::from_file(x).unwrap())
            .collect()
    }
}

pub fn load_melee_weapon<P>(path: P) -> Result<Item>
where
    P: AsRef<std::path::Path>,
{
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    let o: Item = toml::from_slice(&buf)?;
    Ok(o)
}

pub fn load_range_weapon<P>(path: P) -> Result<RangeWeapon>
where
    P: AsRef<std::path::Path>,
{
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    let o: RangeWeapon = toml::from_slice(&buf)?;
    Ok(o)
}

pub fn load_armor<P>(path: P) -> Result<Armor>
where
    P: AsRef<std::path::Path>,
{
    let mut buf = Vec::new();
    File::open(path)?.read_to_end(&mut buf)?;
    let o: Armor = toml::from_slice(&buf)?;
    Ok(o)
}

pub fn projectiles() -> HashMap<String, Projectile> {
    let mut map = HashMap::new();
    weapons::ProjectileHead::iter().for_each(|head| {
        weapons::ProjectileShaft::iter().for_each(|shaft| {
            map.insert(
                format!("{shaft:?} {head:?}"),
                weapons::Projectile { shaft, head },
            );
        })
    });
    map
}
