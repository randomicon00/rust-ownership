fn main() {
    // Scenario that would lead to a compile-time error

    let my_closure;

    {
        let my_string = String::from("hello");

        // This closure captures `my_string` by reference.
        my_closure = || {
            println!("Inside closure: {}", my_string);
        };

        // `my_string` goes out of scope here.
    }

    // Calling `my_closure` here would cause a compile-time error
    // because `my_string` no longer exists. This would be a dangling reference.
    // my_closure(); // Uncommenting this line would cause an error.
    println!("Commented out the problematic closure call to avoid compile error.");

    // How to fix this issue using the `move` keyword

    let my_moved_closure;

    {
        let my_vec = vec![1, 2, 3];

        // This closure uses the `move` keyword, so it takes ownership of `my_vec`.
        my_moved_closure = move || {
            // `my_vec` is now owned by the closure.
            println!("Inside moved closure: {:?}", my_vec);
        };

        // `my_vec`'s ownership was moved to the closure.
        // We can no longer use `my_vec` directly in this scope.
        // println!("{:?}", my_vec); // Uncommenting this would cause an error.
    }

    // We can call `my_moved_closure` even though the original `my_vec`
    // would have gone out of scope. This is because the closure owns the data.
    my_moved_closure();

    println!("Successfully called the moved closure.");
}
