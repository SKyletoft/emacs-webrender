# callproc.c check NEWS existences to set Vdata_directory
run_command('touch', meson.current_build_dir() / 'NEWS', check: true)

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

DOC_C_tgt = custom_target(
    'DOC_C',
    output : 'DOC.1',
    depends: [make_docfile_tgt],
    command : [make_docfile_tgt, '-d', source_root / 'src', doc_sources],
    capture: true,
    # build_always: true
)

lisp_preloaded = ['loaddefs.el', 'loadup.el']
foreach filename : run_command('../build-aux/lisp_meson.sh', source_root / 'lisp/loadup.el', check: true).stdout().strip().split('\n')
  _lisp_excluded = ['leim/leim-list.el', 'site-load.elc', 'site-init.elc'] + lisp_preloaded
  if not _lisp_excluded.contains(filename)
    lisp_preloaded += filename
  endif
endforeach

DOC_lisp_tgt = custom_target(
    'DOC_lisp',
    output : 'DOC.2',
    depends: [make_docfile_tgt, autoloads_tgt],
    command : [make_docfile_tgt, '-d', build_root / 'lisp', lisp_preloaded],
    capture: true,
    # build_always: true
)

DOC_tgt = custom_target(
  'DOC',
  output : 'DOC',
  input: [DOC_C_tgt, DOC_lisp_tgt],
  command : [cat, '@INPUT@'],
  capture: true,
  build_by_default: true
)
