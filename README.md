# mash

This reinvents a wheel, mainly `join`. The main difference is that this does not require sorting anything first, because it loads the whole `selector` side into memory.

## Future Direction

The main goal is just to provide a nice api around a tool that can replace some complicated `join | sort | cut` type commands. 