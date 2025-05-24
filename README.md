# Rust Ownership 101

Still, rust ownership model which entails to how memory management is done is, in my opinion, the main innovation that the language has to offer compared to other system level programming models. 

Understanding how the rust ownership model works, is, in other words, understanding how rust works. 

## Examples

*   `ownership-ex5.rs`: Demonstrates how closures capture variables. It shows a scenario that can lead to a dangling reference if the captured variable goes out of scope before the closure is called. The example then illustrates how the `move` keyword resolves this issue by transferring ownership of the captured variables to the closure, allowing the closure to be used even after the original variables are no longer in scope.

Todo
- [ ] Adding two more examples of some of the shortcomings of rust, safety wise.
- [x] Add leaking memory and how to avoid it. 
