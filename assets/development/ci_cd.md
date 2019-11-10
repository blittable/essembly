   ##                                CI_CD 

        0) Build and test on commit 
        1) Verify `core` can build to wasm 
        2) Fuzzing 
        3) Test and build on 4 platforms (x64, Android, iOS, ARM)


### Platform Notes

[embedded arm] (https://rust-embedded.github.io/book/intro/install/linux.html)
- building on linux:
`sudo apt install gdb-multiarch openocd qemu-system-arm`