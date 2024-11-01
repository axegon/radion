# RADION - rtl-sdr bindings for Rust

![Here se go again](https://imgs.xkcd.com/comics/standards.png)

### ^ `:s/standard/binding/g`



There are a million and one of those libraries, I know. The reason I sat down and did this on a Friday night is simple: all the other ones were abandoned years ago and are no longer actively maintained. Arguably the same will happen to this one too but it is in active development while I still need it for a weekend project.

Disclosure: It has only been tested on Linux systems(it may or may not work on a Raspberry Pi, I intend to try it in the distant future). Does it work on other operating systems? Maybe, maybe not...

### Requirements

* rtl-sdr-devel or however your distro has decided to call it.
* The appropriate udev rules for your SDR(tested with a RTL2838).

If you want to view the output of the examples, you can check the script [here](https://gist.github.com/axegon/1fcbfc2ad38a4e14625755b2cdbe32a3).