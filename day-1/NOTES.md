# Part One
- Dial from 0 - 99
- Dial starts @ 50
- Dial loops (like a dial on a safe)

- input.txt holds one turn per line (direction + amount)
- "The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence."

# Part Two

This one I found to be tricky, a two key considerations make this easier

- If a dial ends at 0 from a `clockwise move`, it MUST have crossed 99 
    - Ensure you don't add twice (once for crossing 99, once for hitting zero), instead only add once

- If a dial ends at 0 from a `counter-clockwise` move, it may not have crossed the 99/0 threshold