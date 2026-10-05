# Lesson 3: impl Methods (structs and enums)

## What `impl` is

`impl` is short for implementation. `impl Player { ... }` reads as "implementation of functions for `Player`".

- `struct` defines what a type **has** (data).
- `impl` defines what a type **does** (functions).

```rust
struct Player {          // has
    hp: u32,
    level: u8,
}

impl Player {            // does
    fn new() -> Player { ... }
    fn show(&self) { ... }
    fn hurt(&mut self, n: u32) { ... }
}
```

Other short keywords: `fn` function, `mut` mutable, `struct` structure, `enum` enumeration, `mod` module, `pub` public, `ref` reference, `dyn` dynamic.

## Why use `impl`

1. Organize: all behavior of a type in one place.
2. Dot syntax: `p.hurt(30)` instead of `hurt_player(&mut p, 30)`.
3. No name clashes: `Player::new()` and `Enemy::new()` coexist.
4. Protect data: with private fields, only methods change them, so rules live in one place.
5. Traits: `impl Trait for Type` shares behavior across types (later lesson).

Python keeps data and methods in one `class`. Rust splits them: data in `struct`, behavior in `impl`. A type may have several `impl` blocks.

## Four kinds of method

| Signature | Meaning | Call | Needs `let mut p`? |
|---|---|---|---|
| `fn new() -> Player` | no `self`, associated function | `Player::new()` | n/a |
| `fn show(&self)` | read only | `p.show()` | no |
| `fn hurt(&mut self, n: u32)` | can change fields | `p.hurt(30)` | yes |
| `fn retire(self)` | takes ownership, value consumed | `p.retire()` | no, but `p` is dead after |

`self` is the value the method was called on. Rust makes you state how it is borrowed.

A `&mut self` method counts as a write, so the variable must be `let mut`. Without it:

```
error[E0596]: cannot borrow `p` as mutable, as it is not declared as mutable
```

You never write `&mut p` in the call. The method call borrows automatically from the signature. The compiler still checks `mut` on the variable.

Using `p` after `p.retire()`:

```
error[E0382]: borrow of moved value: `p`
```

## Memory

Data lives in each value. Functions live once in the code section, shared by every value of that type.

```
p1: [hp=100][level=1]      data, one per value
p2: [hp=50][level=3]
code: Player::hurt         one copy
```

`p.hurt(30)` is compiled as `hurt(&mut p, 30)`. It passes the 8-byte address of `p`, not a copy. Inside, `self.hp` is a read or write at that address plus a fixed offset. No object header, no lookup, no runtime cost.

`retire(self)` moves the value in. The caller's variable is dead afterwards.

## Built-in methods use `impl` too

`saturating_sub` is a method on integer types, defined in the standard library with `impl`.

```rust
let hp: u32 = 10;
hp.saturating_sub(30);   // 0
hp - 30;                 // panic in debug build (overflow)
```

| Method | On overflow |
|---|---|
| `a - b` | panic in debug, wrap in release |
| `a.saturating_sub(b)` | stops at the limit (0 for unsigned) |
| `a.wrapping_sub(b)` | wraps around |
| `a.checked_sub(b)` | returns `Option`, `None` on overflow |

Browse methods with `cargo doc --open`, or type `hp.` in the editor and read the autocomplete list.

## impl on enums

Same rules. Inside the method, `self` is one variant, so use `match self`.

```rust
enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14 * r * r,
            Shape::Rect(w, h) => w * h,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "circle",   // _ ignores the data
            Shape::Rect(_, _) => "rect",
        }
    }
}
```

`_` means "I do not need this value".

### Changing a variant with `&mut self`

```rust
enum Light { Red, Green, Yellow }

impl Light {
    fn next(&mut self) {
        *self = match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        };
    }
}

let mut light = Light::Red;
light.next();   // now Green
```

`*self = ...` replaces the whole value at that address. `*` means "the value at this address". In memory only the tag byte changes:

```
light: [tag=0 Red]  ->  [tag=1 Green]     same address
```

### Adding a variant

Add `Triangle(f64, f64)` to `Shape` and the compiler reports every `match` that does not handle it (`E0004: non-exhaustive patterns`). You cannot forget a place.

## Experiment results

Lesson 3 (struct) output:

```
hp = 100, level = 1
hp = 70, level = 1
retired at level 1
```

Lesson 4 (enum) output:

```
circle area = 12.56
rect area = 12
red
green
yellow
red
```

## Not covered yet

Traits, `Option` and `Result` methods (`map`, `?`, `unwrap_or`), `Vec`, lifetimes.
