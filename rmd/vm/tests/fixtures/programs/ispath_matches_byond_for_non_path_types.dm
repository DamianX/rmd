/obj/thing

/obj/thing/sub

/obj/other

/proc/test()
	var/obj/thing/sub/S = new
	var/obj/thing/T = new
	var/obj/other/O = new
	return "[ispath(/obj/thing)] [ispath(T)] | [ispath(/obj/thing, null)] | [ispath(/obj/thing/sub, T)] [ispath(/obj/thing, S)] [ispath(/obj/thing/sub, O)] | [ispath(/obj/thing, 5)] [ispath(/obj/thing, "/obj/thing")]"
