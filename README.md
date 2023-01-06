# rust-raytracer
Raytracer book project in Rust

## Notes

- Uses glm library for primitive data types and matrix operations

- Started out implementing shape functionality with Traits but didn't like the results.  Switched to composition + enums
for cleaner &amp; more performant code.

- Spent way too long trying to implement Groups with bidirectional trees.  Not an easy thing in Rust.
Had to give up after some point and just reused code from [this repo](https://github.com/ahamez/ray-trace) for my own sanity.

- Implemented all but CSGs (requires tree?)

- Performance difference between the dev and release builds is insane

## Samples

![Cover](/demos/cover.png?raw=true "Cover")

![Cat](/demos/cat.png?raw=true "Cat")

![Dragons](/demos/dragons.png "Dragons")

![Cow](/demos/cow.png?raw=true "Cow")

![Teddy Bear](/demos/teddy.png?raw=true "Teddy")

![Teapot](/demos/teapot.png?raw=true "Teapot")

