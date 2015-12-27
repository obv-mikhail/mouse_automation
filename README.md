# Mouse-Automation
A winapi wrapper that provides easy mouse automation. Can be used from a variety of languages.

## Moving the mouse
```mouse_automation.move_mouse(mut x: i32, mut y: i32, abs: bool);```

First 2 parameters should be values specifying the amount of movement that needs to occur. If the third parameter is specified to be true the mouse will move to an absolute position.

## Simulating a click
```mouse_automation.left_down();  
mouse_automation.left_up();```

```mouse_automation.right_down();  
mouse_automation.right_up();
```

```mouse_automation.middle_down();  
mouse_automation.middle_up();
```

Two commands are needed to simulate a click, the first sends a down event, and the second an up event. 

## Wheel scrolling
```mouse_automation.wheel(movement: u32);
```

The sign of the parameter indicates the direction of the scrolling.
