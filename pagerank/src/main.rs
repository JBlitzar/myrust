use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

struct Page {
    id: usize,
    rank: f64,
    outgoing_links: usize,
    incoming_links: Vec<usize>,
}

impl Page {
    fn new(id: usize) -> Self {
        Page { id, rank: 1.0, outgoing_links: 0, incoming_links: Vec::new() }
    }
    fn name(&self, page_names: &HashMap<usize, String>) -> String {
        page_names.get(&self.id).unwrap().clone()
    }
    fn update(&mut self, pages: &HashMap<usize, Page>) -> f64 {
        let mut new_rank = 0.0;
        // the score of each page pointing to it
        for incoming_id in self.incoming_links.iter() {
            new_rank += pages.get(incoming_id).unwrap().rank;
        }

        // divided by number of outgoing links
        new_rank /= self.outgoing_links as f64; // have to cast

        return new_rank;
    }
    
}

fn parse_row(seen_pages:&mut HashSet<String>, page_names: &mut HashMap<usize, String>, pages: &mut HashMap<usize, Page>, from: usize, to: usize, from_name: &String){
    // if name not in names map, then add it
    // append pages[to].incoming
    // increment pages[from].outgoing
    if !seen_pages.contains(from_name) {
       seen_pages.insert((from_name.clone())).to_string();
       page_names.insert(from,(from_name.clone()).to_string());
       pages.insert(from, Page::new(from));

    }

    pages.get_mut(&to).unwrap().incoming_links.push(from);
    pages.get_mut(&from).unwrap().outgoing_links += 1;

}



fn main() {

    // magically promote maps to being O(1)
    let mut page_names = HashMap::<usize, String>::new();
    let mut pages = HashMap::<usize, Page>::new();
    let mut seen_pages: HashSet<String> = HashSet::<String>::new();
    
    let file = File::open("enwiki.wikilink_graph.2018-03-01.csv").unwrap();
    let mut reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap(); // each line is read lazily
        println!("{}", line);
    }
}
