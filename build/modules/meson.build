if not get_option('modules').allowed()
  subdir_done()
endif

modules_deps = []
if host_system == 'windows'
  have_modules = true
elif ['gnu', 'linux'].contains(host_system)
  dep_dl = compiler.find_library('dl', required: get_option('modules'))
  if dep_dl.found()
    have_modules = true
    modules_deps += dep_dl
  endif

else
  # BSD systems have dlopen in libc.
  have_modules = compiler.has_function('dlopen')
endif

if not get_variable('have_modules', false)
  if get_option('modules').enabled()
    error('Dynamic modules are not supported on your system')
  endif

  subdir_done()
endif

cfg_data.set(
  'HAVE_MODULES', 1,
  description : 'Define to 1 if dynamic modules are enabled')

temacs_deps += modules_deps

if host_system == 'windows'
  modules_suffix='.dll'
elif host_system == 'darwin'
  modules_suffix='.dylib'
  modules_secondary_suffix='.so'
else
  modules_suffix='.so'
endif

if get_variable('modules_suffix', '') != ''
  cfg_data.set_quoted(
    'MODULES_SUFFIX', modules_suffix,
    description : 'System extension for dynamic libraries')
endif

if get_variable('modules_secondary_suffix', '') != ''
  cfg_data.set_quoted(
    'MODULES_SECONDARY_SUFFIX', modules_secondary_suffix,
    description : 'Alternative system extension for dynamic libraries.')
endif

foreach f : ['dladdr', 'dlfunc']
  if compiler.has_function(f, dependencies: modules_deps)
    cfg_data.set('HAVE_' + f.underscorify().to_upper(), 1)
  endif
endforeach
