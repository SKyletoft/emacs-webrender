gameuser=''
gamegroup=''
gameuser_opt_arr = get_option('gameuser').split(':')
if gameuser_opt_arr.length() > 0
  gameuser = gameuser_opt_arr.get(0)
  if gameuser == 'yes'
    gameuser = 'games'
  endif
  if gameuser_opt_arr.length() > 1
    gamegroup = gameuser_opt_arr.get(1)
  endif
endif
