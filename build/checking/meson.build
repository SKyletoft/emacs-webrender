if get_option('check-lisp-object-type')
  cfg_data.set('CHECK_LISP_OBJECT_TYPE', 1,
               description : 'Define to enable compile-time checks for the Lisp_Object data type.')
endif

enable_checking = false
CHECK_STRUCTS = false
gc_check_stringbytes= false
gc_check_string_overrun= false
gc_check_string_free_list= false
glyphs_debug= false
foreach flag : get_option('checking')
  if flag == 'yes'
    enable_checking = true
  elif flag == 'no'
    enable_checking = false
    CHECK_STRUCTS = false
    gc_check_stringbytes= false
    gc_check_string_overrun= false
    gc_check_string_free_list= false
    glyphs_debug= false
  elif flag == 'all'
    enable_checking=true
    CHECK_STRUCTS=true
    gc_check_stringbytes=true
    gc_check_string_overrun=true
    gc_check_string_free_list=true
    glyphs_debug=true
  elif flag == 'stringbytes'
    gc_check_stringbytes=true
  elif flag == 'stringoverrun'
    gc_check_string_overrun=true
  elif flag == 'stringfreelist'
    gc_check_string_free_list=true
  elif flag == 'structs'
    CHECK_STRUCTS=true
  elif flag == 'glyphs'
    glyphs_debug=true
  endif
endforeach
