# Drop by Nora Luna

I started off this project with the class template which is very useful since it contains all the basic crates and basic set up required for embedded software. This makes the set up process very easy. 

The first portion of the assignment I worked on was setting up a skeleton for the project. This was basically deciding what functions I would have and where they would go in the main program. This skeleton is what helped me develop the program incrementally.
```
loop {
   display_a_single_dot();
   while board_is_falling() {
      yell();
      show_exclaimation();
   }
}
```

Once I had this skeleton, I started in main grabbing all the needed peripherals. I used the [greyscale](https://github.com/pdx-cs-rust-embedded/mb2-grayscale/blob/main/src/main.rs) repository as a guide for setting up a nonblocking display. I started out with developing the single dot which is what is displayed when the microbit is not falling. Once I had that working, I implemented the display for an exclamation point, which is what is displayed when the microbit is falling.

Next I implemented the square wave functionality, using the [hello-audio](https://github.com/pdx-cs-rust-embedded/hello-audio/tree/main) repository as a guide. I tested this with the nonblocking display and they worked together. 

Lastly, I developed the accelerometer functionality. 

### Testing
I did attempt to set up the embedded-test crate for unit testing, but I could not get one of the dependecy crates to download correctly onto my system. It seems like an issue on the developer side of things. Instead I tested each piece incrementally as I developed it on the microbit to make sure it was working. 