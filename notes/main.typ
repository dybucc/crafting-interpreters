#import "@local/typst-template:0.40.0": *

#show: template.with(
  title: [Written challenges --- Crafting interpreters],
  authorship: (
    (
      name: "Adam Martinez",
      affiliation: "University of Life",
      email: "adammartinezoussat@gmail.com",
    ),
  ),
)

= The Lox language

1. Come up with Lox language edge cases.

`var foobar = (foo = bar)` for some existing variables `foo`, `bar` handles assignment in ways
unintuitive to C programmers; This should return something useful that is *not* `nil`. This actually
aligns with the way Rust does things; assignment is itself an expression, but it returns the `()`
value.

Another rough edge is the fact that being dynamically typed, the language takes a bit more time to
implement the runtime check for inherited class data members that are colliding with the
non-inherited class data members of the deriving class. This stems from the fact classes in Lox are
a literal piece of garbage. It would've been best to implement a new primitive type `dictionary`
with similar runtime checks for dynamic data member insertion, which itself would have exploited the
inclusion of closures onto Lox to add data members that themselves were references to routines
(which would also replicate the behavior of C when you build up a `struct` with function pointers.)

It's odd that Lox handles inner declarations of functions as capturing their environment, and on top
of that, allowing the lifetime of their captures values ot go beyond the scope in which those values
are implemented. I say this as a Rust programmer, so there's that, but even then, there's some hairy
things that would have to be figured out to find how to handle effects (in the type theory sense)
that relied on captures values, without having the GC mistakenly trash possibly costly but still
useful values within those closures.

2. Think about parts of the language that are underspecified in the chapter.

I stray far from GC-languages because lexical lifetimes are the essence of life. But I wonder how
much memory would a heavy numerical computation consume (e.g. a few million block matrix gaxpys) if
the computations were to be coded as chains of higher order functions with closures that capture
state and force it to have an extended lifetime.

Much in the same vein, can Lox closures be assumed to stop requiring regular stack frame-based
variable tracking the moment they are declared within a lexical scope, or can it be assumed that
some form of chained tracking is performed on the functions that _both_ capture state from the
surrounding lexical scope and are part of the same lexical scope. If for some lexical scopes $A, B$,
it holds that $A subset.neq B$, how does Lox handle variable capturing? Surely a naive approach that
took the route of chained tracking would make closure-heavy programs severely underperform. This
would be a hard-pressed issue to solve beyond just getting rid of the GC altogether or otherwise
coming up with a specific strategy for capturing (and _non_-capturing) closures.

Also, are there any values that qualify as being constant, and thus require not being mutated
throughout the program? These should likely be at least considered in the way Rust does it. This
would imply using some default specification for non-mutable variables, and some other (possibly
additional and not completely different) keyword for implying mutability.

3. Think about annoying things that don't make Lox usable in real life.

Something that is both left unspecified and would quite possibly cause issues is the interaction of
differing types, that don't necessarily align with the right abstract type classes (in the type
theoretical sense.) This would imply that some two different types would only correlate for, say,
addition, in whichever way runtime support was given to type resolution. The solution to this would
likely go through getting rid of a dynamic solution altogether.

Serialization/deserialization is likely another huge pain point, because the type theory in the
language is weak. This would make things as comfortable as `serde`'s implementation infeasible, in
part also due to the prior argument.

Barring the fact that the standard library is non-existent, there should at least be some form of
low-level interaction with the system such that language users could build abstractions over OS
primitives. This should go through performing interop with C, though this likely also implies that
there ought be some interface to other languages, even if it's indirect through C.
