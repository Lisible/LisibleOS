all: target/kernel.o

clean_run: | clean run

run: target/kernel.o
	qemu-system-i386 -kernel $< -machine type=pc-i440fx-3.1 -serial mon:stdio

target/boot.o: boot/boot.S
	mkdir -p target
	nasm -felf32 $< -o $@

target/kernel.o: target/boot.o target/i686-lisibleos/release/libkernel.a
	mkdir -p target
	ld -m elf_i386 -n -T boot/linker.ld -o $@ $^

target/i686-lisibleos/release/libkernel.a:
	mkdir -p target
	RUST_TARGET_PATH=$(shell pwd) xargo build -p kernel --target i686-lisibleos --release

clean:
	RUST_TARGET_PATH=$(shell pwd) xargo clean --target i686-lisibleos
	rm -f target/kernel.o target/boot.o target/i686-lisibleos/debug/libkernel.a
	rm -f target/kernel.o target/boot.o target/i686-lisibleos/release/libkernel.a
