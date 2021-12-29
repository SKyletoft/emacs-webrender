if not get_option('lcms2').allowed()
  subdir_done()
endif

dep_lcms2 = dependency('lcms2', required: get_option('lcms2'))

if not dep_lcms2.found()
  subdir_done()
endif

cfg_data.set('HAVE_LCMS2', 1,
             description : 'Define to 1 if you have the lcms2 library (-llcms2).')
temacs_deps += dep_lcms2
