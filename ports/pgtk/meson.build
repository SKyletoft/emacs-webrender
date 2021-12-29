if window_system != 'pgtk'
  subdir_done()
endif

supported_features = [
  'harfbuzz',
  'svg', 'webp', 'imagemagick', 'gif',
  'jpeg', 'tiff', 'png',
  'xwidgets', 'libotf',
  'toolkit-scroll-bars',
]

gtk3_required = true
cairo_required = true
freetype_required = true

cfg_data.set('HAVE_PGTK', 1,
             description : 'Define to 1 if you have pure Gtk+-3.')

term_header='gtkutil.h'

window_system_sources += [
    'pgtkfns.c', 'pgtkterm.c', 'pgtkselect.c', 'pgtkmenu.c', 'pgtkim.c', 'xsettings.c']
