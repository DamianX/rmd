/datum/thing

/datum/other

/datum/holder
	var/datum/thing/owner

/datum/holder/proc/owns()
	return istype(owner)

var/datum/holder/H = new

/proc/typed_param(datum/thing/value)
	return istype(value)

/proc/test()
	H.owner = new /datum/thing
	var/datum/thing/value = new /datum/thing
	var/is_match = istype(value)
	value = new /datum/other
	var/is_mismatch = istype(value)
	var/list/datum/thing/things = list()
	return "[is_match] [is_mismatch] [istype(H.owner)] [typed_param(new /datum/thing)] [H.owns()] [istype(things)]"
