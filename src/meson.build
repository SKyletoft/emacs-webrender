module_env_snippet_25=fs.read('module-env-25.h').strip('\n')
module_env_snippet_26=fs.read('module-env-26.h').strip('\n')
module_env_snippet_27=fs.read('module-env-27.h').strip('\n')
module_env_snippet_28=fs.read('module-env-28.h').strip('\n')
module_env_snippet_29=fs.read('module-env-29.h').strip('\n')

configure_file(
  input : 'emacs-module.h.in',
  output : 'emacs-module.h',
  configuration : {
    'emacs_major_version': major_version,
    'module_env_snippet_25': module_env_snippet_25,
    'module_env_snippet_26': module_env_snippet_26,
    'module_env_snippet_27': module_env_snippet_27,
    'module_env_snippet_28': module_env_snippet_28,
    'module_env_snippet_29': module_env_snippet_29
  }
)

makefile = configure_file(
  input : 'Makefile_meson.mk.in',
  output : 'Makefile_meson.mk',
  configuration : {
    'SHELL': '/bin/sh',
    'configure_input': 'src/Makefile_meson.  Generated from Makefile_meson.mk.in by meson.',
    'srcdir': meson.current_source_dir(),
    'abs_top_srcdir': source_root,
    'top_builddir': build_root,
    'top_srcdir': source_root,
    'HAVE_NATIVE_COMP': cfg_data.get('HAVE_NATIVE_COMP', 0) == 1 ? 'yes' : 'no',
    'AUTO_DEPEND': '',
    'BUILD_DETAILS': '',
    'CAIRO_CFLAGS': '',
    'CAIRO_LIBS': '',
    'CC': '',
    'CFLAGS': '',
    'CFLAGS_SOUND': '',
    'CHECK_STRUCTS': '',
    'CM_OBJ': '',
    'CPPFLAGS': '',
    'CXX': '',
    'CYGWIN_OBJ': '',
    'C_SWITCH_MACHINE': '',
    'C_SWITCH_SYSTEM': '',
    'C_SWITCH_X_SITE': '',
    'DBUS_CFLAGS': '',
    'DBUS_LIBS': '',
    'DBUS_OBJ': '',
    'DUMPING': get_option('dumping'),
    'DYNLIB_OBJ': '',
    'EMACSRES': '',
    'EXEEXT': '',
    'FIRSTFILE_OBJ': '',
    'FONTCONFIG_CFLAGS': '',
    'FONTCONFIG_LIBS': '',
    'FONT_OBJ': '',
    'FREETYPE_CFLAGS': '',
    'FREETYPE_LIBS': '',
    'GETADDRINFO_A_LIBS': '',
    'GETLOADAVG_LIBS': '',
    'GMALLOC_OBJ': '',
    'GNUSTEP_CFLAGS': '',
    'GNU_OBJC_CFLAGS': '',
    'GTK_OBJ': '',
    'HAIKU_CFLAGS': '',
    'HAIKU_CXX_OBJ': '',
    'HAIKU_LIBS': '',
    'HAIKU_OBJ': '',
    'HARFBUZZ_CFLAGS': '',
    'HARFBUZZ_LIBS': '',
    'HAVE_BE_APP': '',
    'HAVE_PDUMPER': '',
    'HYBRID_MALLOC': '',
    'IMAGEMAGICK_CFLAGS': '',
    'IMAGEMAGICK_LIBS': '',
    'JSON_CFLAGS': '',
    'JSON_LIBS': '',
    'JSON_OBJ': '',
    'LCMS2_CFLAGS': '',
    'LCMS2_LIBS': '',
    'LDFLAGS': '',
    'LD_SWITCH_SYSTEM': '',
    'LD_SWITCH_SYSTEM_TEMACS': '',
    'LD_SWITCH_X_SITE_RPATH': '',
    'LIBGCCJIT_CFLAGS': '',
    'LIBGCCJIT_LIBS': '',
    'LIBGIF': '',
    'LIBGMP': '',
    'LIBGNUTLS_CFLAGS': '',
    'LIBGNUTLS_LIBS': '',
    'LIBGPM': '',
    'LIBJPEG': '',
    'LIBMODULES': '',
    'LIBOBJS': '',
    'LIBOTF_CFLAGS': '',
    'LIBOTF_LIBS': '',
    'LIBPNG': '',
    'LIBS': '',
    'LIBSELINUX_LIBS': '',
    'LIBSOUND': '',
    'LIBSYSTEMD_CFLAGS': '',
    'LIBSYSTEMD_LIBS': '',
    'LIBS_GNUSTEP': '',
    'LIBS_SYSTEM': '',
    'LIBS_TERMCAP': '',
    'LIBTIFF': '',
    'LIBXMENU': '',
    'LIBXML2_CFLAGS': '',
    'LIBXML2_LIBS': '',
    'LIBXMU': '',
    'LIBXPM': '',
    'LIBXSM': '',
    'LIBXTR6': '',
    'LIBXT_OTHER': '',
    'LIBX_OTHER': '',
    'LIBZ': '',
    'LIB_ACL': '',
    'LIB_CLOCK_GETTIME': '',
    'LIB_EACCESS': '',
    'LIB_EXECINFO': '',
    'LIB_MATH': '',
    'LIB_PTHREAD': '',
    'LIB_TIMER_TIME': '',
    'M17N_FLT_CFLAGS': '',
    'M17N_FLT_LIBS': '',
    'MKDIR_P': '/usr/bin/mkdir -p',
    'MODULES_OBJ': '',
    'NOTIFY_CFLAGS': '',
    'NOTIFY_LIBS': '',
    'NOTIFY_OBJ': '',
    'NS_OBJ': '',
    'NS_OBJC_OBJ': '',
    'OTHER_FILES': '',
    'PAXCTL': '',
    'PAXCTL_dumped': '',
    'PAXCTL_notdumped': '',
    'PGTK_LIBS': '',
    'PGTK_OBJ': '',
    'PNG_CFLAGS': '',
    'POST_ALLOC_OBJ': '',
    'PRE_ALLOC_OBJ': '',
    'PROFILING_CFLAGS': '',
    'RALLOC_OBJ': '',
    'RSVG_CFLAGS': '',
    'RSVG_LIBS': '',
    'SETFATTR': '',
    'SETTINGS_CFLAGS': '',
    'SETTINGS_LIBS': '',
    'SQLITE3_LIBS': '',
    'SYSTEM_TYPE': '',
    'TERMCAP_OBJ': '',
    'TOOLKIT_LIBW': '',
    'UNEXEC_OBJ': '',
    'VMLIMIT_OBJ': '',
    'W32_LIBS': '',
    'W32_OBJ': '',
    'W32_RES_LINK': '',
    'WARN_CFLAGS': '',
    'WEBKIT_CFLAGS': '',
    'WEBKIT_LIBS': '',
    'WEBP_CFLAGS': '',
    'WEBP_LIBS': '',
    'WERROR_CFLAGS': '',
    'WIDGET_OBJ': '',
    'WINDOW_SYSTEM_OBJ': '',
    'XCB_LIBS': '',
    'XDBE_CFLAGS': '',
    'XDBE_LIBS': '',
    'XFIXES_CFLAGS': '',
    'XFIXES_LIBS': '',
    'XFT_LIBS': '',
    'XGSELOBJ': '',
    'XINERAMA_CFLAGS': '',
    'XINERAMA_LIBS': '',
    'XINPUT_CFLAGS': '',
    'XINPUT_LIBS': '',
    'XMENU_OBJ': '',
    'XOBJ': '',
    'XRANDR_CFLAGS': '',
    'XRANDR_LIBS': '',
    'XRENDER_LIBS': '',
    'XWIDGETS_OBJ': '',
    'configuration': '',
    'version': ''
  }
)

makefile_path = build_root / '@0@'.format(makefile)
cur_source_dir = meson.current_source_dir()
cur_build_dir = meson.current_build_dir()

sources = [
# $(FIRSTFILE_OBJ)
]
sources += [
# $(VMLIMIT_OBJ)
]
sources += [
  'dispnew.c',
  'frame.c',
  'scroll.c',
  'xdisp.c',
  'menu.c',
]

if window_system == 'x11'
  sources += [
    'xmenu.c'
  ]
endif

sources += [
  'window.c',
  'charset.c',
  'coding.c',
  'category.c',
  'ccl.c',
  'character.c',
  'chartab.c',
  'bidi.c',
]

#### CM_OBJ
sources += [
  'cm.c'
]
#### End CM_OBJ

sources += [
  'term.c',
  'terminal.c',
  'xfaces.c',
]

if cfg_data.get('HAVE_DBUS', 0) == 1
  sources += [
    'dbusbind.c'
  ]
endif

sources += [
  'emacs.c',
  'keyboard.c',
  'macros.c',
  'keymap.c',
  'sysdep.c',
  'bignum.c',
  'buffer.c',
  'filelock.c',
  'insdel.c',
  'marker.c',
  'minibuf.c',
  'fileio.c',
  'dired.c',
  'cmds.c',
  'casetab.c',
  'casefiddle.c',
  'indent.c',
  'search.c',
  'regex-emacs.c',
  'undo.c',
  'alloc.c',
  'pdumper.c',
  'data.c',
  'doc.c',
  'editfns.c',
  'callint.c',
  'eval.c',
  'floatfns.c',
  'fns.c',
  'font.c',
  'print.c',
  'lread.c',
]

if cfg_data.get('HAVE_MODULES', 0) == 1
  sources += [
    'emacs-module.c'
  ]
endif

sources += [
  'syntax.c',
  # $(UNEXEC_OBJ)
  'bytecode.c',
  'comp.c',
]

if cfg_data.get('HAVE_MODULES', 0) == 1 or cfg_data.get('HAVE_NATIVE_COMP', 0) == 1
  sources += [
    'dynlib.c'
  ]
endif

sources += [
  'process.c',
  'gnutls.c',
  'callproc.c',
  'region-cache.c',
  'sound.c',
  'timefns.c',
  'atimer.c',
  'doprnt.c',
  'intervals.c',
  'textprop.c',
  'composite.c',
  'xml.c',
  'lcms.c',
]

#### File notification FIXME
if cfg_data.get('USE_FILE_NOTIFY', 0) == 1
  sources += [
    'inotify.c'
  ]
endif
#### File notification

sources += [
  # $(XWIDGETS_OBJ)
  'profiler.c',
  'decompress.c',
  'thread.c',
  'systhread.c',
  'sqlite.c'
]

sources += [
  # $(if $(HYBRID_MALLOC),sheap.o)
]

sources += window_system_sources

sources += [
  'xgselect.c'
]

if cfg_data.get('HAVE_JSON', 0) == 1
  sources += [
    'json.c'
  ]
endif

sources += cfg_data.get('TERMINFO', 0) == 1 ? 'terminfo.c' : 'tparam.o'

sources += [
#$(PRE_ALLOC_OBJ) $(GMALLOC_OBJ) $(RALLOC_OBJ) \
#$(POST_ALLOC_OBJ) $(WIDGET_OBJ) $(LIBOBJS)
]

buildobj_h_sh = find_program(meson.source_root() / 'build-aux/buildobj_h.sh')
buildobj_h = custom_target(
    'buildobj.h',
    output : 'buildobj.h',
    command : [buildobj_h_sh, sources],
    capture: true
)

doc_sources = sources

cur_source_dir = meson.current_source_dir()
dmpstruct_headers = [cur_source_dir / 'lisp.h', cur_source_dir / 'buffer.h', cur_source_dir / 'intervals.h', cur_source_dir / 'charset.h', cur_source_dir / 'bignum.h']
dmpstruct_h = custom_target(
    'dmpstruct.h',
    output : 'dmpstruct.h',
    command : [awk, '-f', cur_source_dir / 'dmpstruct.awk', dmpstruct_headers],
    capture: true
)

globals_h_sources = sources
if window_system == 'wr'
  globals_h_sources += 'wrterm/wrfns.c'
endif

globals_h = custom_target(
    'globals.h',
    output : 'globals.h',
    input: globals_h_sources,
    depends: [make_docfile_tgt],
    command : [make_docfile_tgt, '-d', meson.current_source_dir(), '-g', '@INPUT@'],
    capture: true,
    # build_always: true
)

subdir('emacs-sys')

temacs_prerequisites_tgt = custom_target(
    'temacs_prerequisites',
    output : 'temacs_prerequisites.stamp',
    command : [make, '-C', meson.current_build_dir(), '-f', makefile_path, 'temacs' + exeext],
    console : true
)

temacs_tgt = executable(
    'temacs',
    sources : [
      sources,
      buildobj_h,
      globals_h,
      dmpstruct_h,
      temacs_sources,
      libgnu_tgt,
      temacs_prerequisites_tgt
    ],
    c_args : ['-Demacs'],
    include_directories : inc_dirs,
    dependencies : temacs_deps)

bootstrap_emacs_exe = 'bootstrap-emacs' + exeext
emacs_exe = 'emacs' + exeext
bootstrap_emacs_tgt = custom_target(
    'bootstrap-emacs',
    output : bootstrap_emacs_exe,
    depends: [temacs_tgt],
    command : [make, '-j12', '-C', meson.current_build_dir(), '-f', makefile_path, bootstrap_emacs_exe],
    console : true,
    env: env,
    build_always_stale: true,
    build_by_default: true
)

emacs_tgt = custom_target(
    'emacs',
    output : emacs_exe,
    depends: [temacs_tgt],
    command : [make, '-j12', '-C', meson.current_build_dir(), '-f', makefile_path, emacs_exe],
    console : true,
    env: env,
)

if get_option('dumping') == 'pdumper'
  emacs_pdmp_tgt = custom_target(
      'emacs_pdmp',
      output : 'emacs.pdmp',
      depends: [bootstrap_emacs_tgt, emacs_tgt],
      command : [make, '-j12', '-C', meson.current_build_dir(), '-f', makefile_path, 'emacs.pdmp'],
      console : true,
      env: env,
      build_always_stale: true,
      build_by_default: true
  )
endif
