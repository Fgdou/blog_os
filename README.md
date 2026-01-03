# Blog OS

This is a kab follow the "Writing an OS in Rust", from Philipp Oppermann : https://os.phil-opp.com/

# Run on Qemu
```
cargo run
```

# Manually run on Qemu
```sh
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```

# Manually copy to USB stick
```
cargo bootimage
dd if=target/x86_64-blog_os/debug/bootimage-blog_os.bin of=/dev/sdX && sync
```
