```cargo run us-citizenship.txt```

This only works on a linux terminal.

In the flashcard file, the first line contains the card width. Questions and
answers should be line wrapped to fit this width.
The questions can have optional numberings or namings, before
":". These are ignored. Spaces, then newlines after "Q ... :" and "A:", and
whitespace characters at the end of a question or answer are also ignored.

Questions and answers are printed directly to the terminal, so they should't
contain symbols that might cause problems.
