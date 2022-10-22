use rand::seq::SliceRandom;
use rand::{self, Rng};
/// 矮人族
#[derive(Debug)]
struct Dwarf {}

/// 精灵族
#[derive(Debug)]
struct Elf {}

/// 人类
#[derive(Debug)]
struct Human {}

/// 角色
#[derive(Debug)]
enum Thing {
    /// 刀
    Sword,
    /// 小饰品
    Trinket,
}

/// 施法者
trait Enchanter: std::fmt::Debug {
    /// 资格、能力
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);

        print!("{:?} mutters incoherently. ", self);
        if spell_is_successful {
            println!("The {:?} glows brightly.", thing);
        } else {
            println!(
                "The {:?} fizzes, \
            then truns into a worthless trinket.",
                thing
            );
            *thing = Thing::Trinket;
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

fn main() {
    let mut it = Thing::Sword;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

    spellcaster.enchant(&mut it);
}
