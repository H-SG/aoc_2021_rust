# Advent of Code 2021 - Rust
So this is my work on Advent of Code 2021 with Rust. I've never used rust before the first challenge this year, so this will be quite a journey!

## Advent Journal
In lieue of making blog posts (later!) I'll log my experiences so far day by day

### Day 1
I almost missed getting my submissions done on time, but we made! I'm primarily working with VS Code and WSL II. Setting up the Rust environment with `rustup` was very easy and I do enjoy how compilation and packaging is managed with crates and `cargo`. One of the things which always bothered me about C and C++ projects is dependancy management. Though that is perhaps not entirely fair considering how long ago (in terms of time and skills!) I've done seriously work with those languages.

It would be nice to sit down and the read THE BOOK as oxidised people like to call it, but I don't have time for that, I must code. As can be seen in the maximum verbosity of my comments I've done a lot of hand holding for myself as a reference to go back on. Some key take-aways which were a bit obtuse to understand were:
- How the `Result` type is used in rust for error handling
  - Caveat: I don't think I completely understand Rust error handling yet
- How primitives and such are defined and what this lifespan and borrowing thing is about
- How return handling and function datatypes are set
  - The `<>` syntax was not immediately obvious to me

While some of the official docs ramped up rapidly in difficulty or skipped what I felt were key steps, overall community documentation and support seems really good.

After setting up file parsing I had a lot of fun playing around with various array slices and multiple loops until I did some pen and paper testing of the solution function to get a simple algorithm out of it. Sadly, I neglected to commit those experiments. I expect a lot of my solutions are likely to be inelegant until I take some time to evaluate the solutions of others and get a better feel for the language.

### Day 2
Day 2 was quite easy, with the opportunity to use on of my favourite features in any language, pattern matching! I had some additional niggles with data types and passing strings around, but it was relatively quick to get done with the base I've established from day 1.

I'm on the fence about looking at interesting crates which do things like array maths handling. While probably clean, fast, and easy I think they may slow my learning speed on this adventure. I really want to get a better grip on the errors and `Results` in the language going forward.
