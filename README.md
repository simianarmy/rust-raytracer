# rust-raytracer

Using the excellent [The Ray Tracer Challenge](http://www.raytracerchallenge.com) book to get my hands dirty with Rust (fwiw)!

## Notes

- Uses glm library for primitive data types and matrix operations

- Started out implementing shape functionality with traits but didn't like the results.  Switched to composition + enums
for cleaner &amp; more performant code.

- Spent waaaay too long trying to implement Groups with bidirectional trees.  Not an easy thing in Rust without tons of Arc/RefCell/Weak/blah.
Had to give up after some point and just reused the cleaner groups code from [this repo](https://github.com/ahamez/ray-trace) for my own sanity.

- Implemented all chapters but CSGs (requires trees again)

- Performance difference between the dev and release builds is insane

## TODO

- Material on groups apply to children

- Parallelize renderer

- Area lights, Spot lights

## Results

![Cover](/demos/cover.png?raw=true "Cover")

![Cat](/demos/cat.png?raw=true "Cat")

![Dragons](/demos/dragons.png "Dragons")

![Cow](/demos/cow.png?raw=true "Cow")

![Teddy Bear](/demos/teddy.png?raw=true "Teddy")

![Teapot](/demos/teapot.png?raw=true "Teapot")

