if not get_option('sqlite3').allowed()
  subdir_done()
endif

dep_sqlite3 = compiler.find_library('sqlite3', required : get_option('sqlite3'))

if not (dep_sqlite3.found() and compiler.has_function('sqlite3_open_v2', dependencies: dep_sqlite3))
  subdir_done()
endif

cfg_data.set('HAVE_SQLITE3', 1,
             description : 'Define to 1 if you have the libsqlite3 library (-lsqlite).')
if compiler.has_function('sqlite3_load_extension', dependencies: dep_sqlite3)
  cfg_data.set('HAVE_SQLITE3_LOAD_EXTENSION', 1,
               description : 'Define to 1 if sqlite3 supports loading extensions.')
endif
temacs_deps += dep_sqlite3
