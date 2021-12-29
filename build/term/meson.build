tputs_libs = ['tinfo', 'ncurses', 'terminfo', 'curses', 'termcap', 'tinfow', 'ncursesw']
dep_tputs = disabler()
foreach tputs_lib_name : tputs_libs
  _dep_tputs = dependency(tputs_lib_name, required: false)
  if _dep_tputs.found() and compiler.links(
    '''extern void tputs (const char *, int, int (*)(int));
       int main (int argc, char **argv)
       {
	  if (argc == 10000)
	    tputs (argv[0], 0, 0);
	  return 0;
       }''', dependencies : [_dep_tputs], name : 'tputs_lib')
    dep_tputs = _dep_tputs
    break
  endif
endforeach

if not dep_tputs.found()
  error('''The required function 'tputs' was not found in any library.
  The following libraries were tried (in orde  r):
  libtinfo, libncurses, libterminfo, libcurses, libtermcap
  Please try installing whichever of these libraries is most appropriate
  for your system, together with its header files.
  For example, a libncurses-dev(el) or similar package.''')
endif

dep_tputs_name = dep_tputs.name()
if not ['termcap'].contains(dep_tputs_name)

  cfg_data.set('TERMINFO', 1,
               description : 'Define to 1 if you use terminfo instead of termcap.')

  # On some systems, the ncurses library is split in two libraries: ncurses and tinfo.
  # ncursesw is ncurses + Unicode (UTF8) support
  if ['tinfo', 'ncurses', 'tinfow', 'ncursesw'].contains(dep_tputs_name)
    cfg_data.set('USE_NCURSES', 1,
                 description : 'Define to 1 if you use ncurses.')
  endif

  if compiler.links('''
   extern char *BC;
   int main() {
   return !*BC;
   }''', name : f'whether @dep_tputs_name@ library defines BC', dependencies: dep_tputs)
    cfg_data.set('TERMINFO_DEFINES_BC', 1,
                 description : '''Define to 1 if the
      terminfo library defines the variables BC, PC, and UP.''')
  endif
endif

temacs_deps += dep_tputs
