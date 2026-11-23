/proc/d(value)
	return isnull(value) ? "null" : "'[value]'"

// key and value after an associative loop
/proc/keyed_exhausted()
	var/k = "k0"
	var/v = "v0"
	for(k, v in list("a" = 1, "b" = 2))
		continue
	return "k=[d(k)] v=[d(v)]"
/proc/keyed_broke()
	var/k
	var/v
	for(k, v in list("a" = 1, "b" = 2))
		if(k == "b")
			break
	return "k=[d(k)] v=[d(v)]"
/proc/keyed_empty()
	var/k = "k0"
	var/v = "v0"
	for(k, v in list())
		continue
	return "k=[d(k)] v=[d(v)]"

// range loops
/proc/range_body_adds()
	var/i
	var/n = 0
	var/seen = ""
	for(i = 1 to 6)
		seen += "[i],"
		n++
		i += 1
	return "i=[d(i)] iterations=[n] seen=[seen]"
/proc/range_body_jumps()
	var/i
	var/n = 0
	for(i = 1 to 4)
		n++
		i = 10
	return "i=[d(i)] iterations=[n]"
/proc/range_body_rewinds()
	var/i
	var/n = 0
	for(i = 1 to 3)
		n++
		if(n < 6)
			i = 0
	return "i=[d(i)] iterations=[n]"
/proc/range_descending()
	var/i
	for(i = 4 to 1 step -1)
		continue
	return "i=[d(i)]"
/proc/range_empty()
	var/i = "init"
	for(i = 5 to 1)
		continue
	return "i=[d(i)]"
/proc/range_in_form()
	var/i = "init"
	for(i in 1 to 4)
		continue
	return "i=[d(i)]"
/proc/range_in_form_empty()
	var/i = "init"
	for(i in 5 to 1)
		continue
	return "i=[d(i)]"
/proc/range_fraction()
	var/i
	for(i = 1 to 2 step 0.4)
		continue
	return "i=[d(i)]"
/proc/range_end_changes()
	var/i
	var/n = 0
	var/end = 4
	for(i = 1 to end)
		n++
		end = 2
	return "i=[d(i)] iterations=[n]"
/proc/range_step_changes()
	var/i
	var/n = 0
	var/s = 1
	for(i = 1 to 8 step s)
		n++
		s = 3
	return "i=[d(i)] iterations=[n]"
/proc/range_broke()
	var/i
	for(i = 1 to 4)
		if(i == 2)
			break
	return "i=[d(i)]"
/proc/range_continue()
	var/i
	var/n = 0
	for(i = 1 to 4)
		if(i == 2)
			i = 3
			continue
		n++
	return "i=[d(i)] iterations=[n]"

// list loops
/proc/list_body_writes()
	var/x
	var/seen = ""
	for(x in list(1, 2, 3))
		seen += "[x],"
		x = 99
	return "x=[d(x)] seen=[seen]"
/proc/list_typed_filter()
	var/datum/x = "init"
	for(x in list(1, 2, 3))
		continue
	return "x=[d(x)]"
/proc/list_null_source()
	var/x = "init"
	var/list/L
	for(x in L)
		continue
	return "x=[d(x)]"

// the return value as the loop variable
/proc/dot_exhausted()
	for(. in list(1, 2, 3))
		continue
/proc/dot_broke()
	for(. in list(1, 2, 3))
		if(. == 2)
			break
/proc/dot_range()
	for(. = 1 to 4)
		continue
/proc/dot_results()
	return "exhausted=[d(dot_exhausted())] broke=[d(dot_broke())] range=[d(dot_range())]"

// targets that are not plain locals
/datum/loop_holder
	var/v = "v0"
	var/x = "field"
/datum/loop_holder/proc/var_key_declares_value()
	for(var/k, v in list("a" = 1))
		continue
	return "src.v=[d(src.v)]"
/datum/loop_holder/proc/bare_name_is_a_field()
	for(x in list(1, 2))
		continue
	return "src.x=[d(src.x)]"
/proc/keyed_dot_value()
	var/k
	var/seen = ""
	for(k, . in list("a" = 1, "b" = 2))
		seen += "[d(k)]=[d(.)],"
	return "seen=[seen] dot=[d(.)] k=[d(k)]"
/proc/keyed_field_value()
	var/datum/loop_holder/h = new
	var/k
	var/out = ""
	for(k, h.v in list("a" = 1, "b" = 2))
		out += "[d(k)]=[d(h.v)],"
	return "[out] after=[d(h.v)]"
/proc/field_list()
	var/datum/loop_holder/h = new
	var/out = ""
	for(h.v in list(1, 2))
		out += "[d(h.v)],"
	return "[out] after=[d(h.v)]"
/proc/field_range()
	var/datum/loop_holder/h = new
	var/out = ""
	for(h.v = 1 to 3)
		out += "[d(h.v)],"
	return "[out] after=[d(h.v)]"
/proc/var_key_declares_value()
	var/datum/loop_holder/h = new
	return h.var_key_declares_value()
/proc/bare_name_is_a_field()
	var/datum/loop_holder/h = new
	return h.bare_name_is_a_field()
