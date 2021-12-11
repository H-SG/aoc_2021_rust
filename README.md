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

### Day 3
Ho boy! Binary operations are something I've never really done a lot of, and in this case they may be mildly overkill since we can probably do it all with string manipulation.

Regardless, the beast is slain even if some parts of it feel truly hideous to me. I'm afraid to look at some other solutions...

Once again I've neglected to commit some bizarre intermediate solutions, which I regret now.

### Day 4
Man, I feel like I made a major breakthrough today with some of the `iter` comprehension. While my initial solution was strongly dependant on nested loops, the final version is starting to feel like something that might be idiomatic rust. I've been purposely avoiding looking at other rust AOC solutions to try and use only the documentation and some other online resources to not taint my nascent dialect too much. I'm not quite sure yet I can build these types of chained commands without doing the loops first, but understanding is dawning and friction is falling.

I will likely refractor out the solutions for the various days soonish as the amount of code is getting a bit silly to navigate. This will also present a nice opportunity to see how importing from other rust files works with some of the functions that are starting to become common.

Lastly, I also need to give some attention to the debugger in VS Code. I really would like to get expressions working while stepping through the code. Maybe it's just something that rust doesn't do and I've been poisoned irrevocably by Python.

### Day 5
This will live on in infamy of me massively overcomplicating it. I built an engine that will recursively check each vector, assuming that they aren't discrete, but continuous for crossings or collinearity (and if intersecting while collinear then the intersecting distance). I will not be sharing that monstrosity, we're done here.

Aside, I've decided to not split the solutions up into separate instances, because it's nice to be able to scroll back and see what you've done before

### Day 6
This was super easy as long as you live on geological time scales. The naive solution gave way to something more sensible and very fast. Madames et Monsieurs, `rotate_left()`.

### Day 7
Another reasonably quick one, it seemed intuitive that we needed to move the crabs to the median, and then mean positions. I'm not sure if my rounding was off, but I went overboard with the second part to find the minimum fuel use. New function I learned today was `.copied()` which you can call on an `.iter()` to create a new copy on which you can do changes instead of borrowing the original.

One thing which has been annoying about rust is the relative instability of the language syntax, examples from just a few years ago are quite different and function in ways not similar to what we have now in the language. Reminds me of the dark days of python2 -> python3

### Day 8
I'll be honest, I'm not entirely sure how to not make this look nicer. Otherwise, our first crate is in the project! I was somewhat annoyed in that I had to restart VS Code entirely to make it stop giving linting errors about `.sorted()` not being a valid function on `.chars()`, but we got there in the end.

### Day 9
Nothing too surprising here, but I did get a chance to use recursion! I initially planned to use a 2d array map, but it was easier to just keep track of the seafloor width and use offsets as needed. I do like the filters I build into the index offsets to make sure we are always in valid bounds.

Did I mention I used recursion?

### Day 10
What can I say about my first solution... You can go and look it up in the commit history, but suffice to say it was very ugly. I made the logic for finding pairs overly complex based on flawed reasoning from my side. It got the right answer, but at what cost. I had in my mind the idea when I started to have a list from which items got dynamically popped, why I didn't do that the first time around is a mystery to me.

### Day 11
Took me some time to make sure I understood the logic of what was happening. I think my solution is a bit hacky in handling squids only flashing once per step, and I really should have used a datastructure here for the squids. Each squid could have a flashed state for the current cycle within the step, indices of it's neighbours, and it's current energy level. This would've likely made the loops a lot neater, but hey, the hack worked and going from part 1 to 2 was trivial.