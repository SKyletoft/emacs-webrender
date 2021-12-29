if not get_option('xml2').allowed()
  subdir_done()
endif

dep_libxml2 = dependency('libxml-2.0', version : '>= 2.6.17', required: get_option('xml2'))

if not dep_libxml2.found()
  subdir_done()
endif

cfg_data.set('HAVE_LIBXML2', 1,
             description : 'Define to 1 if you have the libxml library (-lxml2).')
temacs_deps += dep_libxml2
