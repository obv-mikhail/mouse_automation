# mouse_automation
A winapi wrapper that provides easy mouse automation. Can be used from a variety of languages.

## Installation
### Rust
Add this to the toml file:
```toml
[dependencies]
mouse_automation = "0.1.1"
```
Example:
```Rust
extern crate mouse_automation;

fn main() {
    mouse_automation::move_mouse(0, 0, true); // moves mouse to top left corner
}
```

### Other
The dll needs to be in your project's directory.

**Python example:**
```Python
import ctypes
mouse_automation = ctypes.CDLL('mouse_automation')
mouse_automation.move_mouse(0, 0, 1) # moves mouse to top left corner
```

## Functionality
### Moving the mouse
```Rust
mouse_automation.move_mouse(mut x: i32, mut y: i32, abs: bool);
```

First 2 parameters should be values specifying the amount of movement that needs to occur. If the third parameter is specified to be true the mouse will move to an absolute position.

### Simulating a click
```Rust
mouse_automation.left_down();  
mouse_automation.left_up();
```

```Rust
mouse_automation.right_down();  
mouse_automation.right_up();
```

```Rust
mouse_automation.middle_down();  
mouse_automation.middle_up();
```

Two commands are needed to simulate a click, the first sends a down event, and the second an up event. 

### Wheel scrolling
```Rust
mouse_automation.wheel(movement: u32);
```

The sign of the parameter indicates the direction of the scrolling.
