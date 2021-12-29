lisp_source_dir = meson.current_source_dir()
lisp_build_dir = meson.current_build_dir()

cal_dir = lisp_build_dir / 'calendar'
tramp_dir = lisp_build_dir / 'net'
mh_e_dir = lisp_build_dir / 'mh-e'

foreach dir : [cal_dir, tramp_dir, mh_e_dir]
  if not fs.exists(dir)
    run_command('mkdir', '-p', dir, check: true)
  endif
endforeach

lisp_comp = [
'emacs-lisp/macroexp.el',
'emacs-lisp/cconv.el',
'emacs-lisp/byte-opt.el',
'emacs-lisp/bytecomp.el']
if get_variable('HAVE_NATIVE_COMP', false)
  lisp_comp += ['emacs-lisp/comp.el', 'emacs-lisp/comp-cstr.el']
endif
lisp_comp += 'emacs-lisp/autoload.el'

lisp_core = ['nxml/xsd-regexp.el']
# exclude lisp no been used when compiled, dir or lisp filename relative to lisp_source_dir
lisp_not_used = [
  'play',
  'erc',
  'org',
  'calc',
  'gnus/gnus-icalendar.el',
  'net/soap-inspect.el',
  'net/soap-client',
  'obsolete/bruce.el',
  'obsolete/erc-hecomplete.el',
  'obsolete/yow.el',
  # 'cedet/ede.el',
  # 'cedet/ede',
  'nxml'] # We can always easily fetch the needed on using straight.el
loaddefs = run_command(
    'find', '.', '-type', 'f',
    '-name', '*loaddefs.el',
    '!', '-name', '.*',
    check: true).stdout().strip().split('\n')
# Elisp files auto-generated.
lisp_auto_gen = ['cus-load.el', 'finder-inf.el',
'subdirs.el', 'eshell/esh-groups.el']
lisp_auto_gen += loaddefs

foreach f : run_command(
    'find', '.',
    '-type', 'f', '-name', '*.el',
    check: true).stdout().strip().split('\n')

  if f == ''
    break
  endif

  skip_p = false
  foreach f2 : lisp_not_used + lisp_auto_gen
    if f2 == ''
      break
    endif
    if fs.is_samepath(fs.parent(f), f2) or fs.is_samepath(f, f2)
      skip_p = true
    endif
  endforeach

  if not skip_p
    lisp_core += f
  endif
  skip_p = false
endforeach

foreach f_source : lisp_core
  if not fs.is_absolute(f_source)
    f_source = lisp_source_dir / f_source
  endif
  f_target = f_source.replace(lisp_source_dir, lisp_build_dir)
  dir_target = fs.parent(f_target)
  if not fs.exists(dir_target)
    run_command('mkdir', '-p', dir_target, check: true)
  endif
  run_command('ln', '-sf', f_source, f_target, check: true)
endforeach

update_subdirs_tgt = custom_target(
    'update-subdirs',
    output: ['update-subdirs.stamp'],
    command : [make, '-C', lisp_build_dir, 'update-subdirs'],
)

autoloads_tgt = custom_target(
    'autoloads',
    output: ['loaddefs.el'],
    depends: [emacs_tgt],
    command : [make, '-C', lisp_build_dir, 'autoloads'],
)

lisp_updates_dependencies = [autoloads_tgt, emacs_tgt]
if get_option('dumping') == 'pdumper'
  lisp_updates_dependencies += emacs_pdmp_tgt
endif
lisp_updates_tgt = custom_target(
    'updates',
    output: ['updates.stamp'],
    depends: lisp_updates_dependencies,
    command : [make, '-C', lisp_build_dir, 'updates'],
    build_always_stale: true,
    build_by_default: true
)
