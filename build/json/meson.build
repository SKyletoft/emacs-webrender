if not get_option('json').allowed()
  subdir_done()
endif

dep_json = dependency('jansson', version : '>= 2.7', required: get_option('json'))

if not dep_json.found()
  subdir_done()
endif

cfg_data.set('HAVE_JSON', 1,
             description : 'Define if using Jansson.')
temacs_deps += dep_json
