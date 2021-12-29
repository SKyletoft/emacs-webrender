if window_system != 'x11'
  subdir_done()
endif

supported_features = [
  'harfbuzz',
  'x-toolkit',
  'svg', 'webp', 'imagemagick', 'gif',
  'jpeg', 'tiff', 'xpm', 'png', 'xwidgets', 'libotf', 'm17n-flt',
  'toolkit-scroll-bars',
]

gtk3_required = get_option('xwidgets').enabled() or get_option('x-toolkit') == 'gtk3'

HAVE_X_WINDOWS=true
HAVE_X11=true
temacs_deps += [
  dependency('x11', required: true),
  dependency('xcb', required: true),
  dependency('xcb-xfixes', required: true),
  dependency('xcb-shape', required: true),
  dependency('xcb-render', required: true),
  dependency('x11-xcb', required: true),
  dependency('xrender', required: true)]

  # if ['athena', 'lucid'].contains(x_toolkit_opt)
  #   USE_X_TOOLKIT='LUCID'
  # elif x_toolkit_opt == 'motif'
  #   USE_X_TOOLKIT='MOTIF'
  # elif ['yes', 'gtk'].contains(x_toolkit_opt)
  #   with_gtk=true
  # elif x_toolkit_opt == 'gtk2'
  #   with_gtk2=true
  # elif x_toolkit_opt == 'gtk3'
  # endif

window_system_sources += [
    'xterm.c', 'xfns.c', 'xselect.c', 'xrdb.c', 'xsmfns.c', 'xsettings.c'
  ]
