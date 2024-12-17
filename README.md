# frame space tests

implements the math required for an inline viewport and line pushing to ensure the log scrolls up above the viewport.

![diagram](/assets/diagram.png)


- We need to push the terminal down by the number of lines printed (capped at the height of the terminal)
- We need to push up any text that is overlapping with the frame
