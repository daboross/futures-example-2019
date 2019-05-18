futures-example-2019
====================

It's not immediately obvious what crates one should use trying to get `.await` working with Rust in
May, 2019.

To help that cause, I've created this crate. It does a single http request to
`https://jsonplaceholder.typicode.com/todos/1` using hyper, and prints the result. This should work
as a good starter for projects using tokio or hyper with async/await syntax.

This uses:

- [tokio] as an event loop
- [hyper] for http calls, as an example
- [futures-0.3 (futures-preview)] with "compat" feature for future-related things
- [the new `fut.await` syntax]

The crate is fairly simple, so it should be easy to explore.

Thanks to:

- [this blog post, detailing futures 0.3's compatibility API]( https://rust-lang-nursery.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html)
- [this users.rust-lang.org post, describing using tokio with futures-0.3](https://users.rust-lang.org/t/solved-issue-while-trying-to-abstract-repository-to-a-trait-async-await/28259/8?u=daboross)
- [the Rust "async book"](https://rust-lang.github.io/async-book/getting_started/chapter.html)

[tokio]: https://github.com/tokio-rs/tokio
[hyper]: https://github.com/hyperium/hyper
[futures-0.3 (futures-preview)]: https://github.com/rust-lang-nursery/futures-rs
[the `.await` syntax]: https://boats.gitlab.io/blog/post/await-decision/
