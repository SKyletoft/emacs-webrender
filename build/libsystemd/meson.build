if not get_option('libsystemd').allowed()
  subdir_done()
endif

dep_libsystemd = dependency('libsystemd', version : '>= 222', required: get_option('libsystemd'))

if not dep_libsystemd.found()
  subdir_done()
endif

cfg_data.set('HAVE_LIBSYSTEMD', 1,
             description : 'Define if using libsystemd.')
temacs_deps += dep_libsystemd
