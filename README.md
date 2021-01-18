# telegrust-histo

Why? I was curious, and [Python proved too slow](https://github.com/urbanij/telegram-histo)<sup>1</sup> at parsing timestamps, although the plot looks nicer there.

<sup>1</sup> 0.23 seconds vs 30 seconds benchmarked on the same ~45k messages dataset.


#### Usage:

```sh
# install telegrust-histo with cargo, 
# provided that you have the rust toolchain set up.
cargo install telegrust-histo

# cd to wherever your exported folder is, in my case:
cd ~/Downloads/Telegram\ Desktop/ChatExport_2021-01-15 

# see telegrust-histo options
telegrust-histo -help

# run telegrust-histo 
telegrust-histo -b 300 -v
```


![](https://github.com/urbanij/telegrust-histo/blob/main/histogram.svg?raw=true)

#### Demo:

```sh
~/ChatExport_2021-01-15> telegrust-histo -b 100 -v
Renaming messages.html to messages1.html
Processing messages1.html
Processing messages2.html
Processing messages3.html
Processing messages4.html
Processing messages5.html
Processing messages6.html
Processing messages7.html
Processing messages8.html
Processing messages9.html
Renaming messages1.html to messages.html
```