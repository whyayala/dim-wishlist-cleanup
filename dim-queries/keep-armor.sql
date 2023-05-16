basestat:total:>=68 
    or (
        basestat:total:>=65 (
            maxbasestatvalue:any 
            or basestat:any:>=23 
            or stat:hunterpve:>=30 
            or stat:hunterpvp:>=30 
            or stat:titanpve:>=30 
            or stat:titanpvp:>=30 
            or stat:warlockpve:>=30 
            or stat:warlockpvp:>=30
        )
    )
    or (
        modslot:artifice basestat:total:>=62
        (
            maxbasestatvalue:any 
            or basestat:any:>=20
            or stat:hunterpve:>=27 
            or stat:hunterpvp:>=27 
            or stat:titanpve:>=27
            or stat:titanpvp:>=27
            or stat:warlockpve:>=27
            or stat:warlockpvp:>=27
        )
    )
    or (
        (source:raid or source:ironbanner or modslot:artifice) is:classitem 
    )