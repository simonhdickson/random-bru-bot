use std::{cmp::Ordering, collections::{BTreeMap, BTreeSet}};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Person {
    name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Weighting(usize, Person);

impl Ord for Weighting {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
            .then_with(|| self.1.cmp(&other.1))
    }
}

impl PartialOrd for Weighting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
pub struct PeopleWeighting(BTreeMap<Person, BTreeSet<Weighting>>);

impl PeopleWeighting {
    pub fn new() -> PeopleWeighting {
        PeopleWeighting(BTreeMap::new())
    }

    pub fn push(&mut self, person: Person) {
        let mut value = BTreeSet::new();

        for (p, heap) in self.0.iter_mut() {
            heap.insert(Weighting(0, person.clone()));
            value.insert(Weighting(0, p.clone()));
        }

        self.0.insert(person, value);
    }

    pub fn apply_picks(&mut self, picks: Vec<Pick>) {
        for pick in picks {
            match pick {
                Pick::Pair(a, b) => {
                    self.increment_weighting(a, b);
                },
                Pick::Triple(a, b, c) => {
                    self.increment_weighting(a.clone(), b.clone());
                    self.increment_weighting(a, c.clone());
                    self.increment_weighting(b.clone(), c);

                },
            }
        }
    }

    pub fn increment_weighting(&mut self, person_a: Person, person_b: Person) {
        self.increment(person_a.clone(), person_b.clone());
        self.increment(person_b, person_a);
    }

    fn increment(&mut self, person_a: Person, person_b: Person) {
        let mut weight_a = self.0.get_mut(&person_a).unwrap();
        let Weighting(count,_) = weight_a.iter().filter(|Weighting(_,p)| p == &person_b).next().unwrap().clone();
        weight_a.remove(&Weighting(count, person_b.clone()));
        weight_a.insert(Weighting(count+1, person_b.clone()));
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

fn make_selection(mut people: PeopleWeighting) -> Vec<Pick> {
    let mut result = Vec::new();

    while !people.0.is_empty() {
        if people.0.len() == 3 {
            result.push(Pick::Triple(
                people.0.pop_first().unwrap().0,
                people.0.pop_first().unwrap().0,
                people.0.pop_first().unwrap().0,
            ));
            
            break;
        }

        if let Some((person, weightings)) = people.0.pop_first() {
            if let Some(next) = weightings
                .iter()
                .filter(|w| !result.iter().any(|p: &Pick| p.contains(&w.1)))
                .next()
            {
                people.0.remove(&person);
                people.0.remove(&next.1);

                result.push(Pick::Pair(person, next.1.clone()));
            } else {
                panic!("wat")
            }
        } else {
            panic!("wat")
        }
    }

    result
}

#[test]
fn test_bad_add() {
    let mut people = PeopleWeighting::new();
    people.push(Person { name: "alice".to_owned() });
    people.push(Person { name: "bob".to_owned() });
    people.push(Person { name: "chuck".to_owned() });
    people.push(Person { name: "dave".to_owned() });

    let first_round = make_selection(people.clone());

    println!("first round: {:#?}", first_round);

    people.apply_picks(first_round);

    people.push(Person { name: "edward".to_owned() });

    let second_round = make_selection(people.clone());

    println!("second round: {:#?}", second_round);

    people.apply_picks(second_round);

    people.push(Person { name: "fred".to_owned() });

    let third_round = make_selection(people.clone());

    println!("third round: {:#?}", third_round);

    people.apply_picks(third_round);
    
    println!("{:#?}", people);

    panic!("wat");
}
