all: target/kernel.o

run: target/kernel.o
	qemu-system-i386 -kernel $< -machine type=pc-i440fx-3.1

target/boot.o: boot/boot.S
	mkdir -p target
	nasm -felf32 $< -o $@

target/kernel.o: target/boot.o target/i686-lisibleos/release/libkernel.a
	mkdir -p target
	ld -m elf_i386 -n -T boot/linker.ld -o $@ $^

target/i686-lisibleos/release/libkernel.a:
	mkdir -p target
	xargo build -p kernel --target i686-lisibleos --release

clean:
	xargo clean --target i686-lisibleos
	rm target/kernel.o target/boot.o target/i686-lisibleos/debug/libkernel.a
	rm target/kernel.o target/boot.o target/i686-lisibleos/release/libkernel.a
