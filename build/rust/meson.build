rust_required = get_option('window-system') == 'wr'

if not rust_required
  subdir_done()
endif

# rust = import('unstable-rust')
cargo = find_program('cargo')
cargo_args = []
cargo_target_path = meson.source_root() / 'target'
bindgen_extra_clang_args = [
  '-Demacs',
  '-DEMACS_EXTERN_INLINE',
  '-DEMACS_MESONBUILD',
  '-I' + meson.build_root(),
  '-I' + meson.source_root() / 'src',
  '-I' + meson.source_root() / 'lib',
  '-I' + meson.build_root() / 'src',
]

if is_release
  cargo_args += '--release'
  cargo_target_path = cargo_target_path / 'release'
else
  cargo_target_path = cargo_target_path / 'debug'
endif
