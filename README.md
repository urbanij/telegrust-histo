# telegrust-histo

<img src="https://img.shields.io/crates/d/telegrust-histo">


Why? I was curious, and [Python proved too slow](https://github.com/urbanij/telegram-histo)<sup>1</sup> at parsing timestamps, although the plot looks nicer there.

#### Install:
```bash
# install telegrust-histo with cargo, 
# provided that you have the rust toolchain set up.
cargo install telegrust-histo
```

#### Usage:
```bash
# cd to wherever your exported folder is
cd ~/Downloads/Telegram\ Desktop/ChatExport_2021-01-15 

# print options
telegrust-histo -help

# run it
telegrust-histo -b 300 -v
```


![](https://github.com/urbanij/telegrust-histo/blob/main/histogram_200.svg?raw=true)

#### Demo:

```sh
~/ChatExport_2021-01-15> time telegrust-histo -b 200  # time is optional btw of course
[+] Generated histogram_200.svg (using 200 bins) in the current folder.
[+] Processed 47 files (46610 messages) from 2017-04-16 22:02 to 2021-01-13 13:30
telegrust-histo  0.27s user 0.02s system 97% cpu 0.295 total
```
---
<sup>1</sup> 0.3 seconds vs 30 seconds benchmarked on the same ~45k messages dataset.

