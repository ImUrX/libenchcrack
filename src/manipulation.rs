use enum_map::Enum;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use std::cmp;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Version {
    V1_8,
    V1_9,
    V1_11,
    V1_11_1,
    V1_13,
    V1_14,
    V1_14_3,
    V1_16
}

impl Version {
    pub fn before(&self, other: Version) -> bool {
        *self < other
    }

    pub fn after(&self, other: Version) -> bool {
        *self > other
    }

    pub fn latest() -> Self {
        Version::V1_16
    }
}

#[wasm_bindgen]
#[derive(AsRefStr, EnumIter)]
pub enum Material {
    Netherite,
    Diamond,
    Golden,
    Iron,
    Chainmail, //not for js
    Fire,
    Turtle, //not for js
    Leather, //not for js
    Stone,
    Wooden
}

pub const SET_MATERIAL: usize = 9;

type SortFn<'r> = &'r dyn Fn(Item) -> bool;
const SORT: &[SortFn; SET_MATERIAL] = &[
    &|x| x.is_helmet(),
    &|x| x.is_chestplate(),
    &|x| x.is_leggings(),
    &|x| x.is_boots(),
    &|x| x.is_sword(),
    &|x| x.is_pickaxe(),
    &|x| x.is_axe(),
    &|x| x.is_shovel(),
    &|x| x.is_hoe()
];

impl Material {
    pub fn get_items(&self) -> [Item; SET_MATERIAL] {
        let mut arr = [Item::Book; SET_MATERIAL];
        for (item, out) in Item::iter().filter(|x| self.has_item(x, false)).zip(arr.iter_mut()) {
            *out = item;
        }
        for (i, func) in SORT.iter().enumerate() {
            for j in i..SET_MATERIAL {
                if func(arr[j]) { arr.swap(i, j) }
            }
        }
        arr
    }

    pub fn has_item(&self, item: &Item, not_js: bool) -> bool {
        let name = item.as_ref();
        if not_js { return name.starts_with(self.as_ref()); }
        match self {
            Self::Fire => Self::Chainmail.has_item(item, true) || (Self::Iron.has_item(item, true) && (item.is_tool() || item.is_sword())),
            Self::Stone => Self::Turtle.has_item(item, true) || (Self::Leather.has_item(item, true) && item.is_armor() && !item.is_helmet()) || name.starts_with(self.as_ref()),
            Self::Wooden => Self::Leather.has_item(item, true) || name.starts_with(self.as_ref()),
            _ => name.starts_with(self.as_ref())
        }
    }
}

// I think the probability of this getting optimized by the compiler is low but who cares
// and maybe im wrong and the compiler is smarter than im (surely it is)
#[wasm_bindgen]
#[derive(AsRefStr, EnumIter, PartialEq, Copy, Clone, Enum, Debug)]
// I tried to do cfg_attr(test, derive(Debug)) but it didn't work?? I misunderstood something for sure...
pub enum Item {
    LeatherHelmet,
    LeatherChestplate,
    LeatherLeggings,
    LeatherBoots,
    IronHelmet,
    IronChestplate,
    IronLeggings,
    IronBoots,
    ChainmailHelmet,
    ChainmailChestplate,
    ChainmailLeggings,
    ChainmailBoots,
    GoldenHelmet,
    GoldenChestplate,
    GoldenLeggings,
    GoldenBoots,
    DiamondHelmet,
    DiamondChestplate,
    DiamondLeggings,
    DiamondBoots,
    WoodenSword,
    StoneSword,
    IronSword,
    GoldenSword,
    DiamondSword,
    WoodenPickaxe,
    StonePickaxe,
    IronPickaxe,
    GoldenPickaxe,
    DiamondPickaxe,
    WoodenAxe,
    StoneAxe,
    IronAxe,
    GoldenAxe,
    DiamondAxe,
    WoodenShovel,
    StoneShovel,
    IronShovel,
    GoldenShovel,
    DiamondShovel,
    WoodenHoe,
    StoneHoe,
    IronHoe,
    GoldenHoe,
    DiamondHoe,
    CarrotOnAStick,
    FishingRod,
    FlintAndSteel,
    Shears,
    Bow,
    Book,
    Pumpkin,
    Skull,
    // 1.9
    Elytra,
    Shield,
    // 1.13
    Trident,
    TurtleHelmet,
    // 1.14
    Crossbow,
    // 1.16
    NetheriteHelmet,
    NetheriteChestplate,
    NetheriteLeggings,
    NetheriteBoots,
    NetheriteSword,
    NetheritePickaxe,
    NetheriteAxe,
    NetheriteShovel,
    NetheriteHoe
}

impl Item {
    pub fn is_helmet(&self) -> bool {
        self.as_ref().ends_with("Helmet")
    }

    pub fn is_chestplate(&self) -> bool {
        self.as_ref().ends_with("Chestplate")
    }

    pub fn is_leggings(&self) -> bool {
        self.as_ref().ends_with("Leggings")
    }
    
    pub fn is_boots(&self) -> bool {
        self.as_ref().ends_with("Boots")
    }

    pub fn is_armor(&self) -> bool {
        self.is_boots() || self.is_chestplate() || self.is_helmet() || self.is_leggings()
    }

    pub fn is_sword(&self) -> bool {
        self.as_ref().ends_with("Sword")
    }

    pub fn is_axe(&self) -> bool {
        self.as_ref().ends_with("Axe")
    }

    pub fn is_pickaxe(&self) -> bool {
        self.as_ref().ends_with("Pickxe")
    }

    pub fn is_shovel(&self) -> bool {
        self.as_ref().ends_with("Shovel")
    }

    pub fn is_hoe(&self) -> bool {
        self.as_ref().ends_with("Hoe")
    }

    pub fn is_tool(&self) -> bool {
        self.is_axe() || self.is_pickaxe() || self.is_shovel() || self.is_hoe()
    }

    pub fn has_durability(&self) -> bool {
        self.is_armor() || self.is_sword() || self.is_tool()
        || [Item::Bow, Item::CarrotOnAStick, Item::Elytra, Item::FishingRod,
        Item::FlintAndSteel, Item::Shears, Item::Shield, Item::Trident, Item::Crossbow].contains(self)
    }

    pub fn get_enchantability(&self) -> i32 {
        if let Some(mat) = self.get_material() {
            if self.is_armor() {
                return match mat {
                    Material::Leather => 15,
                    Material::Iron => 9,
                    Material::Chainmail => 12,
                    Material::Golden => 25,
                    Material::Diamond => 10,
                    Material::Turtle => 9,
                    Material::Netherite => 15,
                    _ => 0
                };
            } else if self.is_sword() || self.is_tool() {
                return match mat {
                    Material::Wooden => 15,
                    Material::Stone => 5,
                    Material::Iron => 14,
                    Material::Golden => 22,
                    Material::Diamond => 10,
                    Material::Netherite => 15,
                    _ => 0
                };
            }
        };
        match self {
            Item::Bow | Item::FishingRod | Item::Trident | Item::Crossbow | Item::Book => 1,
            _ => 0
        }
    }

    pub fn get_introduced_version(&self) -> Version {
        if Material::Netherite.has_item(self, true) { return Version::V1_16; }
        match self {
            Item::Elytra | Item::Shield => Version::V1_9,
            Item::Trident | Item::TurtleHelmet => Version::V1_13,
            Item::Crossbow => Version::V1_14,
            _ => Version::V1_8
        }
    }

    pub fn get_material(&self) -> Option<Material> {
        Material::iter().find(|x| x.has_item(self, true))
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone, EnumIter)]
pub enum Enchantment {
    Protection,
    FireProtection,
    FeatherFalling,
    BlastProtection,
    ProjectileProtection,
    Respiration,
    AquaAffinity,
    Thorns,
    DepthStrider,
    Sharpness,
    Smite,
    BaneOfArthropods,
    Knockback,
    FireAspect,
    Looting,
    Efficiency,
    SilkTouch,
    Unbreaking,
    Fortune,
    Power,
    Punch,
    Flame,
    Infinity,
    LuckOfTheSea,
    Lure,
    // 1.9
    FrostWalker,
    Mending,
    // 1.11
    BindingCurse,
    VanishingCurse,
    // 1.11.1
    Sweeping,
    // 1.13
    Loyalty,
    Impaling,
    Riptide,
    Channeling,
    // 1.14
    Multishot,
    QuickCharge,
    Piercing
}

type IncompatibilityFunc<'r> = &'r dyn Fn(Enchantment, Enchantment, Version) -> bool;

const INCOMPATIBLES: &[IncompatibilityFunc] = &[
    &|a, b, _x| a == b,
    &|a, b, x| x.after(Version::V1_11) && a == Enchantment::Infinity && b == Enchantment::Mending,
    &|a, b, _x| a == Enchantment::Sharpness && b == Enchantment::Smite,
    &|a, b, _x| a == Enchantment::Sharpness && b == Enchantment::BaneOfArthropods,
    &|a, b, _x| a == Enchantment::Smite && b == Enchantment::BaneOfArthropods,
    &|a, b, _x| a == Enchantment::FrostWalker && b == Enchantment::DepthStrider,
    &|a, b, _x| a == Enchantment::SilkTouch && b == Enchantment::Looting,
    &|a, b, _x| a == Enchantment::SilkTouch && b == Enchantment::Fortune,
    &|a, b, _x| a == Enchantment::SilkTouch && b == Enchantment::LuckOfTheSea,
    &|a, b, _x| a == Enchantment::Riptide && b == Enchantment::Loyalty,
    &|a, b, _x| a == Enchantment::Riptide && b == Enchantment::Channeling,
    &|a, b, _x| a == Enchantment::Multishot && b == Enchantment::Piercing,
    &|a, b, x| x != Version::V1_14 && a == Enchantment::Protection && b == Enchantment::FireProtection,
    &|a, b, x| x != Version::V1_14 && a == Enchantment::Protection && b == Enchantment::BlastProtection,
    &|a, b, x| x != Version::V1_14 && a == Enchantment::Protection && b == Enchantment::ProjectileProtection,
    &|a, b, x| x != Version::V1_14 && a == Enchantment::BlastProtection && b == Enchantment::FireProtection,
    &|a, b, x| x != Version::V1_14 && a == Enchantment::BlastProtection && b == Enchantment::ProjectileProtection,
    &|a, b, x| x != Version::V1_14 && a == Enchantment::FireProtection && b == Enchantment::ProjectileProtection
];

impl Enchantment {
    pub fn levels_to_xp(start_level: i32, num_levels: i32) -> i32 {
        let mut amt = 0;
        let end_level = start_level - num_levels + 1; // + 1 because of range
        for level in (end_level..=start_level).rev() {
            if level > 30 {
                amt += (9 * (level - 1)) - 158;
            } else if level > 15 {
                amt += (5 * (level - 1)) - 38;
            } else {
                amt += (2 * (level - 1)) + 7;
            }
        }
        amt
    }

    pub fn can_apply(&self, item: Item, primary: bool) -> bool {
        if item == Item::Book { return true; }
        match self {
            Enchantment::Protection | Enchantment::FireProtection | Enchantment::BlastProtection | Enchantment::ProjectileProtection => item.is_armor(),
            Enchantment::Thorns => if primary { item.is_chestplate() } else { item.is_armor() },
            Enchantment::FeatherFalling | Enchantment::DepthStrider | Enchantment::FrostWalker => item.is_boots(),
            Enchantment::Respiration | Enchantment::AquaAffinity => item.is_helmet(),
            Enchantment::BindingCurse => item.is_armor() || [Item::Pumpkin, Item::Elytra, Item::Skull].iter().any(|x| *x == item),
            Enchantment::Sharpness | Enchantment::Smite | Enchantment::BaneOfArthropods => item.is_sword() || (!primary && item.is_axe()),
            Enchantment::Knockback | Enchantment::FireAspect | Enchantment::Looting | Enchantment::Sweeping => item.is_sword(),
            Enchantment::Efficiency => item.is_tool() || (!primary && item == Item::Shears),
            Enchantment::SilkTouch | Enchantment::Fortune => item.is_tool(),
            Enchantment::Power | Enchantment::Punch | Enchantment::Flame | Enchantment::Infinity => item == Item::Bow,
            Enchantment::LuckOfTheSea | Enchantment::Lure => item == Item::FishingRod,
            Enchantment::Unbreaking | Enchantment::Mending => item.has_durability(),
            Enchantment::VanishingCurse => item.has_durability() || item == Item::Pumpkin || item == Item::Skull,
            Enchantment::Loyalty | Enchantment::Impaling | Enchantment::Riptide | Enchantment::Channeling => item == Item::Trident,
            Enchantment::Multishot | Enchantment::QuickCharge | Enchantment::Piercing => item == Item::Crossbow,
        }
    }

    pub fn is_treasure(&self) -> bool {
        [Enchantment::FrostWalker, Enchantment::Mending,
        Enchantment::BindingCurse, Enchantment::VanishingCurse].iter().any(|x| x == self)
    }

    pub fn get_max_level(&self) -> i32 {
        match self {
            Enchantment::Sharpness | Enchantment::Smite | Enchantment::BaneOfArthropods | Enchantment::Efficiency
            | Enchantment::Power | Enchantment::Impaling => 5,
            Enchantment::Protection | Enchantment::FireProtection | Enchantment::BlastProtection | Enchantment::ProjectileProtection
            | Enchantment::FeatherFalling | Enchantment::Piercing => 4,
            Enchantment::Thorns | Enchantment::DepthStrider | Enchantment::Respiration | Enchantment::Looting
            | Enchantment::Sweeping | Enchantment::Fortune | Enchantment::LuckOfTheSea | Enchantment::Lure
            | Enchantment::Unbreaking | Enchantment::Loyalty | Enchantment::Riptide | Enchantment::QuickCharge => 3,
            Enchantment::FrostWalker | Enchantment::Knockback | Enchantment::FireAspect | Enchantment::Punch => 2,
            Enchantment::AquaAffinity | Enchantment::BindingCurse | Enchantment::SilkTouch | Enchantment::Flame
            | Enchantment::Infinity | Enchantment::Mending | Enchantment::VanishingCurse | Enchantment::Channeling
            | Enchantment::Multishot => 1
        }
    }

    pub fn get_min_enchantability(&self, level: i32) -> i32 {
        match self {
            Enchantment::Protection => 1 + (level - 1) * 11,
            Enchantment::FireProtection => 10 + (level - 1) * 8,
            Enchantment::FeatherFalling => 5 + (level - 1) * 6,
            Enchantment::BlastProtection => 5 + (level - 1) * 8,
            Enchantment::ProjectileProtection => 3 + (level - 1) * 6,
            Enchantment::Respiration => level * 10,
            Enchantment::AquaAffinity => 1,
            Enchantment::Thorns => 10 + (level - 1) * 20,
            Enchantment::DepthStrider => level * 10,
            Enchantment::FrostWalker => level * 10,
            Enchantment::BindingCurse => 25,
            Enchantment::Sharpness => 1 + (level - 1) * 11,
            Enchantment::Smite => 5 + (level - 1) * 8,
            Enchantment::BaneOfArthropods => 5 + (level - 1) * 8,
            Enchantment::Knockback => 5 + (level - 1) * 20,
            Enchantment::FireAspect => 10 + (level - 1) * 20,
            Enchantment::Looting => 15 + (level - 1) * 9,
            Enchantment::Sweeping => 5 + (level - 1) * 9,
            Enchantment::Efficiency => 1 + (level - 1) * 10,
            Enchantment::SilkTouch => 15,
            Enchantment::Unbreaking => 5 + (level - 1) * 8,
            Enchantment::Fortune => 15 + (level - 1) * 9,
            Enchantment::Power => 1 + (level - 1) * 10,
            Enchantment::Punch => 12 + (level - 1) * 20,
            Enchantment::Flame => 20,
            Enchantment::Infinity => 20,
            Enchantment::LuckOfTheSea => 15 + (level - 1) * 9,
            Enchantment::Lure => 15 + (level - 1) * 9,
            Enchantment::Mending => 25,
            Enchantment::VanishingCurse => 25,
            Enchantment::Loyalty => 5 + level * 7,
            Enchantment::Impaling => 1 + (level - 1) * 8,
            Enchantment::Riptide => 10 + level * 7,
            Enchantment::Channeling => 25,
            Enchantment::Multishot => 20,
            Enchantment::QuickCharge => 12 + (level - 1) * 20,
            Enchantment::Piercing => 1 + (level - 1) * 10
        }
    }

    pub fn get_max_enchantability(&self, level: i32) -> i32 {
        match self {
            Enchantment::Protection => 1 + level * 11,
            Enchantment::FireProtection => 10 + level * 8,
            Enchantment::FeatherFalling => 5 + level * 6,
            Enchantment::BlastProtection => 5 + level * 8,
            Enchantment::ProjectileProtection => 3 + level * 6,
            Enchantment::Respiration => 30 + level * 10,
            Enchantment::AquaAffinity => 41,
            Enchantment::Thorns => 40 + level * 20,
            Enchantment::DepthStrider => 15 + level * 10,
            Enchantment::FrostWalker => 15 + level * 10,
            Enchantment::BindingCurse => 50,
            Enchantment::Sharpness => 21 + (level - 1) * 11,
            Enchantment::Smite => 25 + (level - 1) * 8,
            Enchantment::BaneOfArthropods => 25 + (level - 1) * 8,
            Enchantment::Knockback => 55 + (level - 1) * 20,
            Enchantment::FireAspect => 40 + level * 20,
            Enchantment::Looting => 65 + (level - 1) * 9,
            Enchantment::Sweeping => 20 + (level - 1) * 9,
            Enchantment::Efficiency => 50 + level * 10,
            Enchantment::SilkTouch => 65,
            Enchantment::Unbreaking => 55 + (level - 1) * 8,
            Enchantment::Fortune => 65 + (level - 1) * 9,
            Enchantment::Power => 16 + (level - 1) * 10,
            Enchantment::Punch => 37 + (level - 1) * 20,
            Enchantment::Flame => 50,
            Enchantment::Infinity => 50,
            Enchantment::LuckOfTheSea => 65 + (level - 1) * 9,
            Enchantment::Lure => 65 + (level - 1) * 9,
            Enchantment::Mending => 75,
            Enchantment::VanishingCurse => 50,
            Enchantment::Loyalty => 50,
            Enchantment::Impaling => 21 + (level - 1) * 8,
            Enchantment::Riptide => 50,
            Enchantment::Channeling => 50,
            Enchantment::Multishot => 50,
            Enchantment::QuickCharge => 50,
            Enchantment::Piercing => 50
        }
    }

    pub fn get_weight(&self, version: Version) -> i32 {
        match self {
            Enchantment::Protection | Enchantment::Sharpness | Enchantment::Efficiency | Enchantment::Power
            | Enchantment::Piercing => if version == Version::V1_14 { 30 } else { 10 },
            Enchantment::FireProtection | Enchantment::FeatherFalling | Enchantment::ProjectileProtection
            | Enchantment::Smite | Enchantment::BaneOfArthropods | Enchantment::Knockback | Enchantment::Unbreaking
            | Enchantment::Loyalty | Enchantment::QuickCharge => if version == Version::V1_14 { 10 } else { 5 },
            Enchantment::BlastProtection | Enchantment::Respiration | Enchantment::AquaAffinity | Enchantment::DepthStrider
            | Enchantment::FrostWalker | Enchantment::FireAspect | Enchantment::Looting | Enchantment::Sweeping | Enchantment::Fortune 
            | Enchantment::Punch | Enchantment::Flame | Enchantment::LuckOfTheSea | Enchantment::Lure | Enchantment::Mending | Enchantment::Impaling 
            | Enchantment::Riptide | Enchantment::Multishot => if version == Version::V1_14 { 3 } else { 2 },
            Enchantment::Thorns | Enchantment::BindingCurse | Enchantment::SilkTouch | Enchantment::Infinity
            | Enchantment::VanishingCurse | Enchantment::Channeling => 1
        }
    }

    pub fn get_introduced_version(&self) -> Version {
        match self {
            Enchantment::FrostWalker | Enchantment::Mending => Version::V1_9,
            Enchantment::BindingCurse | Enchantment::VanishingCurse => Version::V1_11,
            Enchantment::Sweeping => Version::V1_11_1,
            Enchantment::Loyalty | Enchantment::Impaling | Enchantment::Riptide | Enchantment::Channeling => Version::V1_13,
            Enchantment::Multishot | Enchantment::QuickCharge | Enchantment::Piercing => Version::V1_14,
            _ => Version::V1_8
        }
    }

    pub fn get_max_level_in_table(&self, item: Item) -> i32 {
        let enchantability = item.get_enchantability();
        if enchantability == 0 || self.is_treasure() || !self.can_apply(item, true) {
            return 0;
        }
        let mut level = 30 + 1 + enchantability/4 + enchantability/4;
        level += ((level as f32) * 0.15).round() as i32;
        for max_level in (1..=self.get_max_level()).rev() {
            if level >= self.get_min_enchantability(max_level) {
                return max_level;
            }
        }
        0
    }

    pub fn is_compatible_with(&self, ench: Enchantment, version: Version) -> bool {
        !INCOMPATIBLES.iter().any(|func| func(*self, ench, version) || func(ench, *self, version) )
    }

    pub fn calc_enchantment_table_level(rand: &mut java_rand::Random, slot: i32, bookshelves: i32, item: Item) -> i32 {
        if item.get_enchantability() == 0 {
            return 0;
        }
        let level = rand.next_i32_bound(8) + 1 + (bookshelves >> 1) + rand.next_i32_bound(bookshelves + 1);
        match slot {
            0 => cmp::max(level / 3, 1),
            1 => level * 2 / 3 + 1,
            2 => cmp::max(level, bookshelves * 2),
            _ => panic!("More than 3 enchantment slots?")
        }
    }

    pub fn get_highest_allowed_enchantments(level: i32, item: Item, treasure: bool, version: Version) -> Vec<EnchantmentInstance> {
        let mut allowed_enchs = Vec::new();
        
        if version.before(item.get_introduced_version()) {
            return allowed_enchs;
        }

        for ench in Enchantment::iter() {
            if version.before(ench.get_introduced_version()) {
                continue;
            }

            if (treasure || !ench.is_treasure()) && ench.can_apply(item, true) {
                for ench_lvl in (1..=ench.get_max_level()).rev() {
                    if level >= ench.get_min_enchantability(ench_lvl) && level <= ench.get_max_enchantability(ench_lvl) {
                        allowed_enchs.push(EnchantmentInstance::new(ench, ench_lvl));
                        break;
                    }
                }
            }
        }
        allowed_enchs
    }

    pub fn add_random_enchantments(rand: &mut java_rand::Random, item: Item, level: i32, treasure: bool, version: Version) -> Vec<EnchantmentInstance> {
        let enchantability = item.get_enchantability();
        let mut level = level;
        let mut enchs = Vec::new();
        if enchantability <= 0 {
            return enchs;
        }

        level += 1 + rand.next_i32_bound(enchantability/4 + 1) + rand.next_i32_bound(enchantability/4 + 1);
        let percent_change: f32 = (rand.next_f32() + rand.next_f32() - 1f32) * 0.15;
        level += (level as f32 * percent_change).round() as i32;
        if level < 1 {
            level = 1;
        }

        let mut allowed_enchs = Self::get_highest_allowed_enchantments(level, item, treasure, version);
        if allowed_enchs.is_empty() {
            return enchs;
        }

        {
            let ench = Self::weighted_random(rand, &mut allowed_enchs, &|x| x.enchantment.get_weight(version));
            if ench.is_some() {
                enchs.push(ench.unwrap());
            }
        }

        while rand.next_i32_bound(50) <= level {
            if version == Version::V1_14 {
                level = level * 4 / 5 + 1;
                allowed_enchs = Self::get_highest_allowed_enchantments(level, item, treasure, version);
            }

            for ench in enchs.iter() {
                let enchantment = ench.enchantment;
                allowed_enchs.retain(|x| x.enchantment.is_compatible_with(enchantment, version));
            }

            if allowed_enchs.is_empty() {
                break;
            }

            let ench = Self::weighted_random(rand, &mut allowed_enchs, &|x| x.enchantment.get_weight(version));
            if ench.is_some() {
                enchs.push(ench.unwrap());
            }

            level /= 2;
        }
        enchs
    }

    pub fn get_enchantments_in_table(rand: &mut java_rand::Random, xp_seed: i32, item: Item, slot: i32, levels: i32, version: Version) -> Vec<EnchantmentInstance> {
        rand.set_seed(xp_seed as u64 + slot as u64);
        let mut v = Self::add_random_enchantments(rand, item, levels, false, version);
        if Item::Book == item && v.len() > 1 {
            v.remove(rand.next_i32_bound(v.len() as i32) as usize);
        }
        v
    }

    fn weighted_random<T>(rand: &mut java_rand::Random, v: &mut Vec<T>, weight_extractor: &dyn Fn(&T) -> i32) -> Option<T> {
        let mut weight = v.iter().map(|x| weight_extractor(x)).sum();
        if weight <= 0 {
            return None;
        }
        weight = rand.next_i32_bound(weight);

        let index = v.iter().position(|x| {
            weight -= weight_extractor(x);
            return weight < 0;
        });

        if index.is_none() {
            return None;
        }
        Some(v.remove(index.unwrap()))
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone)]
pub struct EnchantmentInstance {
    pub enchantment: Enchantment,
    pub level: i32
}

#[wasm_bindgen]
impl EnchantmentInstance {
    #[wasm_bindgen(constructor)]
    pub fn new(enchantment: Enchantment, level: i32) -> Self {
        EnchantmentInstance {
            enchantment, level
        }
    }
}
