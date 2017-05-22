/*
    Copyright 2016 Choongwoo Han <cwhan.tunz@gmail.com>.
    Copyright 2016 KAIST SoftSec.
    ---------------------------------------------------------------------
    Forkserver written and design by Michal Zalewski <lcamtuf@google.com>
    and Jann Horn <jannhorn@googlemail.com>

    Copyright 2013, 2014, 2015, 2016 Google Inc. All rights reserved.

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at:

      http://www.apache.org/licenses/LICENSE-2.0
*/


#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <signal.h>
#include <errno.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <sys/wait.h>

#define PATH_FORKSRV_FD      198
#define FORK_WAIT_MULT       10

static pid_t forksrv_pid;
static int fsrv_ctl_fd, fsrv_st_fd;

static pid_t child_pid = 0;
static int timedout = 0;
static int stdin_fd;

static void alarm_callback(int sig) {
    if (child_pid) {
        kill(child_pid, SIGKILL);
        timedout = -1;
    }
}

void error_exit(char* msg) {
    perror(msg);
    exit(-1);
}

pid_t init_forkserver_impl(int argc, char** args, uint64_t timeout, int forksrv_fd, int *fsrv_ctl_fd, int *fsrv_st_fd) {
    static struct itimerval it;
    int st_pipe[2], ctl_pipe[2];
    int status;
    int devnull, i;
    int32_t rlen;
    pid_t forksrv_pid;
    char **argv = (char **)malloc( sizeof(char*) * (argc + 1) );

    if (!argv) error_exit( "args malloc" );
    for (i = 0; i<argc; i++)
        argv[i] = args[i];
    argv[i] = 0;

    if (pipe(st_pipe) || pipe(ctl_pipe)) error_exit("pipe() failed");

    forksrv_pid = fork();

    if (forksrv_pid < 0) error_exit("fork() failed");

    if (!forksrv_pid) {

        struct rlimit r;

        if (!getrlimit(RLIMIT_NOFILE, &r) && r.rlim_cur < PATH_FORKSRV_FD + 2) {

            r.rlim_cur = PATH_FORKSRV_FD + 2;
            setrlimit(RLIMIT_NOFILE, &r); /* Ignore errors */

        }

        r.rlim_max = r.rlim_cur = 0;

        setrlimit(RLIMIT_CORE, &r); /* Ignore errors */

        setsid();

        devnull = open( "/dev/null", O_RDWR );
        if ( devnull < 0 ) error_exit( "devnull open" );
        dup2(devnull, 1);
        dup2(devnull, 2);
        close(devnull);

        dup2(stdin_fd, 0);
        close(stdin_fd);

        if (dup2(ctl_pipe[0], forksrv_fd) < 0) error_exit("dup2() failed");
        if (dup2(st_pipe[1], forksrv_fd + 1) < 0) error_exit("dup2() failed");

        close(ctl_pipe[0]);
        close(ctl_pipe[1]);
        close(st_pipe[0]);
        close(st_pipe[1]);

        setenv("LD_BIND_NOW", "1", 0);

        execv(argv[0], argv);

        exit(0);
    }
    free(argv);

    close(ctl_pipe[0]);
    close(st_pipe[1]);

    *fsrv_ctl_fd = ctl_pipe[1];
    *fsrv_st_fd  = st_pipe[0];

    it.it_value.tv_sec = (timeout * FORK_WAIT_MULT) / 1000;
    it.it_value.tv_usec = ((timeout * FORK_WAIT_MULT) % 1000) * 1000;

    setitimer(ITIMER_REAL, &it, NULL);

    rlen = read(*fsrv_st_fd, &status, 4);

    it.it_value.tv_sec = 0;
    it.it_value.tv_usec = 0;

    setitimer(ITIMER_REAL, &it, NULL);

    if (rlen == 4) {
      return forksrv_pid;
    }

    if (timedout) error_exit("Timeout while initializing fork server (adjusting -t may help)");

    if (waitpid(forksrv_pid, &status, 0) <= 0)
        error_exit("waitpid() failed");

    error_exit("Fork server failed");
}

void initialize_libexec (int argc, char** args, int fd, uint64_t timeout) {
    struct sigaction sa;

    sa.sa_flags     = SA_RESTART;
    sa.sa_sigaction = NULL;

    sigemptyset(&sa.sa_mask);

    sa.sa_handler = alarm_callback;
    sigaction(SIGALRM, &sa, NULL);

    stdin_fd = fd;
    forksrv_pid = init_forkserver_impl(argc, args, timeout, PATH_FORKSRV_FD, &fsrv_ctl_fd, &fsrv_st_fd);
}

void kill_forkserver() {
    if (forksrv_pid) {
        kill(forksrv_pid, SIGKILL);
        forksrv_pid = 0;
    }
}

int exec_fork(uint64_t timeout) {
    int res, childstatus;
    static struct itimerval it;

    if ((res = write(fsrv_ctl_fd, &res, 4)) != 4)
        error_exit("exec_fork: Unable to request new process from fork server");

    if ((res = read(fsrv_st_fd, &child_pid, 4)) != 4)
        error_exit("exec_fork: Unable to request new process from fork server");

    if (child_pid <= 0) error_exit("exec_fork: Fork server is mibehaving");

    it.it_value.tv_sec = (timeout / 1000);
    it.it_value.tv_usec = (timeout % 1000) * 1000;
    setitimer(ITIMER_REAL, &it, NULL);

    if ((res = read(fsrv_st_fd, &childstatus, 4)) != 4)
        error_exit("exec_fork: Unable to communicate with fork server");

    if (!WIFSTOPPED(childstatus)) child_pid = 0;

    it.it_value.tv_sec = 0;
    it.it_value.tv_usec = 0;
    setitimer(ITIMER_REAL, &it, NULL);

    if ( WIFEXITED( childstatus ) ) return 0;

    if ( WIFSIGNALED( childstatus ) ) {
        if ( WTERMSIG( childstatus ) == SIGSEGV ) return SIGSEGV;
        else if ( WTERMSIG( childstatus ) == SIGFPE ) return SIGFPE;
        else if ( WTERMSIG( childstatus ) == SIGILL ) return SIGILL;
        else if ( WTERMSIG( childstatus ) == SIGABRT ) return SIGABRT;
        else return timedout;
    } else {
        return 0;
    }
}
