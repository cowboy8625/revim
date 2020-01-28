ReVim
=====

    ReVim stands for "Rust Edition Vim".  I wanted a Simple vim that would compile and run on any terminal
like in windows. We all know Vim can be ran on windows but its not the easiest to do. 
ALSO I just really wanted to make a text editor, so this was a cool project to do.
Currently ReVim is in a very early statge of progress.  If you would like to contribute
to ReVim submite a merge request.

Dependency's
============
    * crossterm = "0.14.2"
    * ropey = "1.1.0"

TODO List
=========

- [x] Welcome Message
- [ ] Status Bar
    - [x] Modes
    - [ ] File Type
    - [ ] File Name
    - [ ] Curser Location
- [ ] Modes
    - [x] Normal Mode
        - [ ] Fix Curser Movement with in the file
    - [x] Command Mode
    - [x] Insert Mode
        - [ ] Fix Welcome Message when starting a new file
    - [ ] Search Mode
- [ ] FileIO
    - [ ] Load File
    - [x] Save File
        - [ ] **Text is save reversed**
    - [ ] Open Full File
- [ ] Show `~` at a empty line
- [ ] Scrolling
- [ ] Syntax HighLighing

