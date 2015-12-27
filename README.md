# Mouse-Automation
A winapi wrapper that provides easy mouse automation. Can be used from a variety of languages.

# Moving the mouse
mouse_automation.move_mouse(mut x: i32, mut y: i32, abs: bool);

First 2 parameters should be i32 values specifying the amount of movement that needs to occur. The abs parameter can be a true bool to make the mouse move to an absolute position rather than a relative one.

# Simulating a click
mouse_automation.left_down();

mouse_automation.left_up();


mouse_automation.right_down();

mouse_automation.right_up();


mouse_automation.middle_down();

mouse_automation.middle_up();

The first function call is used to send a down command, and the second to release the mouse. 

# Wheel scrolling
mouse_automation.wheel(movement: u32);

The sign of the parameter indicates the direction of the scrolling.
