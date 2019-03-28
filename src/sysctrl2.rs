register_bit!(ChgOn, 0);
register_bit!(DsgOn, 1);
register_bit!(CcEn, 6);

register!(SysCtrl2, SYS_CTRL2, {
    chgon: ChgOn,
    dsgon: DsgOn,
    ccen: CcEn
});