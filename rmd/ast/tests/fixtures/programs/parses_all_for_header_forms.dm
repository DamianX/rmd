
/proc/test()
    for(;;) break
    for(var/i; i < 10; i++) continue
    for(var/two, two < 2) continue
    for(i = 1 to 10 step 2) continue
    for(var/j in 1 to 5) continue
    for(var/key, value in list()) continue
    for(existing in world) continue
    for(. in world) continue
    for(. = 1 to 3) continue
