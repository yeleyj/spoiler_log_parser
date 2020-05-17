# THIS IS NOT TO BE USED IN A RACE OR COMPETITIVE SETTING! 

### About
I wanted to gain more experience with rust, so I decided to write a parser for the alttpr.com V31 spoiler logs. ALTTPR stands for A Link to the Past Randomizer and randomizes various item and/or boss locations within the game to make each playthrough different.

Currently, the code is not very efficient. I simply got a version that works together, and will work on improving the rust code beneath it (so many String:: calls now!). That's what learning is about, I suppose. I will continue to add tests, documentation, and features in my spare time.

### Usage
```
spoiler_log_parser <path_to_spoiler_log_file>
EXAMPLE: spoiler_log_parser "/home/bob/alttpr - NoGlitches-open-ganon_29MQnAgKvD.txt" # output to stdout
EXAMPLE: spoiler_log_parser "/home/bob/alttpr - NoGlitches-open-ganon_29MQnAgKvD.txt" > /home/bob/spoiler.json # output to file
```

### Building, running, and testing source
Building is simple. From the spoiler_log_parser directory, simply run `cargo test && cargo build && cargo run "/home/bob/spoiler.json" "/home/bob/output.json"`

### Future plans:
- Make binaries available for various platforms (at least linux and Win10) to avoid requiring installing rust (not great on Windows)
- Friendlier names for some things (could tie into the below)
- i18n or whatever works for translations?
- Support other modes (this currently only definitely works on the settings I chose, but should work for all v31 I think)
- Implement the shops struct if requested/required
- Attach to a pretty UI
- Attach to a web server and do more stuff (users, options, etc. maybe?)
- Create a mapper that shows all the locations on map (kinda like a tracker but with more info such as actual rewards)
- Add an option to try to generate routes based on different options, but that is a long way off.

~JY

