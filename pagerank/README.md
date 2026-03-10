# pagerank

Attempt to use pagerank on Wikipedia! Dataset courtesy of https://zenodo.org/records/2539424. I'm using `enwiki.wikilink_graph.2018-03-01.csv`, for future reference.

## How do I run this?

First of all, get yourself a copy of the data at https://zenodo.org/records/2539424/files/enwiki.wikilink_graph.2018-03-01.csv.gz?download=1  (it's 2.4gb)

Then, double-click to extract it.

Then, download the compiled binary at the github release: https://github.com/JBlitzar/myrust/releases/download/pagerank_01/pagerank

That should be all you need! Run this in Terminal, assuming both the code and data are in your downloads folder and you're on mac:
```
cd ~/Downloads; chmod +x pagerank;./pagerank
```

This should produce page_rank_scores.txt, showing the sorted pagerank scores. Be careful opening it, the file's pretty big!

If you just want the scores, they're at the github release: https://github.com/JBlitzar/myrust/releases/tag/pagerank_01

If you want to see the code, check out [src/main.rs](src/main.rs). 

## How does PageRank work?

From the presentation:

> In more detail:
>  - Assign every page an initial score
>  - In each iteration, update a page’s score to be the sum of:
>     - the score of each page pointing to it, divided by the number of outgoing links from each page
>  - Iterate until scores converge

I plan to do exactly that. I'll need to parse csvs and manage OOP, but otherwise this is some pretty good practice in both algorithms and blazingly fast memory safe Rust 🚀. 

Plus, there's the damping factor. And the supernode.



## Stuff I did

I pivoted from OOP to pure vectors for performance.

Switched from `Vec<Vec<usize>>` to CSR for performance (again). 

And, it turns out that the secret sauce to optimizing your code to be 10x faster is just to build as release!

magic command: 
`RUSTFLAGS="-C target-cpu=native" cargo build --release;./target/release/pagerank`

Anyways, CSR was wild to learn because it flattens arrays. Also, we know how big the data is before running, so we can preallocate it.

## Some insights (if you can call it that)

top-ranked pages:
```
3434750 United States   0.00034004136
32927   World War II    0.00015744356
31717   United Kingdom  0.00014872233
11867   Germany 0.00013401479
30680   The New York Times      0.00012671664
10568   Association football    0.00012632448
68253   List of sovereign states        0.00012590224
273285  Race and ethnicity in the United States Census  0.00012018047
14533   India   0.0001181787
5042916 Canada  0.000112617854
645042  New York City   0.00010655126
11039790        Animal  0.00009909094
606848  Catholic Church 0.00009843484
5405    China   0.00009768245
4689264 Australia       0.00009541707
14532   Italy   0.00009315149
17867   London  0.00009252938
15573   Japan   0.00009195115
25391   Russia  0.00009128735
9316    England 0.00009126116
```

Bottom-ranked pages:
```
10325193	Our Lady's Secondary School, Templemore	0.000000011348568
44952857	Alfred John Liversedge	0.000000011348568
44069291	North-West Europe, 1944–1945	0.000000011348556
38762564	2010–13 Colonial Athletic Association realignment	0.000000011348552
29232866	Aziza Brahim	0.000000011348543
33892906	Ezequiel Garré	0.000000011348542
8301408	Matt Bardock	0.000000011348538
21248637	50th Street (IRT Sixth Avenue Line)	0.0000000113485275
19311307	Valley Junction, Oregon	0.000000011348524
47193859	1927 Boston College Eagles football team	0.000000011348524
5837432	Sporting Charleroi	0.000000011348519
42471351	Do Cao Tri	0.000000011348519
48560777	Chloé Robichaud	0.000000011348519
2517243	Kogyoku Tenno	0.0000000113485115
2517322	Tenji Emperor	0.0000000113485115
39680474	European Sprint Swimming Championships 1992	0.00000001134851
11116796	
```

I hypothesize that "European Sprint Swimming Championships 1992" is ranked at the bottom because it's probably linked relatively few times, but it links out *many* times. 