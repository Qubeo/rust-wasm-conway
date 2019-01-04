### Conceptual contemplations ###

**Learning goals**
I want to learn to measure memory and CPU demands / efficiency, and to enumerate it.
I want to learn to reason about the ideal data structure for the task (be it a bit field, a graph, etc.),
and gain conceptual clarity as of what kinds of mathematical / computational structures am I using and why.

I'd like to explore different angles of conceiving and solving the dead-alive pattern-matching problem.
Exploring different fields of mathematics and gaining at least a glimpse of their relatedness.
Could abstract algebras be of relevance here? Grammars? Geometric algebra?
Could some statistical mechanics principles be discovered / illustrated here?

**Data structure**
I'm contemplating the ideal data structure for the dead / alive filtering / pattern matching.
With the Vector of cells, I'm using a one dimensional structure, right?
This could be seen as a specific, narrow kind of a graph though, couldn't it?
What if I used a real graph and existing graph algorithms?


Write progressive optimization functions.
    Pattern-match / filter?
    2 or more in 1 go
    Matrices?
    Index table? (Switching 0th and w-1th index for x-1 etc.?)
    Inner and then borders?
    Generalize the X and Y wrapping function? As a closure?
    Reverse counting: go in chunks of 3, then identify the "neighbors for" cell. WKO POV shift? Hmh, asi blbost, stejně potřebuju wrap.
    -1, 0, 1 2D matrix?


Variations: "snaked" / spiral storage?

**TODO**
Tackle the "wrap-around problem" (is that even useful, non-misleading conceptualization)?
Implement performance meter
    Implement comparisons of different functins
Implement graphs of density, various other metrics
Write proper documentation comments


**Misc**
"There is a fallacy in this: that using a one-dimensional array is more efficient than using a two-dimensional array. If arrays are stored on the heap, certainly; but consequent to Rust’s representation of arrays, a two-dimensional array like [[T; 32]; 32] is a perfectly reasonable way of doing things, with often nicer ergonomics, and no space overhead."
https://www.reddit.com/r/rust/comments/76olo3/why_rust_fails_hard_at_scientific_computing/

https://blog.bitsrc.io/good-engineering-practices-while-working-solo-ad872e727af4

http://lucumr.pocoo.org/2018/3/31/you-cant-rust-that/

Iterators: https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html