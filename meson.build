project(
  'GNU Emacs',
  ['c'],
  default_options: [
    'c_std=gnu11',
    # 'c_std=gnu99',
  ],
  version : '29.0.50'
)

# add_global_arguments('-Demacs', language : 'c')
add_global_arguments('-DEMACS_MESONBUILD', language : 'c')

version       = meson.project_version()
major_version = version.split('.')[0].to_int()
minor_version = version.split('.')[1].to_int()
micro_version = version.split('.')[2].to_int()
compiler = meson.get_compiler('c')
# host = '@0@-@1@-@2@'.format(host_machine.cpu_family(), build_machine.system(), host_machine.system())
host_system = host_machine.system()

add_languages('cpp', required: true)

# message(compiler.get_id())
# message(compiler.get_linker_id())

if host_system == 'windows'
  shlext = '.dll'
  exeext = '.exe'
else
  shlext = '.so'
  exeext = ''
endif

compiler_args = []
temacs_deps = [
  compiler.find_library('m'),
  compiler.find_library('rt', required : false),
  compiler.find_library('dl', required : false),
]
temacs_objects = []
temacs_sources = []
cfg_data = configuration_data()
inc_dirs = [include_directories('.'), include_directories('lib'), include_directories('src')]
env = environment()
awk = find_program(['gawk', 'mawk', 'nawk', 'awk'], required : true)
make = find_program(['make'], required : true)
cat = find_program(['cat'], required : true)
env.set('AWK', awk.full_path())
fs = import('fs')
is_debug = get_option('buildtype').startswith('debug')
is_release = get_option('buildtype') == 'release'

build_root = meson.build_root()
source_root = meson.source_root()
env.set('MESON_SOURCE_ROOT', source_root)
env.set('MESON_BUILD_ROOT', source_root)

BUILD_DETAILS=get_option('build-details')
enable_gtk_deprecation_warnings=get_option('gtk-deprecation-warnings')

subdir('build/compiler')
subdir('build/rust')
subdir('build/mail')
subdir('build/wide-int')
subdir('build/dumping')
subdir('build/checking')
subdir('build/malloc')
subdir('build/term')
subdir('build/sound')
subdir('build/json')
subdir('build/lcms2')
subdir('build/selinux')
subdir('build/dbus')
subdir('build/sqlite3')
subdir('build/xml2')
subdir('build/libsystemd')
subdir('build/gpm')
subdir('build/gnutls')
subdir('build/zlib')
subdir('build/threads')
subdir('build/modules')
subdir('build/comp')

subdir('lwlib')
subdir('msdos')
subdir('nextstep')
subdir('nt')
subdir('oldXMenu')
subdir('test')

subdir('lib')
subdir('ports')
subdir('etc/charsets')
subdir('lib-src')
subdir('leim')
subdir('src')
subdir('lisp')
subdir('etc')

configure_file(output : 'conf_meson.h',
               configuration : cfg_data)

summary({
  'Does Emacs use -lXpm?': cfg_data.get('HAVE_XPM', 0) == 1,
  'Does Emacs use -ljpeg?': cfg_data.get('HAVE_JPEG', 0) == 1,
  'Does Emacs use -ltiff?': cfg_data.get('HAVE_TIFF', 0) == 1,
  'Does Emacs use a gif library?': cfg_data.get('HAVE_GIF', 0) == 1, #$LIBGIF
  'Does Emacs use a png library?': cfg_data.get('HAVE_PNG', 0) == 1, #$LIBPNG
  'Does Emacs use -lrsvg-2?': cfg_data.get('HAVE_RSVG', 0) == 1,
  'Does Emacs use -lwebp?': cfg_data.get('HAVE_WEBP', 0) == 1,
  'Does Emacs use cairo?': cfg_data.get('USE_CAIRO', 0) == 1,
  'Does Emacs use imagemagick?': cfg_data.get('HAVE_IMAGEMAGICK', 0) == 1,
  'Does Emacs use native APIs for images?': cfg_data.get('HAVE_NATIVE_IMAGE_API', 0) == 1,
  'Does Emacs use -lfreetype??': cfg_data.get('HAVE_FREETYPE', 0) == 1,
  'Does Emacs use HarfBuzz?': cfg_data.get('HAVE_HARFBUZZ', 0) == 1,
  'Does Emacs use -lotf?': cfg_data.get('HAVE_LIBOTF', 0) == 1,
  'Does Emacs use -lxft?': cfg_data.get('HAVE_XFT', 0) == 1,
  'Does Emacs use -lm17n-flt?': cfg_data.get('HAVE_M17N_FLT', 0) == 1,
  'Does Emacs support Xwidgets?': cfg_data.get('HAVE_XWIDGETS', 0) == 1,
  'Does Emacs use toolkit scroll bars?': cfg_data.get('USE_TOOLKIT_SCROLL_BARS', 0) == 1,
}, section: 'Window system')

summary({
  'Does Emacs use -lsqlite3?': cfg_data.get('HAVE_SQLITE3', 0) == 1,
  'Does Emacs use -llcms2?': cfg_data.get('HAVE_LCMS2', 0) == 1,
  'Does Emacs support sound?': cfg_data.get('HAVE_SOUND', 0) == 1,
  'Does Emacs use -lgpm?': cfg_data.get('HAVE_GPM', 0) == 1,
  'Does Emacs use -ldbus?': cfg_data.get('HAVE_DBUS', 0) == 1,
  'Does Emacs use -lgconf?': cfg_data.get('HAVE_GCONF', 0) == 1,
  'Does Emacs use GSettings?': cfg_data.get('HAVE_GSETTINGS', 0) == 1,
  'Does Emacs use a file notification library?': cfg_data.get('NOTIFY_SUMMARY', 0) == 1,
  'Does Emacs use access control lists?': cfg_data.get('ACL_SUMMARY', 0) == 1,
  'Does Emacs use -lselinux?': cfg_data.get('HAVE_LIBSELINUX', 0) == 1,
  'Does Emacs use -lgnutls?': cfg_data.get('HAVE_GNUTLS', 0) == 1,
  'Does Emacs use -lxml2?': cfg_data.get('HAVE_LIBXML2', 0) == 1,
  'Does Emacs use -lsystemd?': cfg_data.get('HAVE_LIBSYSTEMD', 0) == 1,
  'Does Emacs use -ljansson?': cfg_data.get('HAVE_JSON', 0) == 1,
  'Does Emacs use the GMP library?': cfg_data.get('HAVE_GMP', 0) == 1,
  'Does Emacs directly use zlib?': cfg_data.get('HAVE_ZLIB', 0) == 1,
  'Does Emacs have dynamic modules support?': cfg_data.get('HAVE_MODULES', 0) == 1,
  'Does Emacs have threading support in lisp?': cfg_data.get('threads_enabled', 0) == 1,
  'Does Emacs support the portable dumper?': cfg_data.get('with_pdumper', 0) == 1,
  'Does Emacs support legacy unexec dumping?': cfg_data.get('with_unexec', 0) == 1,
  'Which dumping strategy does Emacs use?': cfg_data.get('with_dumping', 0) == 1,
  'Does Emacs have native lisp compiler?': cfg_data.get('HAVE_NATIVE_COMP', 0) == 1,
  'Does Emacs use version 2 of the the X Input Extension?': cfg_data.get('HAVE_XINPUT2', 0) == 1,
}, section: 'Features')
