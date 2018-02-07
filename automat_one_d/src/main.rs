use std::fmt::{self, Formatter, Display};
use std::collections::HashMap;

fn add_chars(how_many: usize, goal: String, c: char) -> String{
    (0..how_many).map(|_| c.to_string()).collect::<String>() + &goal
}

fn add_zeroes(how_many: usize, goal: String) -> String{
    add_chars(how_many, goal, '0')
}

pub struct Automaton {
    pub generations: Vec<Ring>,
    pub rule: usize,
    pub rules: HashMap<String, char>,
    pub neighborhood: usize
}

pub struct Ring {
    pub configuration: Vec<char>,
}

impl Automaton {
    pub fn new(base: &Vec<char>, rule: usize, neighborhood: usize) -> Automaton{
        let base_ring = Ring::from(base);
        let mut generations = Vec::new();
        let rules = HashMap::new();
        generations.push(base_ring);

        let mut a = Automaton{
            generations,
            rule,
            rules,
            neighborhood
        };
        a.populate_rules();
        a
    }

    /*
    Produit les règles
     */
    fn populate_rules(&mut self){
        let boundary = usize::pow(2, self.neighborhood as u32);
        let mut rule_bin = {
            let bin = format!("{:b}", self.rule);
            let dif = boundary - bin.len();
            add_zeroes(dif, bin)
        };
        for i in 0..boundary {
            let neighbors = {
                let neighbors_string = format!("{:b}", i);
                add_zeroes(self.neighborhood - neighbors_string.len(), neighbors_string)
            };
            self.rules.insert(neighbors, rule_bin.pop().unwrap());
        }
    }

    /*
    Produit la génération suivante
     */
    pub fn next_gen(&mut self){

        let mut next_gen = Ring::new();
        {
            let last_gen = self.get_last();
            for i in 0..last_gen.size(){
                let dif = self.neighborhood as i32 / 2;
                let range_inf = if self.neighborhood%2==0 {(i-dif+1..i)} else {(i-dif..i)};
                let range_sup = i..i+dif+1;
                let current = range_inf.map(|j| last_gen.get(j).to_string()).collect::<String>() +&range_sup.map(|j| last_gen.get(j).to_string()).collect::<String>();

                let result = self.rules.get(&current).expect(&format!("This is not supposed to happen : no rules exist for this configuration: {}", current));
                next_gen.push(result);
            }
        }
        self.generations.push(next_gen);
    }

    pub fn get_last(&self) -> &Ring{
        self.generations.last().unwrap()
    }

    pub fn state_already_generated(&self) -> bool{
        if self.generations.len()>1 {
            for i in 0..self.generations.len()-1 {
                let ring = self.generations.get(i).unwrap();
                if self.get_last() == ring {
                    return true
                }
            }
        }
        false
    }

    pub fn is_00000001(&self) -> bool{
        let gens = self.generations.len();
        if self.generations.len()>1 {
            let last = self.get_last();
            last.is_00000001_shifting_from(self.generations.get(gens-2).unwrap())
        } else {
            false
        }
    }
}

impl Display for Automaton {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut pretty_string = String::new();
        pretty_string.push_str(&format!("Rule : {}\n", self.rule));
        pretty_string.push_str(" | ");
        let boundary = usize::pow(2, self.neighborhood as u32);
        let rule_bin = {
            let bin = format!("{:b}", self.rule);
            let dif = boundary - bin.len();
            add_zeroes(dif, bin)
        };
        for i in (0..boundary).rev() {
            let neighbors = {
                let neighbors_string = format!("{:b}", i);
                add_zeroes(self.neighborhood - neighbors_string.len(), neighbors_string)
            };
            pretty_string.push_str(&neighbors);
            pretty_string.push_str(" | ");
        }
        pretty_string.push_str("\n | ");
        for c in rule_bin.chars() {
            let result = {
                let dif = self.neighborhood / 2;
                if self.neighborhood%2==0 {
                    add_chars(dif-1, c.to_string(), ' ') + &add_chars(dif-1, ' '.to_string(), ' ')
                } else {
                    add_chars(dif, c.to_string(), ' ') + &add_chars(dif-1, ' '.to_string(), ' ')
                }
            };
            pretty_string.push_str(&result);
            pretty_string.push_str(" | ");
        }

        pretty_string.push_str("\n\ngenerations:\n");
        for g in &self.generations{
            pretty_string.push_str(&format!("{}\n", g));
        }

        write!(f, "{}",pretty_string)
    }
}


impl Ring {
    pub fn new() -> Ring{
        Ring{
            configuration: Vec::new()
        }
    }

    pub fn from(base: &Vec<char>) -> Ring{
        Ring{
            configuration: base.to_vec()
        }
    }

    pub fn push(&mut self, element: &char){
        self.configuration.push(*element);
    }

    pub fn get(&self, index: i32) -> &char{
        self.configuration.get(self.index_correction(index)).unwrap()
    }

    fn index_correction(&self, index: i32) -> usize {
        if index>=0 && index<self.size() {
            index as usize
        } else {
            if index < 0 {
                let i =  (self.size() + index % self.size()) % self.size();
                i as usize
           } else {
                let i = index.abs() % self.size();
                i as usize
            }
        }
    }

    pub fn size(&self) -> i32{
        return self.configuration.len() as i32
    }

    pub fn is_00000001_shifting_from(&self, other: &Ring) -> bool {
        let ring = self.configuration.iter().map(|c| c.to_string()).collect::<String>();
        //TODO nettoyer ça
        let position1 = {
            let mut position = -1;
            for i in 0..self.size() {
                if self.get(i) == &'1' {
                    if position == -1 {
                        position = i;
                    } else {
                        position = -1;
                        break;
                    }
                }
            }
            position
        };

        let position2 = {
            let mut position = -1;
            for i in 0..other.size() {
                if other.get(i) == &'1' {
                    if position == -1 {
                        position = i;
                    } else {
                        position = -1;
                        break;
                    }
                }
            }
            position
        };

        if position1 > -1 && position2 > -1 {
            if position2 == position1-1 {
                println!("{} : {}       {} : {}", self, position1, other, position2);
                //println!("{}", ring);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

}


impl Display for Ring {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument)
        let mut pretty_string = String::new();
        pretty_string.push_str("| ");
        for cell in &self.configuration {
            pretty_string.push_str(&cell.to_string());
            pretty_string.push_str(" | ");
        }

        write!(f, "{}",pretty_string)
    }
}

impl PartialEq for Ring {
    fn eq(&self, other: &Ring) -> bool {
        let first_ring = self.configuration.iter().map(|c| c.to_string()).collect::<String>();
        let second_ring = other.configuration.iter().map(|c| c.to_string()).collect::<String>();
        if first_ring == second_ring {
            true
        } else {
            false
        }
    }
}

fn main() {
    let base = vec!['0', '0', '0', '0', '0', '1'];
    let n = 3;
    let _max = usize::pow(2, u32::pow(2, n as u32));
    for i in 1..256 {
        let mut automaton = Automaton::new(&base, i , n);
        while !automaton.state_already_generated(){
            automaton.is_00000001();
            automaton.next_gen();
        }
        //println!("{}", automaton);
    }

}
