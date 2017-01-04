/*
   Chatkey path coverage instrumentation.

   Modified codes are Copyright 2016 KAIST SoftSec.

   The instrumentation point and the automatic downloading and
   patching script are copied from afl QEMU mode.
   -----------------------------------------------------------------

   Written by Andrew Griffiths <agriffiths@google.com> and
              Michal Zalewski <lcamtuf@google.com>

   Idea & design very much by Andrew Griffiths.

   Copyright 2015, 2016 Google Inc. All rights reserved.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at:

     http://www.apache.org/licenses/LICENSE-2.0
 */

#include <stdint.h>
#include <sys/mman.h>
#include <fcntl.h>
#include <unistd.h>
#include <sparsehash/dense_hash_set>

#ifdef __x86_64__
typedef uint64_t abi_ulong;
#else
typedef uint32_t abi_ulong;
#endif

extern unsigned int afl_forksrv_pid;
#define FORKSRV_FD 198
#define TSL_FD (FORKSRV_FD - 1)

typedef abi_ulong target_ulong;

struct return_data {
    uint64_t exec_id;
    uint64_t subpath;
    uint32_t nodecount;
    uint32_t newnode;
};

extern "C" {
  void afl_request_tsl(target_ulong, target_ulong, uint64_t, int);
}

abi_ulong baeum_entry_point; /* ELF entry point (_start) */

static google::dense_hash_set<abi_ulong> global_node_set;
static google::dense_hash_set<abi_ulong> node_set;
static abi_ulong hash = 5381; // djb2 hash
static int baeum_start = 0;
static struct return_data* ret_data;

#define NODE_STACK_SIZE 5
static abi_ulong node_stack[NODE_STACK_SIZE];
static int node_stack_cur;
static abi_ulong node_stack_hash = 5381; // djb2 hash

extern "C" void global_baeum_setup(void) {
    int fd;
    char *outputpath = getenv("BAEUM_RET_PATH");

    fd = open(outputpath, O_RDWR);
    ret_data = (struct return_data*)mmap(NULL, sizeof(struct return_data), PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    close(fd);

    global_node_set.set_empty_key(0);
}

extern "C" void global_node_update(abi_ulong addr) {
    global_node_set.insert(addr);
}

extern "C" void baeum_setup(void) {
    baeum_start = 1;
    node_set.set_empty_key(0);
}

extern "C" void baeum_close(void) {
    baeum_start = 0;
    if (afl_forksrv_pid)
        close(TSL_FD);
}

static inline void update_node_stack_hash(register abi_ulong addr) {
    register unsigned int i;
    for (i=0; i < sizeof(abi_ulong); i++)
        node_stack_hash = ((node_stack_hash << 5) + node_stack_hash) + ((addr >> (i<<3)) & 0xff);
}

extern "C" void baeum_exit(int crashed) {
    int32_t orig_node_cnt;
    google::dense_hash_set<abi_ulong>::iterator it;
    int i;

    if (!baeum_start)
        return;

    baeum_start = 0;

    orig_node_cnt = global_node_set.size();
    for (it = node_set.begin(); it != node_set.end(); it++) {
        if (global_node_set.insert(*it).second)
            afl_request_tsl(*it, 0, 0, 1);
    }
    if (crashed) {
        i = node_stack_cur == 0 ? NODE_STACK_SIZE - 1 : node_stack_cur - 1;
        while (i != node_stack_cur) {
            update_node_stack_hash(node_stack[i--]);
            if (i < 0)
                i += NODE_STACK_SIZE;
        }
    }
    if (ret_data) {
        ret_data->exec_id = hash;
        ret_data->subpath = node_stack_hash;
        ret_data->nodecount = (uint32_t)node_set.size();
        ret_data->newnode = (uint32_t)(global_node_set.size() - orig_node_cnt);
    }
}

static inline void baeum_update_hash(register abi_ulong addr) {
    register unsigned int i;
    for (i=0; i < sizeof(abi_ulong); i++)
        hash = ((hash << 5) + hash) + ((addr >> (i<<3)) & 0xff);
}

extern "C" void baeum_log_bb(abi_ulong addr) {

    if (!baeum_start)
      return;

    node_set.insert(addr);
    baeum_update_hash(addr);
    node_stack[node_stack_cur++] = addr;
    if (node_stack_cur >= NODE_STACK_SIZE) node_stack_cur -= NODE_STACK_SIZE;
}
