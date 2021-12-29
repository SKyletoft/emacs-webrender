if not get_option('zlib').allowed()
  subdir_done()
endif

dep_zlib = compiler.find_library('z', required: get_option('zlib'))

if not dep_zlib.found()
  subdir_done()
endif

cfg_data.set('HAVE_ZLIB', 1,
             description : 'Define to 1 if you have the zlib library (-lz).')
temacs_deps += dep_zlib
