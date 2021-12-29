if not get_option('gpm').allowed()
  subdir_done()
endif

dep_gpm = compiler.find_library(
  'gpm',
  has_headers: ['gpm.h'],
  required : get_option('gpm'))

if not (dep_gpm.found() and compiler.has_function('Gpm_Open', dependencies: dep_gpm))
  subdir_done()
endif

cfg_data.set('HAVE_GPM', 1,
             description : 'Define to 1 if you have the gpm library (-lgpm).')
temacs_deps += dep_gpm
