use std::fmt::{self, Formatter, Display};
use std::collections::HashMap;

fn add_chars(how_many: usize, goal: String, c: char) -> String{
    if how_many>0 {
        add_chars(how_many-1, c.to_string() +  &goal, c)
    } else {
        goal
    }
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
    pub fn new(base: Vec<char>, rule: usize, neighborhood: usize) -> Automaton{
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

    fn populate_rules(&mut self){
        let boundary = self.neighborhood.pow(2)-1;
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

    pub fn next_gen(&mut self){

        let mut next_gen = Ring::new();
        {
            let last_gen = self.get_last();
            for i in 0..last_gen.size(){
                let current = last_gen.get(i-1).to_string() + &last_gen.get(i).to_string() + &last_gen.get(i+1).to_string();
                let result = self.rules.get(&current).expect(&format!("This is not supposed to happen : no rules exist for this configuration: {}", current));
                next_gen.push(result);
            }
        }
        self.generations.push(next_gen);
    }

    pub fn get_last(&self) -> &Ring{
        self.generations.last().unwrap()
    }

}

impl Display for Automaton {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut pretty_string = String::new();
        pretty_string.push_str(&format!("Rule : {}\n", self.rule));
        pretty_string.push_str(" | ");
        let boundary = self.neighborhood.pow(2)-1;
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

    pub fn from(base: Vec<char>) -> Ring{
        Ring{
            configuration: base
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

fn main() {
    //for argument in env::args() {
    let base = vec!['0', '0', '0', '0', '0', '1'];
    let mut automaton = Automaton::new(base, 1, 3);
    automaton.next_gen();
    automaton.next_gen();
    automaton.next_gen();
    automaton.next_gen();
    automaton.next_gen();
    automaton.next_gen();
    println!("{}", automaton);
}
