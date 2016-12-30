################################################################################
#
# Copyright (c) 2016 SoftSec Lab KAIST
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.
#
################################################################################

SHURL=https://github.com/sparsehash/sparsehash/archive/sparsehash-2.0.3.tar.gz
SHFILENAME=sparsehash-2.0.3.tar.gz
SHDIRNAME=sparsehash-$(subst .tar.gz,,$(SHFILENAME))
SHDIR=$(shell pwd)/sparsehash
SHBUILDDIR=build/sparsehash
PATCHES=$(shell find $(PATCHDIR) -name '*.diff')

all: dep qemu-trace-coverage

clean:
	@rm -rf build
	@rm -rf qemu-trace-coverage

.dep:
	@mkdir -p .dep

.dep/shdown:
	@wget $(SHURL) && touch $@

.dep/shunpack:
	@tar xfz $(SHFILENAME) && rm -f $(SHFILENAME); touch $@

.dep/shrename:
	@mv $(SHDIRNAME) $(SHDIR) && touch $@

build:
	@mkdir -p build

$(SHBUILDDIR)/.compiled: build
	@mkdir -p $(SHBUILDDIR) && cd $(SHBUILDDIR); $(SHDIR)/configure && make
	@touch $@

dep: .dep .dep/shdown .dep/shunpack .dep/shrename $(SHBUILDDIR)/.compiled

qemu-trace-coverage: patches/baeum.cc $(PATCHES)
	./build_qemu_support.sh

.PHONY: all clean
