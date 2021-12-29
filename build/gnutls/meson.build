if not get_option('gnutls').allowed()
  subdir_done()
endif

dep_gnutls = dependency('gnutls', version : '>= 2.12.2', required: get_option('gnutls'))

if not dep_gnutls.found()
  subdir_done()
endif

cfg_data.set('HAVE_GNUTLS', 1,
             description : 'Define if using GnuTLS.')
temacs_deps += dep_gnutls
