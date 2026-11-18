/turf
	var/shared = "base"
	var/set_in_new
	var/new_loc_is_src

/turf/a
	shared = "a"

/turf/a/Del()
	for(var/obj/probe/probe in src)
		probe.deleted_type = type
	..()

/turf/b
	shared = "b"

/turf/b/New(loc)
	..()
	set_in_new = "ran"
	new_loc_is_src = loc == src

/area/zone

/obj/probe
	var/deleted_type

/datum/demir/replace

/datum/demir/replace/bake(atom/target)
	if(!istype(target, /obj/probe))
		return

	var/obj/probe/probe = target
	var/turf/old = locate(1, 1, 1)
	var/area/before = old.loc
	var/turf/made = new /turf/b(old)
	probe.icon_state = "[made == old]-[istype(old, /turf/b)]-[old.shared]-[old.set_in_new]-[old.new_loc_is_src]-[(probe in old.contents)]-[old.loc == before]-[probe.deleted_type]"

/datum/demir/ignored

/datum/demir/ignored/bake(atom/target)
	if(!istype(target, /obj/probe))
		return

	var/obj/probe/probe = target
	var/turf/old = locate(1, 1, 1)
	var/turf/from_obj = new /turf/b(probe)
	var/turf/bare = new /turf/b()
	probe.icon_state = "[from_obj == old]-[bare == old]-[istype(old, /turf/a)]-[old.shared]-[istype(from_obj, /turf/b)]-[istype(bare, /turf/b)]-[probe.deleted_type]"
