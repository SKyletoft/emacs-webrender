check_headers = [
  ['HAVE_ACLV_H', 'aclv.h'],
  ['HAVE_ACL_LIBACL_H', 'acl/libacl.h'],
  ['HAVE_ALLOCA_H', 'alloca.h', '''Define to 1 if you have 'alloca' after including <alloca.h>, a header that
   may be supplied by this distribution.'''],
  ['HAVE_BCRYPT_H', 'bcrypt.h'],
  ['HAVE_BYTESWAP_H', 'byteswap.h'],
  ['HAVE_COFF_H', 'coff.h'],
  ['HAVE_COM_ERR_H', 'com/err.h'],
  ['HAVE_DIRENT_H', 'dirent.h'],
  ['HAVE_EXECINFO_H', 'execinfo.h'],
  ['HAVE_GETOPT_H', 'getopt.h'],
  ['HAVE_GMP_GMP_H', 'gmp/gmp.h'],
  ['HAVE_GMP_H', 'gmp.h'],
  ['HAVE_IEEE754_H', 'ieee754.h'],
  ['HAVE_IFADDRS_H', 'ifaddrs.h'],
  ['HAVE_INTTYPES_H', 'inttypes.h'],
  ['HAVE_KERBEROSIV_KRB_H', 'kerberosiv/krb.h'],
  ['HAVE_KERBEROS_KRB_H', 'kerberos/krb.h'],
  ['HAVE_KRB5_H', 'krb5.h'],
  ['HAVE_KRB_H', 'krb.h'],
  ['HAVE_LIBGCCJIT_H', 'libgccjit.h'],
  ['HAVE_LIBGEN_H', 'libgen.h'],
  ['HAVE_LIBINTL_H', 'libintl.h'],
  ['HAVE_LIMITS_H', 'limits.h'],
  ['HAVE_LINUX_FILTER_H', 'linux/filter.h'],
  ['HAVE_LINUX_FS_H', 'linux/fs.h'],
  ['HAVE_LINUX_SECCOMP_H', 'linux/seccomp.h'],
  ['HAVE_MACHINE_SOUNDCARD_H', 'machine/soundcard.h'],
  ['HAVE_MACH_MACH_H', 'mach/mach.h'],
  ['HAVE_MAILLOCK_H', 'maillock.h'],
  ['HAVE_MALLOC_H', 'malloc.h'],
  ['HAVE_MALLOC_MALLOC_H', 'malloc/malloc.h'],
  ['HAVE_MINIX_CONFIG_H', 'minix/config.h'],
  ['HAVE_MINMAX_IN_LIMITS_H', 'minmax/in/limits.h'],
  ['HAVE_MINMAX_IN_SYS_PARAM_H', 'minmax/in/sys/param.h'],
  ['HAVE_MMSYSTEM_H', 'mmsystem.h'],
  ['HAVE_NET_IF_DL_H', 'net/if/dl.h'],
  ['HAVE_NET_IF_H', 'net/if.h'],
  ['HAVE_NLIST_H', 'nlist.h'],
  ['HAVE_PTHREAD_H', 'pthread.h'],
  ['HAVE_PTY_H', 'pty.h'],
  ['HAVE_PWD_H', 'pwd.h'],
  ['HAVE_SANITIZER_LSAN_INTERFACE_H', 'sanitizer/lsan_interface.h'],
  ['HAVE_SOUNDCARD_H', 'soundcard.h'],
  ['HAVE_STDINT_H', 'stdint.h'],
  ['HAVE_STDIO_EXT_H', 'stdio/ext.h'],
  ['HAVE_STDIO_H', 'stdio.h'],
  ['HAVE_STDLIB_H', 'stdlib.h'],
  ['HAVE_STRINGS_H', 'strings.h'],
  ['HAVE_STRING_H', 'string.h'],
  ['HAVE_SYS_ACL_H', 'sys/acl.h'],
  ['HAVE_SYS_BITYPES_H', 'sys/bitypes.h'],
  ['HAVE_SYS_CDEFS_H', 'sys/cdefs.h'],
  ['HAVE_SYS_FS_S5PARAM_H', 'sys/fs/s5param.h'],
  ['HAVE_SYS_FS_TYPES_H', 'sys/fs/types.h'],
  ['HAVE_SYS_INTTYPES_H', 'sys/inttypes.h'],
  ['HAVE_SYS_LOADAVG_H', 'sys/loadavg.h'],
  ['HAVE_SYS_MOUNT_H', 'sys/mount.h'],
  ['HAVE_SYS_PARAM_H', 'sys/param.h'],
  ['HAVE_SYS_RANDOM_H', 'sys/random.h'],
  ['HAVE_SYS_RESOURCE_H', 'sys/resource.h'],
  ['HAVE_SYS_SELECT_H', 'sys/select.h'],
  ['HAVE_SYS_SOCKET_H', 'sys/socket.h'],
  ['HAVE_SYS_SOUNDCARD_H', 'sys/soundcard.h'],
  ['HAVE_SYS_STATFS_H', 'sys/statfs.h'],
  ['HAVE_SYS_STAT_H', 'sys/stat.h'],
  ['HAVE_SYS_SYSINFO_H', 'sys/sysinfo.h'],
  ['HAVE_SYS_SYSTEMINFO_H', 'sys/systeminfo.h'],
  ['HAVE_SYS_TIME_H', 'sys/time.h'],
  ['HAVE_SYS_TYPES_H', 'sys/types.h'],
  ['HAVE_SYS_UN_H', 'sys/un.h'],
  ['HAVE_SYS_UTSNAME_H', 'sys/utsname.h'],
  ['HAVE_SYS_VFS_H', 'sys/vfs.h'],
  ['HAVE_SYS_VLIMIT_H', 'sys/vlimit.h'],
  ['HAVE_SYS_WAIT_H', 'sys/wait.h'],
  ['HAVE_TERM_H', 'term.h'],
  ['HAVE_UNISTD_H', 'unistd.h'],
  ['HAVE_UTIL_H', 'util.h'],
  ['HAVE_UTMP_H', 'utmp.h'],
  ['HAVE_VALGRIND_VALGRIND_H', 'valgrind/valgrind.h'],
  ['HAVE_VFORK_H', 'vfork.h'],
  ['HAVE_WCHAR_H', 'wchar.h'],
  ['HAVE_WINSOCK2_H', 'winsock2.h'],
  ['HAVE_WS2TCPIP_H', 'ws2tcpip.h'],
  ['XOS_NEEDS_TIME_H', 'needs/time.h'],
  ['_GL_REPLACE_IEEE754_H', 'gl/replace/ieee754.h'],
  ['_REGEX_INCLUDE_LIMITS_H', 'regex/include/limits.h'],
]

foreach h : check_headers
  if compiler.has_header(h.get(1))
    # 'HAVE_' + h.get(1).underscorify().to_upper()
    cfg_data.set(h.get(0), 1, description : h.get(2, ''))
    # else
    #   cfg_data.set(h.get(0), 0, description : h.get(2, ''))
  endif
endforeach

check_functions = [
  ['HAVE_ACCEPT4', 'accept4', '#include<sys/socket.h>'],
  # check token ['HAVE_ACCESS']
  # check token ['HAVE_ACLSORT']
  # check token ['HAVE_ACLX_GET']
  # check token ['HAVE_ACL_COPY_EXT_NATIVE']
  # check token ['HAVE_ACL_CREATE_ENTRY_NP']
  # check token ['HAVE_ACL_DELETE_DEF_FILE']
  # check token ['HAVE_ACL_DELETE_FD_NP']
  # check token ['HAVE_ACL_DELETE_FILE_NP']
  # check token ['HAVE_ACL_ENTRIES']
  # check token ['HAVE_ACL_EXTENDED_FILE']
  # check token ['HAVE_ACL_FIRST_ENTRY']
  # check token ['HAVE_ACL_FREE']
  # check token ['HAVE_ACL_FREE_TEXT']
  # check token ['HAVE_ACL_FROM_MODE']
  # check token ['HAVE_ACL_FROM_TEXT']
  # check token ['HAVE_ACL_GET_FD']
  # check token ['HAVE_ACL_GET_FILE']
  # check token ['HAVE_ACL_SET_FD']
  # check token ['HAVE_ACL_SET_FILE']
  # check token ['HAVE_ACL_TO_SHORT_TEXT']
  # check token ['HAVE_ACL_TRIVIAL']
  # check token ['HAVE_ACL_TYPE_EXTENDED']
  # check token ['HAVE_ALIGNED_ALLOC']
  ['HAVE_ALLOCA', 'alloca', '#include<alloca.h>'],
  # check token ['HAVE_ALSA']
  # check token ['HAVE_CANONICALIZE_FILE_NAME']
  # check token ['HAVE_CFMAKERAW']
  # check token ['HAVE_CFSETSPEED']
  ['HAVE_CLOCK_GETTIME', 'clock_gettime', '#include<time.h>'],
  ['HAVE_CLOCK_SETTIME', 'clock_settime', '#include<time.h>'],
  # check token ['HAVE_CYGWIN_O_PATH_BUG']
  # check token ['HAVE_C_VARARRAYS']
  # check token ['HAVE_DATA_START']
  # check token ['HAVE_DBUS']
  # check token ['HAVE_DBUS_TYPE_IS_VALID']
  # check token ['HAVE_DBUS_VALIDATE_BUS_NAME']
  # check token ['HAVE_DBUS_VALIDATE_INTERFACE']
  # check token ['HAVE_DBUS_VALIDATE_MEMBER']
  # check token ['HAVE_DBUS_VALIDATE_PATH']
  # check token ['HAVE_DBUS_WATCH_GET_UNIX_FD']
  # check token ['HAVE_DECL_ALARM']
  # check token ['HAVE_DECL_ALIGNED_ALLOC']
  # check token ['HAVE_DECL_CLEARERR_UNLOCKED']
  # check token ['HAVE_DECL_DIRFD']
  # check token ['HAVE_DECL_ECVT']
  # check token ['HAVE_DECL_EXECVPE']
  # check token ['HAVE_DECL_FCLOSEALL']
  # check token ['HAVE_DECL_FCVT']
  # check token ['HAVE_DECL_FDOPENDIR']
  # check token ['HAVE_DECL_FEOF_UNLOCKED']
  # check token ['HAVE_DECL_FERROR_UNLOCKED']
  # check token ['HAVE_DECL_FFLUSH_UNLOCKED']
  # check token ['HAVE_DECL_FGETS_UNLOCKED']
  # check token ['HAVE_DECL_FPUTC_UNLOCKED']
  # check token ['HAVE_DECL_FPUTS_UNLOCKED']
  # check token ['HAVE_DECL_FREAD_UNLOCKED']
  # check token ['HAVE_DECL_FWRITE_UNLOCKED']
  # check token ['HAVE_DECL_GCVT']
  # check token ['HAVE_DECL_GETCHAR_UNLOCKED']
  # check token ['HAVE_DECL_GETC_UNLOCKED']
  # check token ['HAVE_DECL_GETDTABLESIZE']
  # check token ['HAVE_DECL_ISBLANK']
  # check token ['HAVE_DECL_LOCALTIME_R']
  # check token ['HAVE_DECL_MEMMEM']
  # check token ['HAVE_DECL_MEMRCHR']
  # check token ['HAVE_DECL_PUTCHAR_UNLOCKED']
  # check token ['HAVE_DECL_PUTC_UNLOCKED']
  # check token ['HAVE_DECL_SECCOMP_FILTER_FLAG_TSYNC']
  # check token ['HAVE_DECL_SECCOMP_SET_MODE_FILTER']
  # check token ['HAVE_DECL_STRMODE']
  # check token ['HAVE_DECL_STRNLEN']
  # check token ['HAVE_DECL_STRTOIMAX']
  # check token ['HAVE_DECL_STRTOLL']
  # check token ['HAVE_DECL_TZNAME']
  # check token ['HAVE_DECL___FPENDING']
  ['HAVE_DIRFD', 'dirfd', '#include<dirent.h>'],
  # check token ['HAVE_DLADDR']
  # check token ['HAVE_DLFUNC']
  # check token ['HAVE_EACCESS']
  # check token ['HAVE_ENDGRENT']
  # check token ['HAVE_ENDPWENT']
  # check token ['HAVE_ENVIRON_DECL']
  # check token ['HAVE_EUIDACCESS']
  # check token ['HAVE_EXPLICIT_BZERO']
  # check token ['HAVE_EXPLICIT_MEMSET']
  ['HAVE_FACCESSAT', 'faccessat', '#include<unistd.h>'],
  # check token ['HAVE_FACL']
  ['HAVE_FCHDIR', 'fchdir', '#include<unistd.h>'],
  ['HAVE_FCHMOD', 'fchmod', '#include<sys/stat.h>'],
  ['HAVE_FCHMODAT', 'fchmodat', '#include<sys/stat.h>'],
  # check token ['HAVE_FCNTL']
  ['HAVE_FDOPENDIR', 'fdopendir', '#include<dirent.h>'],
  ['HAVE_FORK', 'fork', '#include<unistd.h>'],
  # check token ['HAVE_FREEIFADDRS']
  # check token ['HAVE_FREETYPE']
  ['HAVE_FSEEKO', 'fseeko', '#include<stdio.h>'],
  ['HAVE_FSTATAT', 'fstatat', '#include<unistd.h>'],
  ['HAVE_FSYNC', 'fsync', '#include<unistd.h>'],
  # check token ['HAVE_FT_FACE_GETCHARVARIANTINDEX']
  ['HAVE_FUTIMENS', 'futimens', '#include<sys/stat.h>'],
  ['HAVE_FUTIMES', 'futimes', '#include<sys/time.h>'],
  # check token ['HAVE_FUTIMESAT']
  ['HAVE_GAI_STRERROR', 'gai_strerror', '#include<netdb.h>'],
  # check token ['HAVE_GCONF']
  # check token ['HAVE_GETACL']
  # check token ['HAVE_GETADDRINFO_A']
  # check token ['HAVE_GETCWD']
  # check token ['HAVE_GETDTABLESIZE']
  # check token ['HAVE_GETGRENT']
  ['HAVE_GETGROUPS', 'getgroups', '#include<grp.h>'],
  # check token ['HAVE_GETHOSTNAME']
  # check token ['HAVE_GETIFADDRS']
  # check token ['HAVE_GETOPT_LONG_ONLY']
  ['HAVE_GETPAGESIZE', 'getpagesize', '#include<unistd.h>'],
  # check token ['HAVE_GETPT']
  ['HAVE_GETPWENT', 'getpwent', '#include<pwd.h>'],
  ['HAVE_GETRANDOM', 'getrandom', '#include<linux/random.h>'],
  # check token ['HAVE_GETRLIMIT']
  # check token ['HAVE_GETRUSAGE']
  # check token ['HAVE_GETSOCKNAME']
  ['HAVE_GETTIMEOFDAY', 'gettimeofday', '#include<sys/time.h>'],
  # check token ['HAVE_GET_CURRENT_DIR_NAME']
  # check token ['HAVE_GFILENOTIFY']
  # check token ['HAVE_GIF']
  # check token ['HAVE_GLIB']
  # check token ['HAVE_GNUTLS']
  # check token ['HAVE_GPM']
  # check token ['HAVE_GRANTPT']
  # check token ['HAVE_GSETTINGS']
  # check token ['HAVE_GTK3']
  # check token ['HAVE_GTK_FILE_SELECTION_NEW']
  # check token ['HAVE_GTK_WINDOW_SET_HAS_RESIZE_GRIP']
  # check token ['HAVE_HARFBUZZ']
  # check token ['HAVE_IMAGEMAGICK']
  # check token ['HAVE_IMAGEMAGICK7']
  # check token ['HAVE_INOTIFY']
  # check token ['HAVE_ISBLANK']
  # check token ['HAVE_ISWCTYPE']
  # check token ['HAVE_JPEG']
  # check token ['HAVE_JSON']
  ['HAVE_KQUEUE', 'kqueue', '#include<sys/event.h>'],
  # check token ['HAVE_KRB5_ERROR_E_TEXT']
  # check token ['HAVE_KRB5_ERROR_TEXT']
  # check token ['HAVE_LANGINFO_CODESET']
  # check token ['HAVE_LANGINFO__NL_PAPER_WIDTH']
  ['HAVE_LCHMOD', 'lchmod', '#include<sys/stat.h>'],
  # check token ['HAVE_LCMS2']
  # check token ['HAVE_LIBDGC']
  # check token ['HAVE_LIBGCCJIT']
  # check token ['HAVE_LIBGIT']
  # check token ['HAVE_LIBKSTAT']
  # check token ['HAVE_LIBLOCKFILE']
  # check token ['HAVE_LIBMAIL']
  # check token ['HAVE_LIBOTF']
  # check token ['HAVE_LIBPERFSTAT']
  # check token ['HAVE_LIBSELINUX']
  # check token ['HAVE_LIBSYSTEMD']
  # check token ['HAVE_LIBXML2']
  # check token ['HAVE_LIB_BCRYPT']
  # check token ['HAVE_LINUX_SYSINFO']
  # check token ['HAVE_LOCALTIME_INFLOOP_BUG']
  # check token ['HAVE_LOCALTIME_R']
  ['HAVE_LOG2', 'log2', '#include<math.h>'],
  # check token ['HAVE_LONG_LONG_INT']
  # check token ['HAVE_LRAND48']
  ['HAVE_LSTAT', 'lstat', '#include<sys/stat.h>'],
  ['HAVE_LUTIMES', 'lutimes', '#include<sys/time.h>'],
  # check token ['HAVE_M17N_FLT']
  # check token ['HAVE_MAGICKAUTOORIENTIMAGE']
  # check token ['HAVE_MAGICKEXPORTIMAGEPIXELS']
  # check token ['HAVE_MAGICKMERGEIMAGELAYERS']
  # check token ['HAVE_MAGICKRELINQUISHMEMORY']
  # check token ['HAVE_MBSTATE_T']
  ['HAVE_MEMMEM', 'memmem', '#include<string.h>'],
  # check token ['HAVE_MEMPCPY']
  ['HAVE_MEMRCHR', 'memrchr', '#include<string.h>'],
  # check token ['HAVE_MEMSET_S']
  # check token ['HAVE_MKOSTEMP']
  ['HAVE_MMAP', 'mmap', '#include<sys/mman.h>'],
  # check token ['HAVE_MODULES']
  # check token ['HAVE_NATIVE_COMP']
  # check token ['HAVE_NATIVE_IMAGE_API']
  # check token ['HAVE_NEARLY_WORKING_FCHMODAT']
  # check token ['HAVE_NEARLY_WORKING_UTIMENSAT']
  ['HAVE_NEWLOCALE', 'newlocale', '#include<locale.h>'],
  # check token ['HAVE_NS']
  # check token ['HAVE_NTGUI']
  # check token ['HAVE_OTF_GET_VARIATION_GLYPHS']
  # check token ['HAVE_OTF_KANNADA_BUG']
  # check token ['HAVE_PDUMPER']
  # check token ['HAVE_PERSONALITY_ADDR_NO_RANDOMIZE']
  ['HAVE_PIPE2', 'pipe2', '#include<fcntl.h>'],
  # check token ['HAVE_PNG']
  # check token ['HAVE_POSIX_MADVISE']
  ['HAVE_POSIX_MEMALIGN', 'posix_memalign', '#include<stdlib.h>'],
  # check token ['HAVE_POSIX_OPENPT']
  # check token ['HAVE_PROCFS']
  # check token ['HAVE_PSELECT']
  # check token ['HAVE_PSTAT_GETDYNAMIC']
  # check token ['HAVE_PTHREAD']
  ['HAVE_PTHREAD_SETNAME_NP', 'pthread_setname_np', '#include<pthread.h>'],
  # check token ['HAVE_PTHREAD_SETNAME_NP_1ARG']
  # check token ['HAVE_PTHREAD_SETNAME_NP_3ARG']
  ['HAVE_PTHREAD_SET_NAME_NP', 'pthread_set_name_np', '#include<pthread.h>'],
  ['HAVE_PTHREAD_SIGMASK', 'pthread_sigmask', '#include<signal.h>'],
  # check token ['HAVE_PTYS']
  # check token ['HAVE_RANDOM']
  # check token ['HAVE_RAWMEMCHR']
  ['HAVE_READLINK', 'readlink', '#include<unistd.h>'],
  ['HAVE_READLINKAT', 'readlinkat', '#include<unistd.h>'],
  # check token ['HAVE_REALPATH']
  # check token ['HAVE_RECVFROM']
  ['HAVE_RINT', 'rint', '#include<math.h>'],
  # check token ['HAVE_RSVG']
  # check token ['HAVE_SBRK']
  ['HAVE_SELECT', 'select', '#include<sys/select.h>'],
  # check token ['HAVE_SENDTO']
  # check token ['HAVE_SETDTABLESIZE']
  ['HAVE_SETITIMER', 'setitimer', '#include<sys/time.h>'],
  ['HAVE_SETLOCALE', 'setlocale', '#include<locale.h>'],
  # check token ['HAVE_SETRLIMIT']
  # check token ['HAVE_SHUTDOWN']
  # check token ['HAVE_SIG2STR']
  # check token ['HAVE_SIGDESCR_NP']
  # check token ['HAVE_SIGNED_SIG_ATOMIC_T']
  # check token ['HAVE_SIGNED_WCHAR_T']
  # check token ['HAVE_SIGNED_WINT_T']
  # check token ['HAVE_SIGSETJMP']
  # check token ['HAVE_SIGSET_T']
  ['HAVE_SNPRINTF', 'snprintf', '#include<stdio.h>'],
  # check token ['HAVE_SOCKETS']
  # check token ['HAVE_SOUND']
  # check token ['HAVE_STACK_OVERFLOW_HANDLING']
  # check token ['HAVE_STATACL']
  # check token ['HAVE_STATEMENT_EXPRESSIONS']
  # check token ['HAVE_STPCPY']
  ['HAVE_STRSIGNAL', 'strsignal', '#include<signal.h>'],
  ['HAVE_STRTOIMAX', 'strtoimax', '#include<inttypes.h>'],
  ['HAVE_STRTOLL', 'strtoll', '#include<stdlib.h>'],
  # check token ['HAVE_STRUCT_ATTRIBUTE_ALIGNED']
  # check token ['HAVE_STRUCT_DIRENT_D_TYPE']
  # check token ['HAVE_STRUCT_IFREQ_IFR_ADDR']
  # check token ['HAVE_STRUCT_IFREQ_IFR_ADDR_SA_LEN']
  # check token ['HAVE_STRUCT_IFREQ_IFR_BROADADDR']
  # check token ['HAVE_STRUCT_IFREQ_IFR_FLAGS']
  # check token ['HAVE_STRUCT_IFREQ_IFR_HWADDR']
  # check token ['HAVE_STRUCT_IFREQ_IFR_NETMASK']
  # check token ['HAVE_STRUCT_NLIST_N_UN_N_NAME']
  # check token ['HAVE_STRUCT_STAT_ST_ATIMENSEC']
  # check token ['HAVE_STRUCT_STAT_ST_ATIMESPEC_TV_NSEC']
  # check token ['HAVE_STRUCT_STAT_ST_ATIM_ST__TIM_TV_NSEC']
  # check token ['HAVE_STRUCT_STAT_ST_ATIM_TV_NSEC']
  # check token ['HAVE_STRUCT_STAT_ST_BIRTHTIMENSEC']
  # check token ['HAVE_STRUCT_STAT_ST_BIRTHTIMESPEC_TV_NSEC']
  # check token ['HAVE_STRUCT_STAT_ST_BIRTHTIM_TV_NSEC']
  # check token ['HAVE_STRUCT_TM_TM_ZONE']
  # check token ['HAVE_STRUCT_UNIPAIR_UNICODE']
  ['HAVE_SYMLINK', 'symlink', '#include<unistd.h>'],
  ['HAVE_SYNC', 'sync', '#include<unistd.h>'],
  # check token ['HAVE_TIFF']
  ['HAVE_TIMEGM', 'timegm', '#include<time.h>'],
  # check token ['HAVE_TIMERFD']
  # check token ['HAVE_TIMER_GETOVERRUN']
  # check token ['HAVE_TIMER_SETTIME']
  # check token ['HAVE_TIMEZONE_T']
  # check token ['HAVE_TM_GMTOFF']
  # check token ['HAVE_TM_ZONE']
  # check token ['HAVE_TOUCHLOCK']
  # check token ['HAVE_TRUNC']
  # check token ['HAVE_TYPEOF']
  ['HAVE_TZNAME', 'tzname', '#include<time.h>'],
  # check token ['HAVE_UNEXEC']
  # check token ['HAVE_UNSIGNED_LONG_LONG_INT']
  ['HAVE_UTIMENSAT', 'utimensat', '#include<sys/stat.h>'],
  ['HAVE_VFORK', 'vfork', '#include<unistd.h>'],
  # check token ['HAVE_W32NOTIFY']
  # check token ['HAVE_WCHAR_T']
  # check token ['HAVE_WINDOW_SYSTEM']
  # check token ['HAVE_WORKING_FORK']
  # check token ['HAVE_WORKING_FSTATAT_ZERO_FLAG']
  # check token ['HAVE_WORKING_UTIMES']
  # check token ['HAVE_WORKING_VFORK']
  # check token ['HAVE_X11']
  # check token ['HAVE_X11R6']
  # check token ['HAVE_X11R6_XIM']
  # check token ['HAVE_X11XTR6']
  # check token ['HAVE_XAW3D']
  # check token ['HAVE_XDBE']
  # check token ['HAVE_XFIXES']
  # check token ['HAVE_XFT']
  # check token ['HAVE_XIM']
  # check token ['HAVE_XINERAMA']
  # check token ['HAVE_XKB']
  # check token ['HAVE_XPM']
  # check token ['HAVE_XRANDR']
  # check token ['HAVE_XRENDER']
  # check token ['HAVE_XRMSETDATABASE']
  # check token ['HAVE_XSCREENNUMBEROFSCREEN']
  # check token ['HAVE_XSCREENRESOURCESTRING']
  # check token ['HAVE_XWIDGETS']
  # check token ['HAVE_X_I18N']
  # check token ['HAVE_X_SM']
  # check token ['HAVE_X_WINDOWS']
  # check token ['HAVE_ZLIB']
  # check token ['HAVE__SETJMP']
  # check token ['HAVE___BUILTIN_EXPECT']
  # check token ['HAVE___BUILTIN_EXPECT']
  # check token ['HAVE___BUILTIN_EXPECT']
  # check token ['HAVE___BUILTIN_FRAME_ADDRESS']
  # check token ['HAVE___BUILTIN_UNWIND_INIT']
  # check token ['HAVE___EXECUTABLE_START']
  # check token ['HAVE___INLINE']
  # check token ['HAVE___LSAN_IGNORE_OBJECT']
]

foreach f : check_functions
  if compiler.has_function(f.get(1), prefix : f.get(2))
    cfg_data.set(f.get(0), 1)
  endif
endforeach

if compiler.compiles('''
   #include <sys/personality.h>
   int main() {
   personality (personality (0xffffffff) | ADDR_NO_RANDOMIZE);
   }''', name : 'personality_addr_no_randomize available')

  cfg_data.set(
    'HAVE_PERSONALITY_ADDR_NO_RANDOMIZE', 1,
    description : 'Define to 1 if personality flag ADDR_NO_RANDOMIZE exists.')

endif

# Note that Solaris has sys/sysinfo.h which defines struct
# sysinfo as well.  To make sure that we're using GNU/Linux
# sysinfo, we explicitly set one of its fields.
if compiler.has_header('sys/sysinfo.h')
  if compiler.compiles(
    '''#include <sys/sysinfo.h>
      int main() {
      struct sysinfo si;
      si.totalram = 0;
      sysinfo (&si);}''',
    name : 'linux_sysinfo available')

    cfg_data.set(
      'HAVE_LINUX_SYSINFO', 1,
      description : 'Define to 1 if you have Linux sysinfo function.')

    if compiler.compiles(
      '''#include <sys/sysinfo.h>
      int main() {
      struct sysinfo si;
      return si.mem_unit;}''',
      name : 'linux_sysinfo_unit available')

      cfg_data.set(
        'LINUX_SYSINFO_UNIT', 1,
        description : 'Define to 1 if Linux sysinfo sizes are in multiples of mem_unit bytes.')

    endif
  endif
endif

# On Solaris 8 there's a compilation warning for term.h because
# it doesn't define 'bool'.
if compiler.compiles('''
   #include <term.h>
   int main() {}''', name : 'personality_addr_no_randomize available')
  cfg_data.set(
    'HAVE_TERM_H', 1,
    description : 'Define to 1 if you have the <term.h> header file.')
endif

net_headers = [
  'net/if.h',
  'ifaddrs.h',
  'net/if_dl.h'
]

foreach h : net_headers
  if compiler.check_header(h, prefix: '''
    #if HAVE_SYS_SOCKET_H
    #include <sys/socket.h>
    #endif''', args: compiler_args)
    arg_name = 'HAVE_' + h.underscorify().to_upper()
    cfg_data.set(arg_name, 1)
    compiler_args += ('-D' + arg_name)
  endif
endforeach

struct_ifreq_members = [
  'struct ifreq.ifr_flags',
  'struct ifreq.ifr_hwaddr',
  'struct ifreq.ifr_netmask',
  'struct ifreq.ifr_broadaddr',
  'struct ifreq.ifr_addr',
  'struct ifreq.ifr_addr.sa_len'
]

foreach member : struct_ifreq_members
  tmp_array = member.split('.')
  membername = tmp_array.get(tmp_array.length() - 1)
  typename = member.replace('.' + membername, '')
  if compiler.has_member(typename, membername, prefix: '''
    #if HAVE_SYS_SOCKET_H
    #include <sys/socket.h>
    #endif
    #if HAVE_NET_IF_H
    #include <net/if.h>
    #endif''', args: compiler_args)
    cfg_data.set('HAVE_' + member.underscorify().to_upper(), 1)
  endif
endforeach

# whether addresses are sanitized
sanitize_address = compiler.compiles('''
   int main() {
   #ifndef __has_feature
   #define __has_feature(f) 0
   #endif
   #if defined __SANITIZE_ADDRESS__ || __has_feature (address_sanitizer)
   #else
      error "Addresses are not sanitized.";
   #endif
   }''', name : 'sanitize_address')

dep_anl = compiler.find_library('anl', required : false)
if dep_anl.found() and compiler.has_function('getaddrinfo_a', dependencies: dep_anl)
  cfg_data.set('HAVE_GETADDRINFO_A', 1,
               description : 'Define to 1 if you have getaddrinfo_a for asynchronous DNS resolution.')
  temacs_deps += dep_anl
endif
