# Kernel written in Rust
Following the awesome tutorial from  https://os.phil-opp.com/second-edition/.

## Setup
```
$ cargo install bootimage
```

## Runing
```
$ bootimage build
$ qemu-system-x86_64 -drive format=raw,file=bootimage.bin
```
