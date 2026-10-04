# Rust Memory Notes

Notes from lesson 1: variables, mutability, and how values sit in memory.

## 1. Memory is numbered boxes

Each box holds 1 byte. Each box has an address (a number).

```
address:  100  101  102  103
box:     [ 05][ 00][ 00][ 00]
          └──── i32 x ────┘
```

An `i32` uses 4 boxes in a row. The value 5 is stored as `05 00 00 00` (low byte first, called little-endian).

## 2. `mut` vs shadowing

**`mut`** changes the same boxes.

```rust
let mut x = 5;
x = 6;
```

```
before: 100:[05 00 00 00]
after:  100:[06 00 00 00]   same boxes, new content
```

**Shadowing** uses new boxes. The old boxes stay until the scope ends.

```rust
let x = 5;
let x = 6;
```

```
100:[05 00 00 00]   old x, hidden by name
116:[06 00 00 00]   new x
```

| | `mut` | shadowing |
|---|---|---|
| Keyword | `let mut x` | `let x` again |
| Same variable? | yes, same memory | no, new variable |
| Can change type? | no | yes |
| Runtime cost | none | none (may use more stack) |

`mut` exists only at compile time. It tells the borrow checker that writes are allowed. It adds nothing to the compiled program.

## 3. Stack and heap

**Stack**: fast and small. Every variable has a fixed size known at compile time. `i32` is always 4 bytes.

**Heap**: large and flexible. Size can be decided while the program runs.

### Program memory

```
STACK (one per thread)          HEAP (one shared pool)
function frames                 allocations
```

- Stack: one per thread. Each function call adds a frame on top. When the function returns, the frame is removed. A frame holds that function's local variables.
- Heap: one big pool. Anything can ask it for space.

There is not one stack and one heap per string. There is one stack per thread and one shared heap.

### Why String needs both

```rust
let s = String::from("hi");
```

The size of `s` depends on the text. The stack needs a fixed size, so the letters go on the heap. The stack holds a small note that says where they are.

```
STACK                              HEAP
address 100:                       address 5000:
 [ptr = 5000] ---- points to ----> [h][i]
 [len = 2]
 [cap = 2]
```

- `ptr`: heap address where the letters start.
- `len`: letters in use.
- `cap`: space reserved.

The stack part is always 24 bytes (8 + 8 + 8 on a 64-bit machine), no matter how long the text is.

### Many strings

```rust
let a = String::from("hi");
let b = String::from("world");
```

```
STACK (one)                     HEAP (one)
a: [ptr=5000][len=2][cap=2] --> 5000: [h][i]
b: [ptr=5016][len=5][cap=5] --> 5016: [w][o][r][l][d]
```

### Who allocates

`String::from("hi")` asks the allocator for 2 bytes on the heap. The allocator returns an address. Rust stores that address in the stack note (`ptr`).

### Growing

```rust
s.push('!');   // needs 3 bytes, cap is 2
```

The heap block is too small, so Rust asks the allocator for more (cap became 8 in the experiment). The stack note is updated: `len = 3`, `cap = 8`. The allocator may grow the block in place or move it to a new address. In the experiment, the address stayed the same.

### Move

```rust
let b = a;
```

Only the 24-byte stack note is copied. The letters stay where they are. Both notes would now point at the same heap block, which would cause a double free. Rust marks `a` as dead, so only `b` can use the data.

### Free

When the owner goes out of scope, Rust calls `drop`. `drop` returns the heap block to the allocator. The stack note disappears with its frame. One owner means exactly one free.

## 4. Address ranges seen in the experiment (macOS, 64-bit)

- `0x16f...` stack
- `0x101...` heap
- `0x100...` read-only data (string literals like `"hello"`)

## 5. Sizes

| Type | Size in bytes |
|---|---|
| `i32` | 4 |
| `char` | 4 |
| `bool` | 1 |
| `&str` | 16 (pointer + length) |
| `String` | 24 (pointer + length + capacity) |
| `Box<i32>` | 8 |
| `Option<Box<i32>>` | 8 (null pointer means `None`, no extra tag) |

Field order inside `String` is not guaranteed by the language. In the experiment it was `cap, ptr, len`.

## Not covered yet

Padding and struct field reordering, the niche optimization, lifetimes.
