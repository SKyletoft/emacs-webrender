if not get_option('native-compilation').enabled()
  subdir_done()
endif

if '{}-{}'.format(host_machine.cpu(), build_machine.system()) == 'i686-cygwin'
  warning('Native compilation is not supported on 32-bit Cygwin. Use it with caution')
endif

if cfg_data.get('HAVE_PDUMPER', 0) != 1
  error('-Dnative-compilation requires -Ddumping=pdumper')
endif

if cfg_data.get('HAVE_ZLIB', 0) != 1
  error('-Dnative-compilation requires -Dzlib=enabled')
endif

if host_system == 'darwin'
  # TODO Ensure libgccjit installed by Homebrew or macports can be found.
endif

dep_gccgit = compiler.find_library(
  'gccjit',
  has_headers: ['libgccjit.h'],
  required : get_option('native-compilation'))

if not dep_gccgit.found()
  error('libgccjit were not found.')
else
  result = compiler.run(
    '''#include <libgccjit.h>
      #include <stdlib.h>
      #include <stdio.h>
      int
      main (int argc, char **argv)
      {
        gcc_jit_context *ctxt;
        gcc_jit_result *result;
        ctxt = gcc_jit_context_acquire ();
        if (!ctxt)
          exit (1);
        gcc_jit_type *int_type =
          gcc_jit_context_get_type (ctxt, GCC_JIT_TYPE_INT);
        gcc_jit_function *func =
          gcc_jit_context_new_function (ctxt, NULL,
                                        GCC_JIT_FUNCTION_EXPORTED,
                                        int_type, "foo", 0, NULL, 0);
        gcc_jit_block *block = gcc_jit_function_new_block (func, "foo");
        gcc_jit_block_end_with_return (
          block,
          NULL,
          gcc_jit_context_new_rvalue_from_int (ctxt, int_type, 1));
        result = gcc_jit_context_compile (ctxt);
        if (!result)
          exit (1);
        typedef int (*fn_type) (void);
        fn_type foo =
          (fn_type)gcc_jit_result_get_code (result, "foo");
        if (!foo)
          exit (1);
        if (foo () != 1)
          exit (1);
        gcc_jit_context_release (ctxt);
        gcc_jit_result_release (result);
        return 0;
      }''',
    dependencies : [dep_gccgit],
    name : 'libgccjit_smoke_test')
  if result.returncode() == 0
    cfg_data.set(
      'HAVE_NATIVE_COMP', 1,
      description : 'Define to 1 if native compiler is available.')
    cfg_data.set_quoted(
      'NATIVE_ELISP_SUFFIX', '.eln',
      description : 'System extension for native compiled elisp')

    # TODO also check dl lib?
    temacs_deps += dep_gccgit
  else
    if not result.compiled()
      error('The installed libgccjit failed to compile a test program, @0@'.format(result.stderr()))
    else
      error('The installed libgccjit failed to run a test program, @0@'.format(result.stderr()))
    endif
  endif
endif
