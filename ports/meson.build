window_system = get_option('window-system')
all_features = [
  'harfbuzz', 'svg', 'webp', 'imagemagick', 'gif', 'jpeg',
  'tiff', 'xpm', 'png', 'native-image-api', 'libotf', 'm17n-flt', 'xwidgets', 'toolkit-scroll-bars',
]
all_combo_features = ['x-toolkit']

supported_features = []
# $(MSDOS_OBJ) $(MSDOS_X_OBJ) $(NS_OBJ)
#$(CYGWIN_OBJ)
# $(W32_OBJ)
# $(NS_OBJC_OBJ)
window_system_sources = []

if get_option('window-system') == 'auto'
  if host_system == 'haiku'
    window_system = 'be-app'
  elif host_system == 'darwin' #TODO: exclude iOS here
    window_system = 'nextstep'
  elif host_system == 'windows'
    window_system = 'w32'
  elif ['dragonfly', 'freebsd', 'gnu', 'linux', 'netbsd', 'openbsd', 'sunos'].contains(host_system)
    window_system = 'x11'
  else # android , cygwin, emscripten
  endif
endif

summary('Window system', window_system, section: 'Window system')

## Common for all window systems
if window_system != 'none'
  cfg_data.set('HAVE_WINDOW_SYSTEM', 1,
               description : 'Define if you have a window system.')
  cfg_data.set('POLL_FOR_INPUT', 1,
               description : 'Define if you poll periodically to detect C-g.')

  window_system_sources += ['fontset.c', 'fringe.c', 'image.c']
endif
####

gtk3_required = false
cairo_required = false
freetype_required = false

subdir('be-app')
subdir('nextstep')
subdir('pgtk')
subdir('w32')
subdir('webrender')
subdir('x11')

foreach feature : all_features
  supported = supported_features.contains(feature)
  if get_option(feature).enabled()
    if not supported
      error(f'Current window system(@window_system@) does not support -D@feature@=enabled')
    endif
  endif
  set_variable(feature.underscorify() + '_opt', get_option(feature).disable_auto_if(not supported))
  unset_variable('supported')
endforeach

foreach feature : all_combo_features
  supported = supported_features.contains(feature)
  if not ['auto', 'none'].contains(get_option(feature))
    if not supported
      error(f'Current window system(@window_system@) does not support -D@feature@')
    endif
  endif
  set_variable(feature.underscorify() + '_opt', 'none')
  unset_variable('supported')
endforeach

if gtk3_required
  dep_glib = dependency('glib-2.0', version : '>= 2.37.5', required: gtk3_required)
  dep_gtk3 = dependency('gtk+-3.0', version : '>= 3.10', required: gtk3_required)

  if dep_glib.found() and dep_gtk3.found()
    cfg_data.set('HAVE_GTK3', 1,
                 description : 'Define to 1 if using GTK 3 or later.')
    temacs_deps += [dep_glib, dep_gtk3]
  endif

  if get_option('gtk-deprecation-warnings')
    cfg_data.set('GDK_DISABLE_DEPRECATION_WARNINGS', 1,
                 description : 'Define to 1 to disable GTK+/GDK deprecation warnings.')
    cfg_data.set('GLIB_DISABLE_DEPRECATION_WARNINGS', 1,
                 description : 'Define to 1 to disable Glib deprecation warnings.')
  endif
  # TODO check compiles and links
  cfg_data.set('USE_GTK', 1,
               description : 'Define to 1 if using GTK.')
endif

if cairo_required
  dep_cairo = dependency(
    'cairo', version : '>= 1.8.0',
    required: true,
    not_found_message: f'-Dwindow-system=@window_system@ requires cairo')

  if dep_cairo.found()
    cfg_data.set('USE_CAIRO', 1,
                 description : 'Define to 1 if using cairo.')
    temacs_deps += dep_cairo
  endif
endif

if freetype_required
  dep_fontconfig = dependency(
    'fontconfig', version : '>= 2.2.0',
    required: true,
    not_found_message: f'-Dwindow-system=@window_system@ requires libfontconfig')

  dep_freetype = dependency(
    'freetype2', version : '>= 2.5.0',
    required: true,
    not_found_message: f'-Dwindow-system=@window_system@ requires libfreetype')

  if dep_fontconfig.found() and dep_freetype.found()
    cfg_data.set('HAVE_FREETYPE', 1,
                 description : 'Define to 1 if using the freetype and fontconfig libraries.')
    temacs_deps += [dep_fontconfig, dep_freetype]
  endif
endif

if get_variable('toolkit_scroll_bars_opt', get_option('toolkit-scroll-bars').disable_auto_if(true)).allowed()
  if not (window_system == 'x11' and cfg_data.get('USE_X_TOOLKIT', 0) == 0)
    cfg_data.set('USE_TOOLKIT_SCROLL_BARS', 1,
                 description : 'Define to 1 if we should use toolkit scroll bars.')
  endif
endif

if get_variable('native_image_api_opt', get_option('native-image-api').disable_auto_if(true)).allowed()
  cfg_data.set('HAVE_NATIVE_IMAGE_API', 1,
               description : 'Define to use native OS APIs for images.')
endif

if get_variable('png_opt', get_option('png')).allowed()
  if host_machine.system() == 'mingw32' and
    have_png = compiler.check_header('png.h', required: get_option('png'))
  else
    dep_libpng = dependency('libpng', version : '>= 1.0.0', required: false)
    if not dep_libpng.found()
      # Test old way in case pkg-config doesn't have it (older machines).
      message('checking for libpng not configured by pkg-config ...')
      error('Not implemented, check ./configure.ac for detailes if you want to do the implementation')
      dep_libpng = disabler()
    endif
    if dep_libpng.found()
      have_png = true
      temacs_deps += dep_libpng
      have_png = true
    endif
  endif
  if get_variable('have_png', false)
    cfg_data.set('HAVE_PNG', 1,
                 description : 'Define to 1 if you have the png library.')
    if not compiler.has_header_symbol('png.h', 'png_longjmp', dependencies: dep_libpng)
      cfg_data.set('PNG_DEPSTRUCT', 1,
                   description : '''Define to empty to suppress deprecation warnings when building
	with --enable-gcc-warnings and with libpng versions before 1.5,
	which lack png_longjmp.''')
    endif
  endif
endif

if get_variable('jpeg_opt', get_option('jpeg')).allowed()
  dep_libjpeg = dependency('libjpeg', required: get_option('jpeg'))
  if dep_libjpeg.found()
    cfg_data.set('HAVE_JPEG', 1,
                 description : 'Define to 1 if you have the jpeg library (typically -ljpeg).')
    temacs_deps += dep_libjpeg
  endif
endif

if get_variable('tiff_opt', get_option('tiff')).allowed()
  dep_libtiff = dependency('libtiff-4')
  temacs_deps += dep_libtiff
  if host_machine.system() == 'mingw32' and
    have_tiff = compiler.check_header('tiffio.h', required: get_option('png'))
  else
    dep_libtiff = dependency('libtiff-4', required: get_option('tiff'))
    have_tiff = compiler.has_header('tiffio.h', dependencies: dep_libtiff)
  endif
  if get_variable('have_tiff', false)
    cfg_data.set('HAVE_TIFF', 1,
                 description : 'Define to 1 if you have the tiff library (-ltiff).')
  endif
endif

if get_variable('xpm_opt', get_option('xpm')).allowed()
  # AC_DEFINE(HAVE_XPM, 1, [Define to 1 if you have the Xpm library (-lXpm).])
  warning('todo')
endif

if get_variable('gif_opt', get_option('gif')).allowed()
  dep_gif = compiler.find_library(
    'gif',
    has_headers: ['gif_lib.h'],
    required : false)

  # EGifPutExtensionLast only exists from version libungif-4.1.0b1.
  # Earlier versions can crash Emacs, but version 5.0 removes EGifPutExtensionLast.
  if dep_gif.found() and (not compiler.has_function('GifMakeMapObject', dependencies: dep_gif))
    dep_gif = compiler.find_library(
      'ungif',
      required : get_option('gif'))
    if dep_gif.found() and
      (not compiler.has_function('EGifPutExtensionLast', dependencies: dep_gif))
      dep_gif = disabler()
    endif
  endif

  if dep_gif.found()
    cfg_data.set('HAVE_RSVG', 1,
                 description : 'Define to 1 if using librsvg.')
    temacs_deps += dep_gif
  endif
endif

if get_variable('svg_opt', get_option('svg')).allowed()
  dep_librsvg = dependency('librsvg-2.0', version : '>= 2.14.0', required: get_option('svg'))
  if dep_librsvg.found()
    cfg_data.set('HAVE_RSVG', 1,
                 description : 'Define to 1 if using librsvg.')
    temacs_deps += dep_librsvg
  endif
endif

if get_variable('webp_opt', get_option('webp')).allowed()
  dep_libwebp = dependency('libwebp', version : '>= 0.6.0', required: get_option('webp'))

  if dep_libwebp.found()
    cfg_data.set('HAVE_WEBP', 1,
                 description : 'Define to 1 if using libwebp.')
    temacs_deps += dep_libwebp
  endif
endif

if get_variable('imagemagick_opt', get_option('imagemagick')).allowed()
  # No need to require, there is imagemagick@6 alternative
  dep_imagemagick = dependency('MagickWand', version : '>= 7', required: false)
  if dep_imagemagick.found()
    cfg_data.set('HAVE_IMAGEMAGICK7', 1,
                 description : 'Define to 1 if using ImageMagick7.')
  else
    dep_imagemagick = dependency(
      'Wand',
      version : ['>= 6.3.5', '!= 6.8.2'],
      not_found_message: 'ImageMagick wanted, but it does not compile.  Maybe some library files are missing?',
      required: get_option('imagemagick'))

    if host_machine.system() != 'darwin' and
      find_program('brew', required : false).found()
      warning('''
      Homebrew doesn't link ImageMagick 6 by default, so make sure
      pkgconfig can find it.
      export PKG_CONFIG_PATH="$PKG_CONFIG_PATH$PATH_SEPARATOR`$BREW --prefix imagemagick@6 2>/dev/null`/lib/pkgconfig''')
    endif
  endif

  if dep_imagemagick.found()
    foreach f : [
      'MagickRelinquishMemory',
      'MagickExportImagePixels',
      'MagickMergeImageLayers',
      'MagickAutoOrientImage']
      if compiler.has_function(f, dependencies: dep_imagemagick)
        cfg_data.set('HAVE_' + f.underscorify().to_upper(), 1)
      else
        HAVE_IMAGEMAGICK = false
        break
      endif
    endforeach

    cfg_data.set('HAVE_IMAGEMAGICK', 1,
                 description : 'Define to 1 if using ImageMagick.')
    temacs_deps += dep_imagemagick
  endif
endif

if get_variable('libotf_opt', get_option('libotf')).allowed()
  dep_libotf = dependency('libotf', required: get_option('libotf'))

  if dep_libotf.found()
    cfg_data.set('HAVE_LIBOTF', 1,
                 description : 'Define to 1 if using libotf.')
    if compiler.has_function('OTF_get_variation_glyphs', dependencies: dep_libotf)
      cfg_data.set('HAVE_OTF_GET_VARIATION_GLYPHS', 1,
                   description : 'Define to 1 if libotf has OTF_get_variation_glyphs.')
    endif
    temacs_deps += dep_libotf
    if not dep_libotf.version().version_compare('>=0.9.16')
      cfg_data.set('HAVE_OTF_KANNADA_BUG', 1,
                   description : 'Define to 1 if libotf is affected by https://debbugs.gnu.org/28110.')
    endif
  endif
endif

if get_variable('m17n_flt_opt', get_option('m17n-flt')).allowed()
  dep_m17n_flt = dependency('m17n-flt', required: get_option('m17n-flt'))

  if dep_m17n_flt.found()
    cfg_data.set('HAVE_M17N_FLT', 1,
                 description : 'Define to 1 if using libm17n-flt.')
  endif
endif

if get_variable('harfbuzz_opt', get_option('harfbuzz')).allowed()
  harfbuzz_required_ver = window_system == 'w32' ? '1.2.3' : '0.9.42'
  dep_harfbuzz = dependency(
    'harfbuzz', version : f'>@harfbuzz_required_ver@',
    required: get_option('harfbuzz'))

  if dep_harfbuzz.found()
    cfg_data.set('HAVE_HARFBUZZ', 1,
                 description : 'Define to 1 if using HarfBuzz.')
    temacs_deps += dep_harfbuzz
  else
    warning('''This configuration uses the Cairo graphics library,
    but not the HarfBuzz font shaping library (minimum version @0@).
    We recommend the use of HarfBuzz when using Cairo, please install
    appropriate HarfBuzz development packages.'''.format(harfbuzz_required_ver))
  endif
endif

if get_variable('term_header', '') != ''
  if cfg_data.get('USE_GTK', 0) == 1
    term_header='gtkutil.h'
  endif
  cfg_data.set_quoted('TERM_HEADER', term_header,
                      description : 'Define to the header for the built-in window system.')
endif

if cfg_data.get('HAVE_X_WINDOWS', 0) == 1
  window_system_sources += 'xfont.c'
endif

if cfg_data.get('USE_CAIRO', 0) == 1
  if cfg_data.get('HAVE_FREETYPE', 0) == 1
    window_system_sources += 'ftfont.c'
  endif
  window_system_sources += 'ftcrfont.c'
endif

if cfg_data.get('HAVE_HARFBUZZ', 0) == 1
  window_system_sources += 'hbfont.c'
endif

if cfg_data.get('USE_GTK', 0) == 1
  window_system_sources += 'gtkutil.c'
endif

if cfg_data.get('HAVE_GTK3', 0) == 1
  window_system_sources += 'emacsgtkfixed.c'
endif
