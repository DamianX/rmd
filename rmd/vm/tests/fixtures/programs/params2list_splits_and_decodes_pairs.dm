/proc/describe(value)
    if(isnull(value))
        return "null"
    if(islist(value))
        var/list/values = value
        var/out = "("
        for(var/i in 1 to values.len)
            out += "[i > 1 ? "|" : ""][describe(values[i])]"
        return out + ")"
    return "'[value]'"

/proc/show(text)
    var/list/params = params2list(text)
    var/out = "[params.len]:"
    for(var/key in params)
        out += " [describe(key)]=[describe(params[key])]"
    return out

/proc/test()
    return show("left=-14,0;right=13,0&flag&name=a+b%21&left=1,2")

/proc/test_repeated_keys()
    return "[show("a=1&a=2&a=3")], [show("a=1&b&a")], [show("A=1&a=2")]"

/proc/test_empty_parts()
    return "[show("")], [show("a=1&")], [show("a=1&&b=2")], [show("&a=1")], [show("a=b=c")], [show("flag=")]"
