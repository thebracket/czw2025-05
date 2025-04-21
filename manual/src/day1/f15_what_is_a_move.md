# What IS a Move, Anyway?

A move is a "deep copy" - usually compiled to `memcpy` that invalidates the source. The compiler will almost always not actually perform the copy---so generally you can feel safe moving things.

Some rules":

* Use a *move* when you are handing responsibility/ownership to something else.
* Use a `clone` when you want them to operate on a copy (`.clone()`).
* Use a *reference* when you retain ownership, but let the function work with your data.
