# Does the opsystem file prohibit the use of the GNU malloc?
# Assume not, until told otherwise.
GNU_MALLOC=true
# whether malloc is Doug Lea style
doug_lea_malloc = compiler.compiles('''
   #include <malloc.h>
   static void hook (void) {}
   int main() {
   malloc_set_state (malloc_get_state ());
   __after_morecore_hook = hook;
   __malloc_initialize_hook = hook;
   }''', name : 'doug_lea_malloc')
hybrid_malloc=false
system_malloc=true

GMALLOC_OBJ=false
HYBRID_MALLOC=false
if system_malloc
  cfg_data.set('SYSTEM_MALLOC', 1,
               description : '''Define to 1 to use the system memory allocator, even if it is not
     Doug Lea style.''')

  GNU_MALLOC=false
  GNU_MALLOC_reason='''
    (The GNU allocators don't work with this system configuration.)'''
  VMLIMIT_OBJ=''
elif hybrid_malloc
  cfg_data.set('HYBRID_MALLOC', 1,
               description : 'Define to use gmalloc before dumping and the system malloc after.')
  HYBRID_MALLOC=true
  GNU_MALLOC=false
  GNU_MALLOC_reason=' (only before dumping)'
  GMALLOC_OBJ='gmalloc.o'
  VMLIMIT_OBJ=''
else
  if doug_lea_malloc
    GMALLOC_OBJ='gmalloc.o'
  endif
  VMLIMIT_OBJ='vm-limit.o'
  if compiler.check_header('sys/vlimit.h')
    arg_name = 'HAVE_' + 'sys/vlimit.h'.underscorify().to_upper()
    cfg_data.set(arg_name, 1)
  endif
  if compiler.links('''
   extern char data_start[]; char ch;
   int main() {
   return data_start < &ch;
   }''', name : 'data_start')
    cfg_data.set('HAVE_DATA_START', 1,
                 description : '''Define to 1 if data_start is the address of the start
       of the main data segment.''')
  endif
endif

if doug_lea_malloc and hybrid_malloc
  if GNU_MALLOC
    GNU_MALLOC_reason='''
      (Using Doug Lea's new malloc from the GNU C Library.)'''
  endif
  cfg_data.set('DOUG_LEA_MALLOC', 1,
               description : '''Define to 1 if the system memory allocator is Doug Lea style,
     with malloc hooks and malloc_set_state.''')

  ## Use mmap directly for allocating larger buffers.
  ## FIXME this comes from src/s/{gnu,gnu-linux}.h:
  ## #ifdef DOUG_LEA_MALLOC; #undef REL_ALLOC; #endif
  ## Does the AC_FUNC_MMAP test below make this check unnecessary?
  if ['mingw32', 'gnu', 'linux'*].contains(host_machine.system())
    REL_ALLOC=false
  endif
endif
