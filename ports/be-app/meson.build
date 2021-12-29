if window_system != 'be-app'
  subdir_done()
endif

add_languages('cpp', required: true)

supported_features = [
  'harfbuzz',
  'svg', 'webp', 'imagemagick', 'gif',
  'jpeg', 'tiff', 'png', 'native-image-api', 'libotf',
  'toolkit-scroll-bars',
]

gtk3_required = get_option('xwidgets').enabled()
cairo_required = true
freetype_required = true

term_header='haikuterm.h'
cfg_data.set('HAVE_HAIKU', 1,
             description : 'Define if Emacs will be built with Haiku windowing support')
