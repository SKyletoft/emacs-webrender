if not get_option('dbus').allowed()
  subdir_done()
endif

dep_dbus = dependency('dbus-1', version : '>= 1.0', required: get_option('dbus'))

if not dep_dbus.found()
  subdir_done()
endif

cfg_data.set('HAVE_DBUS', 1,
             description : 'Define to 1 if using D-Bus.')
# dbus_watch_get_unix_fd has been introduced in D-Bus 1.1.1.
# dbus_type_is_valid and dbus_validate_* have been introduced in
# D-Bus 1.5.12.
foreach f : ['dbus_watch_get_unix_fd',
	     'dbus_type_is_valid',
	     'dbus_validate_bus_name',
             'dbus_validate_path',
	     'dbus_validate_interface',
	     'dbus_validate_member']
  if compiler.has_function(f, dependencies: dep_dbus)
    cfg_data.set('HAVE_' + f.underscorify().to_upper(), 1)
  endif
endforeach
temacs_deps += dep_dbus
