```cargo run us-citizenship.txt```

This will probably only work on a linux terminal.

While parsing the flashcard file, ignores spaces, then new lines following "Q:" and "A:".
Ignores whitespace at the end of a question or an answer.
Questions and answers are printed directly to the terminal, so they should be line-wrapped if necessary and not contain symbols that might cause problems.
