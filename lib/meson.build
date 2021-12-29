autoconf_opts = [
  '--without-all', '--without-x',
  '--without-xpm', #checked
  '--without-jpeg', #checked
  '--without-tiff', #checked
  '--without-gif', #checked
  '--without-png', #checked
  '--without-rsvg', #checked
  '--without-webp', #checked
  '--without-sqlite3', #checked
  '--without-lcms2', #checked
  '--without-libsystemd', #checked
  '--without-cairo', #checked
  '--without-xml2', #checked, there is more logic in autoconf
  '--without-native-image-api', #checked
  '--without-xft', # deprecated in advance
  '--without-harfbuzz', #checked
  '--without-libotf', #checked
  '--without-m17n-flt', #checked
  '--without-toolkit-scroll-bars', #checked
  '--without-xaw3d', #not ported something related to X_TOOLKIT = LUCID
  '--without-xim', # too much going on there, ignore
  '--without-xdbe', #ignore
  '--without-gpm', #checked
  '--without-dbus', #checked
  '--without-gsettings',
  '--without-selinux', #checked
  '--without-gnutls', #checked
  '--without-zlib', #checked
  '--without-modules',
  '--without-threads', #checked
  '--without-compress-install',
  '--without-json', #checked
  '--with-dumping=none', #checked
  '--disable-acl'
]

dep_acl = not get_option('acl').allowed() ? disabler() : dependency('libacl', required: get_option('acl'))
if dep_acl.found()
  temacs_deps += dep_acl # Could rise issue when meson found acl while autoconf doesn't
else
  autoconf_opts += '--disable-acl' # too complicated, leave it to autoconf
endif

dep_gmp = not get_option('libgmp').allowed() ? disabler() : dependency('gmp', required: get_option('libgmp'))
if dep_gmp.found()
  temacs_deps += dep_gmp # Could rise issue when meson found gmp while autoconf doesn't
else
  autoconf_opts += '--without-libgmp' # too complicated, leave it to autoconf
endif

if get_option('included-regex').disabled()
  autoconf_opts += '--without-included-regex' # too complicated, leave it to autoconf
endif

invoke_script = find_program(source_root / 'build-aux/invoke_script', required : true)
if not fs.is_file(source_root / 'configure')
  autogen_sh = find_program(source_root / 'autogen.sh', required : true)
  run_command(
      invoke_script, source_root, autogen_sh.full_path(),
      check: true)
endif

if not fs.is_file( build_root / 'config.status')
  message('Configuring gnulib...')

  configure = find_program(source_root / 'configure', required : true)
  run_command(
      invoke_script, build_root, configure.full_path(),
      'CC=@0@'.format(compiler.get_id()),
      'CFLAGS=-DEMACS_MESONBUILD -I@0@'.format(build_root), #CFLAGS applies to both (Obj)C and (Obj)C++ #TODO get meson applied cflags pragmatically
      autoconf_opts,
      check: true)
  run_command(make, '-C', build_root / 'lib', 'built_sources', check: true)
  message('Configured gnulib')
endif

lib_build_dir = meson.current_build_dir()

libgnu_tgt = custom_target(
    'libgnu',
    output: 'libgnu.a',
    input: [],
    command : [make, '-C', lib_build_dir, 'libgnu.a'],
    env: {},
    build_by_default: false,
)

libegnu_tgt = custom_target(
    'libegnu',
    output: 'libegnu.a',
    input: [],
    command : [make, '-C', lib_build_dir, 'libegnu.a'],
    env: {},
    build_by_default: false,
)
