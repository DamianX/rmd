
/proc/first_above(floor)
    for(. in list(1, 2, 3))
        if(. > floor)
            break

/proc/found()
    for(. in list("a", "b", "c"))
        if(. == "b")
            return

/proc/counted()
    for(. = 1 to 4)
        if(. == 3)
            return

var/global/spawned_total = 0

/proc/spawned()
    spawn(-1)
        for(. in list(1, 2, 3))
            spawned_total += .
    return spawned_total

/proc/test()
    return "[first_above(1)] [found()] [counted()] [spawned()]"
