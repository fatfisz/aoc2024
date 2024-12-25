use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let input = input.trim();

    println!("{input}");

    let result = 0;

    println!("{result}");
}

// x00 x01 x02 x03 x04 x05 x06 x07 x08 x09 x10 x11 x12 x13 x14 x15 x16 x17 x18 x19 x20 x21 x22 x23 x24 x25 x26 x27 x28 x29 x30 x31 x32 x33 x34 x35 x36 x37 x38 x39 x40 x41 x42 x43 x44
// y00 y01 y02 y03 y04 y05 y06 y07 y08 y09 y10 y11 y12 y13 y14 y15 y16 y17 y18 y19 y20 y21 y22 y23 y24 y25 y26 y27 y28 y29 y30 y31 y32 y33 y34 y35 y36 y37 y38 y39 y40 y41 y42 y43 y44
//
// x00 x01 x02 x03 x04 x05 x06 x07 x08 x09 x10 x11 x12 x13 x14 x15 x16 x17 x18 x19 x20 x21 x22 x23 x24 x25 x26 x27 x28 x29 x30 x31 x32 x33 x34 x35 x36 x37 x38 x39 x40 x41 x42 x43 x44
// XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR
// y00 y01 y02 y03 y04 y05 y06 y07 y08 y09 y10 y11 y12 y13 y14 y15 y16 y17 y18 y19 y20 y21 y22 y23 y24 y25 y26 y27 y28 y29 y30 y31 y32 y33 y34 y35 y36 y37 y38 y39 y40 y41 y42 y43 y44
//  v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v
// z00 jnj wsm mgr mtb rqw kcc ncp twj jtc vmf ftq dbg dgr pgq shb grr rcc dwd ssf sqr nnr jck crt hbc rhk mcj fjc hdk ccs jnr tjk dfn ttn hnk jdq ckw gcg vsq gkk wqg hbk kcp dnf bmv
//
// x00 x01 x02 x03 x04 x05 x06 x07 x08 x09 x10 x11 x12 x13 x14 x15 x16 x17 x18 x19 x20 x21 x22 x23 x24 x25 x26 x27 x28 x29 x30 x31 x32 x33 x34 x35 x36 x37 x38 x39 x40 x41 x42 x43 x44
// AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND
// y00 y01 y02 y03 y04 y05 y06 y07 y08 y09 y10 y11 y12 y13 y14 y15 y16 y17 y18 y19 y20 y21 y22 y23 y24 y25 y26 y27 y28 y29 y30 y31 y32 y33 y34 y35 y36 y37 y38 y39 y40 y41 y42 y43 y44
//  v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v
// prt kmf jrf pps dgw jfq qmr wqt bqm nwn vsm qmm ppp cpr qtq svk bss ccw pnv pdb fpv rqf btq nbr cbh smj qfv nnq dwg knv bgj pct scw sfs hrd qph fjn z37 knb tbr ntw gtv fvc pss qqr
//
//     kmf jrf pps dgw jfq qmr wqt bqm nwn vsm qmm ppp cpr qtq svk bss ccw pnv pdb fpv nnr btq nbr cbh smj qfv nnq dwg knv bgj pct scw sfs hrd qph fjn rrn knb tbr ntw gtv fvc pss
//     OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR  OR
//     cdb shr phc fmr tsw fjt ktn qpn hwp rjw kgt wgk jrk rhw cjt tnn fbb cdt swq smh jsd qpk spp vhv rmv qcm pfb pqk thq chb rdn rdg hmh tdb wdv crk vhj tkt qkk fvv jhd pkv wrt
//      v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v
//     qnf vnm jdk jtm ksh drd qft vrr bkp vnw bcq vnc kgn dbd kcm z16 hnq hqr njb hvv sfw nkn qst btv hnv rcm mfc rsj rbk qsj vtb jmt qpd pwb mcv nbm jrg crp ggg tph shp thm fjs
//
//     jnj wsm mgr mtb rqw kcc ncp twj jtc vmf ftq dbg dgr pgq shb grr rcc dwd ssf sqr rqf jck crt hbc rhk mcj fjc hdk ccs jnr tjk dfn ttn hnk jdq ckw gcg vsq gkk wqg hbk kcp dnf bmv
//     AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND AND
//     prt qnf vnm jdk jtm ksh drd qft vrr bkp vnw bcq vnc kgn dbd kcm fkb hnq hqr njb hvv sfw nkn qst btv hnv rcm mfc rsj rbk qsj vtb jmt qpd pwb mcv nbm jrg crp ggg tph shp thm fjs
//      v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v
//     cdb shr phc fmr tsw fjt ktn qpn hwp rjw kgt wgk jrk rhw cjt tnn fbb cdt swq smh jsd qpk spp vhv rmv qcm pfb pqk thq chb z31 rdg hmh tdb wdv crk vhj tkt qkk fvv jhd pkv wrt kwh
//
//     jnj wsm mgr mtb rqw kcc ncp twj jtc vmf ftq dbg dgr pgq shb grr rcc dwd ssf sqr rqf jck crt hbc rhk mcj fjc hdk ccs jnr tjk dfn ttn hnk jdq ckw gcg vsq gkk wqg hbk kcp dnf bmv kwh
//     XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR XOR OR
//     prt qnf vnm jdk jtm ksh drd qft vrr bkp vnw bcq vnc kgn dbd kcm fkb hnq hqr njb hvv sfw nkn qst btv hnv rcm mfc rsj rbk qsj vtb jmt qpd pwb mcv nbm jrg crp ggg tph shp thm fjs qqr
//      v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v   v
//     z01 z02 z03 z04 z05 z06 z07 z08 z09 z10 z11 z12 z13 z14 z15 fkb z17 z18 z19 z20 z21 z22 z23 z24 z25 z26 z27 z28 z29 z30 rdn z32 z33 z34 z35 z36 rrn z38 z39 z40 z41 z42 z43 z44 z45
//
//
// x21 AND y21 -> *rqf* -> nnr
// x21 XOR y21 -> *nnr* -> rqf
// x37 AND y37 -> *z37* -> rrn
// gcg XOR nbm -> *rrn* -> z37
// tjk AND qsj -> *z31* -> rdn
// tjk XOR qsj -> *rdn* -> z31
// bss OR tnn -> *z16* -> fkb
// grr XOR kcm -> *fkb* -> z16
//
//
// fkb,nnr,rdn,rqf,rrn,z16,z31,z37
//
