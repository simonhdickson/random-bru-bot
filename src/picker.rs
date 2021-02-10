use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct Person {
    pub name: String,
    weightings: HashMap<String, usize>,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Person) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Person {
    fn new(name: String) -> Person {
        Self {
            name,
            weightings: HashMap::new(),
        }
    }

    fn get_weighting(&self, person: &Person) -> usize {
        *self
            .weightings
            .get(&person.name)
            .unwrap_or_else(|| &usize::MAX)
    }

    fn update_weighting(&mut self, people: Vec<Person>) {
        for p in people.iter() {
            if !self.weightings.contains_key(&p.name) {
                self.weightings.insert(p.name.to_owned(), 0);
            }
        }

        for (p, val) in self.weightings.iter_mut() {
            if people.contains(&Person::new(p.to_owned())) {
                *val = 0;
            } else {
                *val += 1;
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonSet {
    previous_dealer: Option<Person>,
    people: BTreeSet<Person>,
}

impl PersonSet {
    pub fn new() -> Self {
        Self {
            previous_dealer: None,
            people: BTreeSet::new(),
        }
    }

    pub fn add_person(&mut self, person: String) {
        self.people.insert(Person::new(person));
    }

    fn get_next_picker(&self, last_pick: &Option<Person>) -> Option<Person> {
        match last_pick {
            Some(last_pick) => {
                let candidate = self
                    .people
                    .iter()
                    .skip_while(|p| p != &last_pick)
                    .skip(1)
                    .next()
                    .cloned();

                candidate.or_else(|| self.people.first().cloned())
            }
            None => self.people.first().cloned(),
        }
    }

    pub fn apply_picks(&mut self, picks: &Vec<Pick>, dealer: Option<Person>) {
        self.previous_dealer = dealer;

        for pick in picks.iter() {
            match pick {
                Pick::Pair(a, b) => {
                    let mut a = self.people.take(a).unwrap();
                    let mut b = self.people.take(b).unwrap();
                    a.update_weighting(vec![b.clone()]);
                    b.update_weighting(vec![a.clone()]);
                    self.people.insert(a);
                    self.people.insert(b);
                }
                Pick::Triple(a, b, c) => {
                    let mut a = self.people.take(a).unwrap();
                    let mut b = self.people.take(b).unwrap();
                    let mut c = self.people.take(c).unwrap();
                    a.update_weighting(vec![b.clone(), c.clone()]);
                    b.update_weighting(vec![a.clone(), c.clone()]);
                    c.update_weighting(vec![a.clone(), b.clone()]);
                    self.people.insert(a);
                    self.people.insert(b);
                    self.people.insert(c);
                }
            }
        }
    }

    pub fn make_selection(mut self) -> (Vec<Pick>, PersonSet) {
        let mut result = self.clone();

        let mut picks = Vec::new();

        let mut last_picker = self.previous_dealer.clone();

        let next_dealer = self.get_next_picker(&last_picker);

        while !self.people.is_empty() {
            if self.people.len() == 3 {
                let mut x = self.people.into_iter();

                picks.push(Pick::Triple(
                    x.next().unwrap(),
                    x.next().unwrap(),
                    x.next().unwrap(),
                ));

                break;
            }

            if let Some(person) = self.get_next_picker(&last_picker) {
                let person_match = self.people.iter().max_by(|x, y| x.cmp(y)).unwrap().clone();

                self.people.remove(&person);
                self.people.remove(&person_match);
                last_picker = Some(person.clone());

                picks.push(Pick::Pair(person, person_match));
            } else {
                panic!("wat")
            }
        }

        result.apply_picks(&picks, next_dealer);

        (picks, result)
    }
}

#[derive(Debug)]
pub enum Pick {
    Pair(Person, Person),
    Triple(Person, Person, Person),
}

impl Pick {
    fn contains(&self, person: &Person) -> bool {
        match self {
            Self::Pair(a, b) => a == person || b == person,
            Self::Triple(a, b, c) => a == person || b == person || c == person,
        }
    }
}

#[test]
fn test_bad_add() {
    let mut people = PersonSet::new();
    people.add_person("alice".to_owned());
    people.add_person("bob".to_owned());
    people.add_person("chuck".to_owned());
    people.add_person("dave".to_owned());

    let (first_round, mut people) = people.make_selection();

    println!("first round: {:#?}", first_round);

    people.add_person("edward".to_owned());

    let (second_round, mut people) = people.make_selection();

    println!("second round: {:#?}", second_round);

    people.add_person("fred".to_owned());

    let (third_round, mut _people) = people.make_selection();

    println!("third round: {:#?}", third_round);

    panic!("wat");
}
