/datum/base
/datum/base/child
/proc/test()
    var/datum/base/value = /datum/base/child
    return istype(value) + istype(value, /datum/base) + istype(/datum/base/child, /datum/base)
