# rx86

x86 emulator by Rust

https://book.mynavi.jp/ec/products/detail/id=41347


## Samples

```asm
BITS 32
start:
  mov eax, 41
  jmp short start
```

```console
$ make && cargo run bin/helloworld
make bin/helloworld
make[1]: `bin/helloworld' is up to date.
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rx86 bin/helloworld`
<Emulator code_size=7>
start emulation...
EIP=0: MovR32Imm32(0)
EIP=5: ShortJump
short jump to 0
end of program
EAX: 0x00000029 =         41
ECX: 0x00000000 =          0
EDX: 0x00000000 =          0
EBX: 0x00000000 =          0
ESP: 0x00000000 =          0
EBP: 0x00000000 =          0
ESI: 0x00000000 =          0
EDI: 0x00000000 =          0
```
