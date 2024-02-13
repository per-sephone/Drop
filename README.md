# Drop by Nora Luna

I started off this project with the class template which is very useful since it contains all the basic crates and basic set up required for embedded software. This makes the set up process very easy. 

The first portion of the assignment I worked on was setting up a skeleton for the project. This was basically deciding what functions I would have and where they would go in the main program. This skeleton is what helped me develop the program incrementally.
```
loop {
   display_a_single_dot(&mut image);
   while board_is_falling() {
      yell();
      show_exclaimation(&mut image);
   }
}
```

Once I had this skeleton, I started in main grabbing all the needed peripherals. I used the {greyscale}(https://github.com/pdx-cs-rust-embedded/mb2-grayscale/blob/main/src/main.rs) repository as a guide for setting up a nonblocking display. I started out with developing the single dot which is what is displayed when the microbit is not falling. Once I had that working, I implemented the display for an exclamation point, which is what is displayed when the microbit is falling.