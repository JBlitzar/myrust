# pagerank

Attempt to use pagerank on Wikipedia! Dataset courtesy of https://zenodo.org/records/2539424. I'm using `enwiki.wikilink_graph.2018-03-01.csv`, for future reference.

## How does PageRank work?

From the presentation:

> In more detail:
>  - Assign every page an initial score
>  - In each iteration, update a page’s score to be the sum of:
>     - the score of each page pointing to it, divided by the number of outgoing links from each page
>  - Iterate until scores converge

I plan to do exactly that. I'll need to parse csvs and manage OOP, but otherwise this is some pretty good practice in both algorithms and blazingly fast memory safe Rust 🚀. 

Plus, there's the damping factor. And the supernode (TODO implement).

I pivoted from OOP to pure vectors.
