use std::collections::HashMap;

pub struct Combinations{
    pub combos: HashMap<String, i32>
}

#[derive(Clone)]
pub struct Metafrost{
    read: Vec<String>,
    position: Vec<String>,
    colour: Vec<String>
}

impl Metafrost{
    pub fn new() -> Metafrost{
        Metafrost{
            read: Vec::new(),
            position: Vec::new(),
            colour: Vec::new()
        }
    }

    pub fn establish_line(&mut self, read: String, position: String, colour: String){
        self.read.push(read);
        self.position.push(position);
        self.colour.push(colour);
    }

    pub fn establish_cof(self, mut combinations: Combinations) -> Combinations {
//        let mut combinations = combinations;
        let mut colour_vec: Vec<String> = Vec::new();
        let mut i = 0;
        while i < self.position.len()-1 {
            if self.position[i] == self.position[i + 1] {
                colour_vec.push(self.colour[i].clone());
            } else {
                colour_vec.push(self.colour[i].clone());
                let kmer_count = combinations.combos.entry(colour_vec.join(",")).or_insert(0);
                *kmer_count += 1;
                colour_vec = Vec::new();
            }
            i += 1
        }
        if self.position[i]==self.position[i-1] {
            colour_vec.push(self.colour[i].clone());
            let kmer_count = combinations.combos.entry(colour_vec.join(",")).or_insert(0);
            *kmer_count += 1;
        }else{
            colour_vec.push(self.colour[i].clone());
            let kmer_count = combinations.combos.entry(colour_vec.join(",")).or_insert(0);
            *kmer_count += 1;
        }
        return combinations;
    }
}