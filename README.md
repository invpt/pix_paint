# pix_paint
Small "pixel art editor" application in Rust using minifb.

# Features
 - A whole eight colors!
 - Saving image as a bitmap file called `save.bmp` within the running directory.

# Limitations
 - Only eight colors, by design to make it simpler.
 - No user interface other than the image you're working on and a color indicator on the right.
 - Due to the lack of any user interface, it must be recompiled to change settings.
 - Can not open images to edit.

# Planned Features
 - At some point I will implement a config file to allow changing settings without requiring recompilation.

