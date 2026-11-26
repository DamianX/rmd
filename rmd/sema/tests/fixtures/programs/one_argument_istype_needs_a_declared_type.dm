/proc/istype(Val, Type)
	set __demir_intrin = "Istype"

/datum/thing

/proc/test()
	var/datum/thing/typed = new
	var/untyped = typed
	istype(null)
	return istype(typed) + istype(untyped)

/proc/const_param(const/datum/thing/held)
	return istype(held)
