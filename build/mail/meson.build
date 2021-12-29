# for retrieving mail, unencrypted network connections are the default
# only on native ms-windows platforms.  (fixme: these platforms should
# also be secure by default.)
if find_program(
  'movemail',
  required : get_option('mailutils')
).found() and run_command('movemail', '--version').returncode() == 0
endif

if get_option('pop').disable_auto_if(host_machine.system() != 'windows').allowed()
  cfg_data.set('MAIL_USE_POP', 1, description : 'Define to support POP mail retrieval.')
endif

cfg_data.set('KERBEROS', 0,
             description : 'Define to support Kerberos-authenticated POP mail retrieval.')

if get_option('kerberos').enabled()
  cfg_data.set('KERBEROS', 1)
endif

if get_option('kerberos5').enabled()
  cfg_data.set('KERBEROS', 1)
  cfg_data.set('KERBEROS5', 1,
                 description : 'Define to use Kerberos 5 instead of Kerberos 4.')
endif

if get_option('hesiod').enabled()
  cfg_data.set('HESIOD', 1,
                 description : 'Define to support using a Hesiod database to find the POP server.')
endif

if get_option('mail-unlink').enabled()
  cfg_data.set('MAIL_UNLINK_SPOOL', 1,
                 description : 'Define to unlink, rather than empty, mail spool after reading.')
endif

mailhost = get_option('mailhost')
if mailhost != ''
  cfg_data.set_quoted('MAILHOST', mailhost,
                     description : 'String giving fallback POP mail host.')
endif

if get_option('dumping') == 'pdumper'
  cfg_data.set('HAVE_PDUMPER', 1,
               description : 'Define to build with portable dumper support')
endif
