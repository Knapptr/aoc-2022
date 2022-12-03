use std::collections::HashMap;

#[derive(Clone)]
pub struct Item {
    char: char,
}
impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.char == other.char
    }
}
impl Item {
    fn from_char(char: char) -> Item {
        if !char.is_ascii_alphabetic() {
            panic!()
        }
        Item { char }
    }
    pub fn get_priority(&self) -> u32 {
        if let true = self.char.is_uppercase() {
            self.char as u32 - 38
        } else {
            self.char as u32 - 96
        }
    }
}
pub struct Sack {
    contents: Vec<Item>,
}
impl Sack {
    pub fn from_string(str: &str) -> Sack {
        let chars: Vec<Item> = str.chars().map(|ch| Item::from_char(ch)).collect();
        Sack { contents: chars }
    }
    pub fn get_compartments_as_sacks(&self) -> (Sack, Sack) {
        let midpoint = self.contents.len() / 2;
        //split at midpoint
        let left = Sack {
            contents: Vec::from(&self.contents[0..midpoint]),
        };
        let right = Sack {
            contents: Vec::from(&self.contents[midpoint..]),
        };
        //return array of halves
        (left, right)
    }
    pub fn get_first_common_item_with<'a>(&'a self, other_sack: &'a Sack) -> &Item {
        let mut map_1 = HashMap::new();
        for item in &self.contents {
            map_1.insert(item.char, true);
        }
        for item in &other_sack.contents {
            match map_1.get(&item.char) {
                Some(_) => return item,
                None => (),
            }
        }
        unreachable!()
    }
    fn get_common_sack(&self, other_sack: &Sack) -> Sack {
        let mut in_common: Vec<Item> = Vec::new();
        let mut map_1 = HashMap::new();
        for item in &self.contents {
            map_1.insert(item.char, true);
        }
        for item in &other_sack.contents {
            match map_1.get(&item.char) {
                Some(_) => in_common.push(item.clone()),
                None => (),
            }
        }
        Sack {
            contents: in_common,
        }
    }

    pub fn get_badge_from(&self, sack2: &Sack, sack3: &Sack) -> Item {
        let common = self.get_common_sack(sack2);
        common.get_first_common_item_with(&sack3).clone()
    }
}
