use std::{fs::File, io::{BufRead, BufReader, Write}};



fn main() {
    let capacity = 56722250;


    let mut page_names: Vec<String> = vec![String::new(); capacity]; // idx is page id, value is page name
    let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); capacity]; // idx is page id, value is list of incoming links
    let mut outgoing: Vec<usize> = vec![0; capacity]; // idx is page id, value is number of outgoing links

    let mut max = 0;
    
 
    
    let file = File::open("enwiki.wikilink_graph.2018-03-01.csv").unwrap();
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
        to = parts.next().unwrap().trim_end().parse().unwrap();
       

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
        incoming[to].push(from);
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
    println!("Capacity: {}", page_names.len());
    println!("Total lines read: {}", i);
    println!("Max page id: {}, Min page id: {}", max, min);

    let mut page_rank_scores = vec![1.0; max + 1];
    let mut next_scores = vec![0.0; max + 1];
    let mut sum;
    const D: f64 = 0.85;
    const EPSILON: f64 = 1e-6;
    let mut i=0;
    let mut t = 0.0;
    loop {
        t = std::time::Instant::now().elapsed().as_secs_f64();
        for page_id in 0..=max {
            sum = 0.0;
            for &incoming_id in &incoming[page_id] {
                sum += page_rank_scores[incoming_id] / outgoing[incoming_id] as f64;
            }
            sum /= max as f64;
            next_scores[page_id] = D * sum + (1.0 - D);
        }
        std::mem::swap(&mut page_rank_scores, &mut next_scores);
        
        i += 1;
        let max_delta = page_rank_scores.iter().zip(next_scores.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        if max_delta < EPSILON {
            break;
        }
        let elapsed = std::time::Instant::now().elapsed().as_secs_f64() - t;
        println!("Iterating... {}, delta: {}. Elapsed: {:.2?}", i, max_delta, elapsed);
    } 
    // println!("Page Rank Scores: {:?}", page_rank_scores);
    // write out
    let file = File::create("page_rank_scores.txt").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    for page_id in 0..=max {
        writeln!(writer, "{}\t{}\t{}", page_id, page_names[page_id], page_rank_scores[page_id]).unwrap();
    }

}
