if window_system != 'wr'
  subdir_done()
endif
supported_features = [
  # 'svg', 'webp', 'imagemagick', 'gif', 'jpeg', 'tiff', 'png', 'native-image-api'
]
freetype_required = true

term_header='wrterm.h'
cfg_data.set('HAVE_WR', 1,
             description : 'Define to 1 if you want to use the webrender.')

if ['linux','freebsd','netbsd','openbsd', 'dragonfly', 'gnu', 'haiku', 'sunos'].contains(host_system)
  temacs_deps += [
    dependency('xcb', required: true,
               not_found_message: '-Dwindow-system=wr requires libxcb on your system: @0@'.format(host_system)),
    dependency('xcb-xfixes', required: true),
    dependency('xcb-shape', required: true),
    dependency('xcb-render', required: true)
  ]
elif host_system == 'darwin'
  temacs_deps += [
    dependency(
        'appleframeworks',
        modules: ['AppKit', 'IOKit', 'Carbon', 'IOSurface', 'QuartzCore', 'OpenGL'])]
endif

#determin which dir contain platform library
if ['linux','android','darwin','cygwin','freebsd','netbsd','openbsd'].contains(host_machine.system())
  libwrterm_path = cargo_target_path / 'libwrterm.a'
else
  libwrterm_path = cargo_target_path / 'libwrterm.lib'
endif

wrterm_inc_dirs = [include_directories('.')]
inc_dirs += wrterm_inc_dirs

# message('Running Cargo check to update libwrterm')
# message('Done updating libwrterm')

bindgen_extra_clang_args += ('-I' + meson.current_source_dir())
# env.set('BINDGEN_EXTRA_CLANG_ARGS', ' '.join(bindgen_extra_clang_args))
wrterm_cargo_tgt = custom_target(
    'wrterm_cargo',
    output : 'wrterm_cargo.stamp',
    depends: [globals_h],
    command : [cargo, 'build', cargo_args],
    console: true,
    build_always: true,
    env: {
      'BINDGEN_EXTRA_CLANG_ARGS': ' '.join(bindgen_extra_clang_args)
    },
)

run_command('mkdir', '-p', fs.parent(libwrterm_path), check: true)
run_command('touch', libwrterm_path, check: true)
wrterm_tgt = custom_target(
    'libwrterm.a',
    output : 'libwrterm.a',
    input: libwrterm_path,
    depends: [wrterm_cargo_tgt],
    command : ['ln', '-s', '-f', libwrterm_path, '@OUTPUT@'],
    build_always: true, # for now, need to make sure that this target got run when libwrterm_path path/content changes
)

#mark cargo artifact to static library
# wrterm = static_library('wrterm', objects : [libwrterm_path], install : false)
libwrterm_all = static_library('wrterm_all', ['wrfns.c', globals_h],
                               objects : [],
                               include_directories: inc_dirs,
                               install : false)

dep_wrterm = declare_dependency(
link_with:[libwrterm_all, wrterm_tgt],
include_directories: wrterm_inc_dirs
)

# temacs_sources += wrterm_tgt
temacs_deps += dep_wrterm
