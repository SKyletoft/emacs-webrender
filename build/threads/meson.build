if not get_option('threads').allowed()
  subdir_done()
endif

dep_threads = dependency('threads', required: get_option('threads'))

if not dep_threads.found()
  subdir_done()
endif

cfg_data.set('THREADS_ENABLED', 1,
             description : 'Define to 1 if you want elisp thread support.')
temacs_deps += dep_threads
