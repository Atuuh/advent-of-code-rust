Advent of code solutions in rust

Repo is in a state of disrepair. It's very manual and convuleted to create a new day and run it.

Ideally we have a few components to help automated these repetitive processes.

- **templater**: given a year/date, create the module files for that days puzzles and link inside years module project [^1]
- **input fetcher**: given a year/date, download the input for an authorised user [^2]
- **cli**: the main entrypoint to actually run and get the puzzle output. Will provide functionality to fetch inputs, template, and run a days solution 

[^1]: I'm not sure exactly how to do this part, will need to research how rust actually handles this
[^2]: Probably needs some secondary auth module - other things might use it too

