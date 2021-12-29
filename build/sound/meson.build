# also disable sound if auto_features is disabled
sound_option = \
  get_option('sound') == 'auto' and (not get_option('auto_features').allowed()) \
  ? 'none' : get_option('sound')

dep_sound = []
if sound_option == 'ossaudio' or sound_option == 'auto'
  ossaudio_dep = dependency(
    'ossaudio',
    method : 'pkg-config',
    not_found_message: 'ossaudio(bsd) sound support requested but not found.',
    required : sound_option == 'ossaudio')
  dep_sound += ossaudio_dep.found() ? ossaudio_dep : []
endif

if sound_option == 'alsa' or sound_option == 'auto'
  alsa_dep = dependency(
    'alsa',
    not_found_message: 'ALSA sound support requested but not found.',
    version : '>= 1.0.0',
    required : sound_option == 'alsa')
  dep_sound += alsa_dep.found() ? alsa_dep : []
endif

have_sound_header=false
if sound_option == 'oss' or sound_option == 'auto'
  sound_headers = [
    'machine/soundcard.h',
    'sys/soundcard.h',
    'soundcard.h',
    'mmsystem.h'
  ]

  foreach h : sound_headers
    have_sound_header = compiler.has_header(h, prefix: '''
    #ifdef __MINGW32__
    #define WIN32_LEAN_AND_MEAN
    #include <windows.h>
    #endif''')
    if have_sound_header
      break
    endif
  endforeach

  if sound_option == 'oss' and (not have_sound_header)
    error('OSS sound support requested but not found.')
  endif

endif

if dep_sound.length() > 0 or have_sound_header
  cfg_data.set('HAVE_SOUND', 1,
                 description : 'Define to 1 if you have sound support.')
  if (not have_sound_header) and (dep_sound.get(0).name() == 'alsa')
    cfg_data.set('HAVE_ALSA', 1,
                 description : 'Define to 1 if ALSA is available.')
  endif
  temacs_deps += dep_sound
endif
