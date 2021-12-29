if window_system != 'nextstep'
  subdir_done()
endif

add_languages('objc', required: true)

if host_system != 'darwin'
  error(f'Window system @window_system@ is not supported on your system @host_system@' )
endif

supported_features = [
  'svg', 'webp', 'imagemagick', 'gif',
  'jpeg', 'tiff', 'png', 'native-image-api',
  'xwidgets',
  'toolkit-scroll-bars',
]

term_header='nsterm.h'
cfg_data.set('HAVE_NS', 1,
             description : 'Define to 1 if you are using the NeXTstep API, either GNUstep or Cocoa on macOS.')

macos_frameworks = [
  'AppKit',
]
# TODO check NS_IMPL_COCOA
macos_frameworks += ['IOKit', 'Carbon', 'IOSurface', 'QuartzCore']
temacs_deps += [dependency('appleframeworks', modules: macos_frameworks)]
