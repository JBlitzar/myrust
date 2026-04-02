use std::{fs::File, io::{BufRead, BufReader, Write}};



fn main() {
    let capacity = 56722250;


    let mut page_names: Vec<String> = vec![String::new(); capacity]; // idx is page id, value is page name
    let mut incoming: Vec<Vec<u32>> = vec![Vec::new(); capacity]; // idx is page id, value is list of incoming links
    let mut outgoing: Vec<u32> = vec![0; capacity]; // idx is page id, value is number of outgoing links

    let mut max = 0;
    
 
    
    // let file = File::open("enwiki.wikilink_graph.2018-03-01.csv").unwrap();
    let file = File::open("test_graph.csv").unwrap();
    let mut reader = BufReader::with_capacity(64 * 1024 * 1024, file);
    let mut is_first_line = true;
    let mut i = 0;
    let total_lines = 163380007;

    let mut from;
    let mut from_name;
    let mut to;
    let mut line = String::new();
    let mut start = std::time::Instant::now();
    let mut parts;
    let mut min = usize::MAX;
    let mut total_edges: usize = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        if is_first_line  {
            is_first_line = false;
            line.clear();
            continue;
        }

        // println!("Line: {}", line);
        parts = line.splitn(4, '\t');
        from = parts.next().unwrap().parse().unwrap();
        from_name = parts.next().unwrap();
        to = parts.next().unwrap().parse().unwrap();
       

        if from > max  {
            max = from;
        }
        if to > max  {
            max = to;
        }
        if from < min {
            min = from;
        }
        if to < min {
            min = to;
        }
        // if (page_names.len() <= max) {
        //     page_names.resize(max + 1, String::new());
        //     incoming.resize(max + 1, Vec::new());
        //     outgoing.resize(max + 1, 0);
        // }
        if page_names[from].is_empty() {
            page_names[from] = from_name.to_owned();
        }
        if page_names[to].is_empty() {
            page_names[to] = parts.next().unwrap().to_owned();
        }
        total_edges += 1;
        incoming[to].push(from as u32);
        outgoing[from] += 1;
       
        i += 1;
        if i % 1_000_000 == 0 {
            
            let percent = (i as f64 / total_lines as f64) * 100.0;
            println!("{}%", percent);
            if percent > 10.0 && percent < 11.0  {
                let elapsed = start.elapsed();
                println!("Time taken for 10%: {:.2?}", elapsed);
                start = std::time::Instant::now();
            }

        }
        line.clear();
    }
    // add supernode
    for page_id in 0..=max {
        if outgoing[page_id] == 0 {
            incoming[0].push(page_id as u32);
            outgoing[page_id] += 1;
            total_edges += 1;
        }
    }


    println!("Capacity: {}", page_names.len());
    println!("Total lines read: {}", i);
    println!("Max page id: {}, Min page id: {}", max, min);

    // csr
    let mut csr_edges   = vec![0; total_edges]; // incoming page ids
    let mut csr_offsets = vec![0; max + 2]; // start of incoming links
    let mut edge_index = 0;
    // for every page, 
    for page_id in 0..=max {
        csr_offsets[page_id] = edge_index;  // start of incoming links for this page
        for &incoming_id in &incoming[page_id] {
            csr_edges[edge_index] = incoming_id; // set the next value to the id
            edge_index += 1;
        }
    }
    csr_offsets[max + 1] = edge_index;

    drop(incoming); // free memory



    let n = max + 1;
    let mut page_rank_scores = vec![1.0 / n as f32; n];
    let mut next_scores = vec![0.0; n];
    let mut sum;
    const D: f32 = 0.85;
    const EPSILON: f32 = 1e-3;
    let mut i=0;
    let mut t;
    let n = (max + 1) as f32;
    let base = (1.0 - D) / n;
    loop {
        t =  std::time::Instant::now();
        for page_id in 0..=max {
            sum = 0.0;
            let start = csr_offsets[page_id];
            let end = csr_offsets[page_id + 1];
            for &incoming_id in &csr_edges[start..end] {
                sum += page_rank_scores[incoming_id as usize] / outgoing[incoming_id as usize] as f32;
            }
            

            next_scores[page_id] = base + D * sum;
        }
        
        i += 1;
        let max_delta = page_rank_scores.iter().zip(next_scores.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f32::max);
        if  max_delta / page_rank_scores[1] < EPSILON && i > 100 {
            break;
        }
        if (i > 100){
            println!("Reached max iterations. Stopping.");
            break;
        }
        std::mem::swap(&mut page_rank_scores, &mut next_scores);

        println!("Iterating... {}, delta: {}. Elapsed: {:.2?}", i, max_delta, t.elapsed());
    } 
    // println!("Page Rank Scores: {:?}", page_rank_scores);
    // write out


    // I had AI write these two lines of code because I couldn't be bothered to make idiomatic Rust sorting while keeping track like zip() in Python. 
    let mut page_rank_with_names: Vec<(u32, &str, f32)> = (0..=max as u32)
        .filter(|&page_id| !page_names[page_id as usize].is_empty())
        .map(|page_id| (page_id, page_names[page_id as usize].as_str(), page_rank_scores[page_id as usize]))
        .collect();

    page_rank_with_names.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let file = File::create("page_rank_scores.txt").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for (page_id, page_name, page_rank) in page_rank_with_names.iter() {     
        writeln!(writer, "{}\t{}\t{}", page_id, page_name, page_rank).unwrap();
    }

    

}
