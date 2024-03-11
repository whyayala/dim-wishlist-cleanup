basestat:total:>=68 
    or (
        basestat:total:>=65 (
            maxbasestatvalue:any 
            or basestat:any:>=27
            or stat:hunterpve:>=30 
            or stat:hunterpvp:>=30 
            or stat:lowmob:>=32
        )
    ) 
    or (
        (modslot:artifice or source:raid) basestat:total:>=63
        (
            maxbasestatvalue:any 
            or basestat:any:>=25
            or stat:hunterpve:>=28 
            or stat:hunterpvp:>=28
            or stat:lowmob:>=30
        )
    ) 