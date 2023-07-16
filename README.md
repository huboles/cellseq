# cellseq v0.2.3

a cellular automata driven midi sequencer

## dependencies

- [alsa](https://www.alsa-project.org)

## installation

- clone repo `$ git clone 'git://git.huck.website/cellseq'`
- enter directory `$ cd cellseq`
- build and run `$ cargo run`
- install `$ cargo install --path .`

## usage

![example session](https://huck.website/videos/cellseq.mp4)

*cellseq* consists of two interactive grids, refered to as the map (on the right)
and the mask (on the left). clicking inside either one toggles the selected grid
square on or off.

when the play button (top left corner) is pressed, the current map will be ran
as an iterative sequence of steps using the rules of
[conway's game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
to determine each step. each step is evenly spaced according to the set bpm and
note division.

    step_ms = 60000 / ( bpm / divisor )

during each step the current map will be compared with the mask, and any cells
that are selected on both are 'triggered'. the vertical slider between the two
boards determine the density of triggers that get turned into actual notes,
allowing greater control over the amount of notes being generated.

the notes generated can be further refined using the controls at the bottom.
selecting a scale will only allow notes in that scale, and the octave center
and range controls how high and low the scale will extend. the two vertical
sliders on the right give a range of velocities that will be generated.

for each trigger that becomes a note, *cellseq* checks if that note is already
playing, in which case it sends a note-off. otherwise it sends a note-on for the
selected midi channel. the voice count is limited to the selected number,
and randomly chooses what voice to cut when the limit is hit.

by turning on the loop functionality a small portion of the sequence will be
repeated ad infinitum. the length of this loop can be adjusted, and the start
point is set when the toggle is turned on.

by using the save and reset buttons on the top row, you can remember and recall
a map state, making it easy to test small changes in a base pattern. the clear
map and mask buttons revert the respective board to an empty state.

the horizontal slider on the bottom gives a variable level of random 'soup' that can be
generated on either the map or the mask according to the given probability.
