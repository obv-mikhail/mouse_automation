# mouse_automation
A winapi wrapper that provides easy mouse automation. Can be used from a variety of languages.

## Installation
### Rust
Add this to the toml file:
```toml
[dependencies]
mouse_automation = "0.1.1"
```
**Example:**
```Rust
extern crate mouse_automation;

fn main() {
    mouse_automation::move_mouse(0, 0, true); // moves mouse to top left corner
}
```


## Functionality
### Moving the mouse
```Rust
mouse_automation.move_mouse(mut x: i32, mut y: i32, abs: bool);
```

Third parameter should be true for asbolute movement, and false for relative.

### Simulating a click
```Rust
mouse_automation.LEFT.down();  
mouse_automation.LEFT.up();
```

```Rust
mouse_automation.RIGHT.down();  
mouse_automation.RIGHT.up();
```

```Rust
mouse_automation.MIDDLE.down();  
mouse_automation.MIDDLE.up();
```

Two commands are needed to simulate a click, the first sends a down event, and the second an up event. 

### Wheel scrolling
```Rust
mouse_automation.wheel(movement: u32);
```

The sign of the parameter indicates the direction of the scrolling.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
