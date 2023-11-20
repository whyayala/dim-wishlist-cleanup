basestat:total:>=68 
    or (
        basestat:total:>=65 (
            maxbasestatvalue:any 
            or basestat:any:>=23 
            or stat:hunterpve:>=30 
            or stat:hunterpvp:>=30
            or stat:hunterlowmob:>=30
            or stat:titan:>=30 
            or stat:warlock:>=30 
        )
    )
    or (
        (modslot:artifice or source:raid) basestat:total:>=62
        (
            maxbasestatvalue:any 
            or basestat:any:>=20
            or stat:hunterlowmob:>=27
            or stat:hunterpve:>=27 
            or stat:hunterpvp:>=27 
            or stat:titan:>=27
            or stat:warlock:>=27
        )
    )
    or (
        (source:raid or source:ironbanner or modslot:artifice) is:classitem 
    )