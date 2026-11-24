/obj/thing

/obj/thing/sub

/obj/other

/proc/test()
	var/obj/thing/sub/S = new
	var/obj/thing/T = new
	var/obj/other/O = new
	return "[istype(T, null)] | [istype(S, T)] [istype(T, S)] [istype(S, O)] | [istype(T, 5)] [istype(T, "/obj/thing")]"
