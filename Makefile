HAXEHXML ?= build.hxml

USE_ZAPCC ?= 0

ifeq ($(DEBUG),1)
	RUSTBASE ?= banner
	HAXEBASE ?= Main-debug

	RUSTDIR ?= rust
	RUSTBUILD ?= target/debug
	HAXEDIR ?= haxe
	HAXEBUILD ?= build/hxwidgets

	RUSTFLAGS ?= build
	HAXEFLAGS ?= $(HAXEHXML) -debug
else ifeq ($(USE_ZAPCC),1)
	RUSTBASE ?= banner
	HAXEBASE ?= Mainnull

	RUSTDIR ?= rust
	RUSTBUILD ?= target/release
	HAXEDIR ?= haxe
	HAXEBUILD ?= build/hxwidgets

	RUSTFLAGS ?= build --release
	HAXEFLAGS ?= $(HAXEHXML) -D toolchain=clang
else
	RUSTBASE ?= banner
	HAXEBASE ?= Main

	RUSTDIR ?= rust
	RUSTBUILD ?= target/release
	HAXEDIR ?= haxe
	HAXEBUILD ?= build/hxwidgets

	RUSTFLAGS ?= build --release
	HAXEFLAGS ?= $(HAXEHXML)
endif

ifeq ($(VERBOSE),1)
	RUSTFLAGS += --verbose
	HAXEFLAGS += -D HXCPP_VERBOSE
endif

RUST ?= cargo
HAXE ?= haxe

RUSTSRC ?= src
HAXESRC ?= src

_RUSTDEPEND ?= $(shell ls $(RUSTDIR)/$(RUSTSRC))
_HAXEDEPEND ?= $(shell ls $(HAXEDIR)/$(HAXESRC)) $(foreach file, $(shell ls $(HAXEDIR)/$(HAXESRC)/../assets), ../assets/$(file)) $(foreach file, $(shell ls $(HAXEDIR)/$(HAXESRC)/../ifc), ../ifc/$(file))

RUSTDEPEND ?= $(foreach file, $(_RUSTDEPEND), $(RUSTDIR)/$(RUSTSRC)/$(file))
HAXEDEPEND ?= $(foreach file, $(_HAXEDEPEND), $(HAXEDIR)/$(HAXESRC)/$(file))

ifeq ($(OS),Windows_NT)
	RUSTTARGET ?= $(RUSTDIR)/$(RUSTBUILD)/$(RUSTBASE).lib
	HAXETARGET ?= $(HAXEDIR)/$(HAXEBUILD)/$(HAXEBASE).exe
else
	RUSTTARGET ?= $(RUSTDIR)/$(RUSTBUILD)/lib$(RUSTBASE).a
	HAXETARGET ?= $(HAXEDIR)/$(HAXEBUILD)/$(HAXEBASE)
endif

PHONY: all

all: $(HAXETARGET)

run: all
	$(HAXETARGET) opening.bnr

$(HAXETARGET): $(RUSTTARGET) $(HAXEDIR)/$(HAXEHXML) $(HAXEDEPEND)
	(cd $(HAXEDIR) && $(HAXE) $(HAXEFLAGS))

$(RUSTTARGET): $(RUSTDEPEND)
	(cd $(RUSTDIR) && $(RUST) $(RUSTFLAGS))

clean:
	rm -rf $(RUSTDIR)/$(RUSTBUILD) $(HAXEDIR)/$(HAXEBUILD)

clobber:
	rm -rf $(RUSTTARGET) $(HAXETARGET)
