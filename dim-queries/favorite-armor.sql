(
    basestat:total:>=68 (
        maxbasestatvalue:any 
        or basestat:any:>=30
    )
)
or (
    (modslot:artifice or source:raid) basestat:total:>=66 (
        maxbasestatvalue:any 
        or basestat:any:>=28
    )
)
or (
    (source:raid or source:ironbanner or modslot:artifice) is:classitem
)