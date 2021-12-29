if not get_option('selinux').allowed()
  subdir_done()
endif

# SELinux is available for GNU/Linux only.
dep_libselinux = dependency('libselinux', required: get_option('selinux'))

if not (dep_libselinux.found() and compiler.has_function('lgetfilecon', dependencies: dep_libselinux))
  subdir_done()
endif

cfg_data.set('HAVE_LIBSELINUX', 1,
             description : 'Define to 1 if using SELinux.')
temacs_deps += dep_libselinux
