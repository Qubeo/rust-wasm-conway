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
Tackle the "wrap-around problem" properly (is that even useful, non-misleading conceptualization)?
Implement tailored (click-in) pattern insertion
Implement performance meter
Enumerate computational complexity (O, M).
    Implement comparisons of different functins
Implement graphs of density, various other metrics
Write proper documentation comments
Add "sentient agents" (perhaps even FEP inspired, with markov blankets), interacting with the environment :)
UX: When adding a custom pattern, display a "shadow" pattern-to-be over the current mouse location in real-time.

**Conceptual learnings**
The wrapping is an array rotation problem. (Which, generalized, is WKO problem?)

**Reflections**
I'm getting distracted and prolapse into optimizing not yet finished functionalities, optimizing optimizations etc.
Also I often get distraught by lenghty and arguably too detailed language specifics readings, instead of MVP drive.
Kind of a crowded, perhaps pressured style. Like my thinking had just this small space to operate in, and started folding onto itself.
Choking with the crowded symbols, pushing for svinování, optimalizace, without having the space to expand the problems and solutions first, though.
Too much of a "defensive programming"?
-> Really take on one solution, one specific way of implementing it, be it the most inefficient one, and make an MVP. Then optimize.


**Misc & learning**
"There is a fallacy in this: that using a one-dimensional array is more efficient than using a two-dimensional array. If arrays are stored on the heap, certainly; but consequent to Rust’s representation of arrays, a two-dimensional array like [[T; 32]; 32] is a perfectly reasonable way of doing things, with often nicer ergonomics, and no space overhead."
https://www.reddit.com/r/rust/comments/76olo3/why_rust_fails_hard_at_scientific_computing/

"Also note that you should never accept a &Vec<T> as a parameter. Always use a &[T] as it's more flexible and you lose nothing."

https://blog.bitsrc.io/good-engineering-practices-while-working-solo-ad872e727af4

http://lucumr.pocoo.org/2018/3/31/you-cant-rust-that/

Iterators: https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html

Macro expanded log: rustc -Z unstable-options --pretty=expanded src/main.rs