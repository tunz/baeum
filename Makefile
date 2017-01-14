

QEMUDIR=$(shell pwd)/qemu

all: $(QEMUDIR)/.compiled baeum

baeum:
	@cargo build --release
	@cp ./target/release/baeum ./

debug:
	@cargo build
	@cp ./target/debug/baeum ./

$(QEMUDIR)/.compiled:
	cd $(QEMUDIR) && make
	@touch $@

clean:
	rm -rf ./baeum

cleanall:
	rm -rf ./baeum ./qemu-trace-coverage $(QEMUDIR)/.compiled
