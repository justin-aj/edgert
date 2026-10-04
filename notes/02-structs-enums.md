# Lesson 2: Data Types, Structs, Enums

## Integer types

`u` means unsigned (no negatives). `i` means signed. The number is the bits used.

| Type | Bits | Bytes | Range |
|---|---|---|---|
| `u8` | 8 | 1 | 0 to 255 |
| `u32` | 32 | 4 | 0 to 4,294,967,295 |
| `i32` | 32 | 4 | -2,147,483,648 to 2,147,483,647 |

`u8` has 2^8 = 256 values, so 0 to 255. Overflow panics in a debug build. It does not wrap silently.

Pick the smallest type that fits. If unsure, use `i32`. Defaults: integer is `i32`, float is `f64`.

`usize` and `isize` match the machine pointer size (8 bytes on 64-bit). `usize` is used for lengths and indexes. Mixing `i32` and `usize` is a compile error. Convert with `as`:

```rust
let i: i32 = 2;
let x = v[i as usize];
```

## All built-in data types

Scalar (one value, stack):

| Type | Bytes | Example |
|---|---|---|
| `i8 i16 i32 i64 i128` | 1, 2, 4, 8, 16 | `-5` |
| `u8 u16 u32 u64 u128` | 1, 2, 4, 8, 16 | `5` |
| `isize usize` | 8 on 64-bit | indexes, lengths |
| `f32 f64` | 4, 8 | `3.14` |
| `bool` | 1 | `true` |
| `char` | 4 | `'a'` |

Compound: tuple `(1, 2.5, 'c')`, array `[1, 2, 3]` (fixed size), unit `()`.

Text: `&str` (borrowed) and `String` (owned, growable).

Collections (heap): `Vec<T>`, `HashMap<K, V>`, `HashSet<T>`.

Pointers: `&T`, `&mut T`, `Box<T>`, `Rc<T>`, `Arc<T>`.

## Python and Rust side by side

| Python | Rust |
|---|---|
| `int` (unlimited size) | `i8` to `i128`, `u8` to `u128`, `isize`, `usize` (fixed bits) |
| `float` | `f64` |
| `str` | `String` or `&str` |
| `list` | `Vec<T>` (one type only) |
| `tuple` | tuple |
| `dict` | `HashMap<K, V>` |
| `set` | `HashSet<T>` |
| `None` | `Option<T>` |
| exception | `Result<T, E>` |
| `class` with fields | `struct` + `impl` |
| `Enum` | `enum` (variants can carry data) |

Other differences:

| | Python | Rust |
|---|---|---|
| Types | checked at runtime | checked at compile time |
| Change a variable | always | only with `mut` |
| Memory | garbage collector | ownership, freed at end of scope |
| Null | `None` anywhere | only inside `Option` |

## Option and Result

Both are ordinary enums.

```rust
enum Option<T> { Some(T), None }
enum Result<T, E> { Ok(T), Err(E) }
```

`Option`: a value may be absent. The compiler forces you to handle `None`.

```rust
match v.get(9) {
    Some(n) => println!("got {n}"),
    None => println!("missing"),
}
```

`Result`: an operation may fail and you want the reason.

```rust
match "abc".parse::<i32>() {
    Ok(n) => println!("number {n}"),
    Err(e) => println!("bad input: {e}"),
}
```

## Struct: this AND that

A struct groups values under one name. All fields exist at once.

```rust
struct Player {
    hp: u32,
    level: u8,
}
let p = Player { hp: 100, level: 1 };
println!("{}", p.hp);
```

Changing a field needs `let mut p`. Same rule as any variable.

## Enum: this OR that

An enum value is exactly one of its variants. Variants may carry data.

```rust
enum Direction { Up, Down, Left, Right }

enum Shape {
    Circle(f64),
    Rect(f64, f64),
}
```

`match` reads the variant and pulls out its data. The compiler requires every variant to be handled. Delete an arm and you get `E0004: non-exhaustive patterns`. When you add a variant later, the compiler shows every `match` that needs updating.

```rust
fn area(s: Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(w, h) => w * h,
    }
}
```

## Struct vs enum

| | struct | enum |
|---|---|---|
| Meaning | this AND that | this OR that |
| Fields | all exist | one variant at a time |
| Size | sum of fields + padding | tag + biggest variant + padding |
| Use for | thing with several properties | thing that is one of several kinds |

## Nesting

Both ways work and are common.

Struct with an enum field:

```rust
enum State { Alive, Dead }
struct Player { hp: u32, state: State }
```

Enum with a struct (or struct-like data) in a variant:

```rust
struct Point { x: i32, y: i32 }

enum Shape {
    Circle { center: Point, radius: f64 },
    Line(Point, Point),
}

enum Event {
    Click { x: i32, y: i32 },
    Key(char),
    Quit,
}
```

Nest to any depth. Structs stay side by side. Enums stay overlapped.

## How they sit in RAM

Addresses here are made up. Layout is typical for rustc on 64-bit little-endian. Padding bytes are shown as `??` because their content is unspecified.

Program memory map:

```
high address
┌──────────────────────┐
│ STACK (grows down)   │  local variables, function frames
├──────────────────────┤
│        free          │
├──────────────────────┤
│ HEAP (grows up)      │  String letters, Vec items, Box
├──────────────────────┤
│ read-only data       │  string literals
│ program code         │
└──────────────────────┘
low address
```

A struct or enum lives where its variable lives. A local variable is on the stack. A value inside a `Box` or `Vec` is on the heap.

### Struct of numbers

```rust
let p = Player { hp: 100, level: 1 };
```

```
address:  1000  1001  1002  1003  1004  1005  1006  1007
byte:     [64]  [00]  [00]  [00]  [01]  [??]  [??]  [??]
          └──── hp = 100 ────┘   level  └─ padding ─┘
```

8 bytes. Field offsets are fixed at compile time, so reading `p.level` is one read at base + 4. No lookup at runtime.

### Plain enum

```rust
let d = Direction::Left;
```

```
1000: [02]      Up=00, Down=01, Left=02, Right=03
```

1 byte, just the tag.

### Enum with data

`Circle(2.0)`:

```
1000: [00]                              tag 0 = Circle
1001: [??] x7                           padding (f64 must start at a multiple of 8)
1008: [00 00 00 00 00 00 00 40]         2.0 as f64
1016: [?? x8]                           unused (Rect's second f64)
```

`Rect(3.0, 4.0)`:

```
1000: [01]                              tag 1 = Rect
1001: [??] x7
1008: [00 00 00 00 00 00 08 40]         3.0
1016: [00 00 00 00 00 00 10 40]         4.0
```

24 bytes either way. Every `Shape` pays for its biggest variant. `match` reads the tag first, then reads the fields that variant owns.

### Struct with a String

```rust
struct Person { name: String, age: u8 }
let p = Person { name: String::from("Ana"), age: 30 };
```

```
STACK (32 bytes)                          HEAP
2000: [cap=3]                             
2008: [ptr=5000] ----------------------> 5000: [41][6e][61]   "Ana"
2016: [len=3]
2024: [age=30][?? x7]
```

Field order inside `String` is not guaranteed. Dropping `p` frees the heap block.

### Enum with a String

```rust
enum Message { Quit, Move(i32, i32), Write(String) }
```

```
Quit:         [tag=0][unused ..........]
Move(5, 7):   [tag=1][x=5][y=7][unused ]
Write("hi"):  [tag=2][String note] ---> heap: [h][i]
```

Only `Write` touches the heap. Rustc may hide the tag in spare bits of the `String`, so the real size can be 24, not 32. Check with `size_of::<Message>()`.

### Box

```rust
let b = Box::new(Player { hp: 100, level: 1 });
```

```
STACK                    HEAP
3000: [ptr=6000] ------> 6000: [64 00 00 00][01][?? ?? ??]
```

### Summary

| Thing | Stack bytes | Heap |
|---|---|---|
| struct of numbers | sum of fields + padding | none |
| plain enum | 1 byte tag | none |
| enum with data | tag + biggest variant + padding | none |
| `String` field | 24 | letters |
| `Vec<T>` field | 24 | items |
| `Box<T>` | 8 | the `T` |

## Experiment results

From `src/main.rs` step 2:

```
size Direction = 1
size Shape     = 24
```

`Shape` is 24 because the tag (1 byte) is padded to 8 so the `f64` fields align, then 8 + 8 for `Rect`.

## Not covered yet

`impl` blocks and methods, `Option` and `Result` combinators (`map`, `?`, `unwrap_or`), traits.
