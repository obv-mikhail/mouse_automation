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
