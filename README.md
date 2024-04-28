# Encounter log parser for ESO (The Elder Scrolls Online)

I created this some time ago to extract some data from ESO fight logs, and decided i might as well share it

Its partially focussed on speed, rellies SIMD, achieving ~430MiB/s per thread on Zen2 CPU

There are some missed optimizations but its more than enough for my use case

Refer to `eso_lib::Event`, `eso_lib::State` for usage, requires nighly compiler
