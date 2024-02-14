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

Lastly, I developed the accelerometer functionality. I primarily used the code from the [magnetrometer](https://github.com/nrf-rs/microbit/blob/main/examples/magnetometer/src/main.rs) repository as well as the [embedded-rust-doc](https://docs.rust-embedded.org/discovery/microbit/08-i2c/using-a-driver.html) for guidance in developing this portion.

After I had developed each part, I noticed that the microbit was not behaving how I expected. The first problem was the exclaimation point was not lighting up upon dropping. I determined the issue was that I was not passing in a timer that would show the exclaimation for a certain period of time. This was actually vital in being able to see the exclaimation point. The other issue was that there was only a slight sound emitting from the microbit. Upon further exploration I realized it was a similar issue. I needed to emit the sound for a certain time period in order to actually hear a continuous sound. Understanding how important timers are for this really helped my understanding of the project. 

One note is that this program seems quite sensitive to being "dropped", even small movements will trigger the dropping functionality.

### Testing
I did attempt to set up the embedded-test crate for unit testing, but I could not get one of the dependecy crates to download correctly onto my system. It seems like an issue on the developer side of things. Instead I tested each piece incrementally as I developed it on the microbit to make sure it was working. 