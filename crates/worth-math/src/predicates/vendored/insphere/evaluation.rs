//! Vendored insphere predicate evaluation.

use super::adaptive::insphereadapt;

use super::super::parameters::{abs, PARAMS};

#[inline]
pub(in crate::predicates) fn insphere(
    pa: [f64; 3],
    pb: [f64; 3],
    pc: [f64; 3],
    pd: [f64; 3],
    pe: [f64; 3],
) -> f64 {
    let aex = pa[0] - pe[0];
    let bex = pb[0] - pe[0];
    let cex = pc[0] - pe[0];
    let dex = pd[0] - pe[0];
    let aey = pa[1] - pe[1];
    let bey = pb[1] - pe[1];
    let cey = pc[1] - pe[1];
    let dey = pd[1] - pe[1];
    let aez = pa[2] - pe[2];
    let bez = pb[2] - pe[2];
    let cez = pc[2] - pe[2];
    let dez = pd[2] - pe[2];
    let aexbey = aex * bey;
    let bexaey = bex * aey;
    let ab = aexbey - bexaey;
    let bexcey = bex * cey;
    let cexbey = cex * bey;
    let bc = bexcey - cexbey;
    let cexdey = cex * dey;
    let dexcey = dex * cey;
    let cd = cexdey - dexcey;
    let dexaey = dex * aey;
    let aexdey = aex * dey;
    let da = dexaey - aexdey;
    let aexcey = aex * cey;
    let cexaey = cex * aey;
    let ac = aexcey - cexaey;
    let bexdey = bex * dey;
    let dexbey = dex * bey;
    let bd = bexdey - dexbey;
    let abc = aez * bc - bez * ac + cez * ab;
    let bcd = bez * cd - cez * bd + dez * bc;
    let cda = cez * da + dez * ac + aez * cd;
    let dab = dez * ab + aez * bd + bez * da;
    let alift = aex * aex + aey * aey + aez * aez;
    let blift = bex * bex + bey * bey + bez * bez;
    let clift = cex * cex + cey * cey + cez * cez;
    let dlift = dex * dex + dey * dey + dez * dez;
    let det = dlift * abc - clift * dab + (blift * cda - alift * bcd);
    let aezplus = abs(aez);
    let bezplus = abs(bez);
    let cezplus = abs(cez);
    let dezplus = abs(dez);
    let aexbeyplus = abs(aexbey);
    let bexaeyplus = abs(bexaey);
    let bexceyplus = abs(bexcey);
    let cexbeyplus = abs(cexbey);
    let cexdeyplus = abs(cexdey);
    let dexceyplus = abs(dexcey);
    let dexaeyplus = abs(dexaey);
    let aexdeyplus = abs(aexdey);
    let aexceyplus = abs(aexcey);
    let cexaeyplus = abs(cexaey);
    let bexdeyplus = abs(bexdey);
    let dexbeyplus = abs(dexbey);
    let permanent = ((cexdeyplus + dexceyplus) * bezplus
        + (dexbeyplus + bexdeyplus) * cezplus
        + (bexceyplus + cexbeyplus) * dezplus)
        * alift
        + ((dexaeyplus + aexdeyplus) * cezplus
            + (aexceyplus + cexaeyplus) * dezplus
            + (cexdeyplus + dexceyplus) * aezplus)
            * blift
        + ((aexbeyplus + bexaeyplus) * dezplus
            + (bexdeyplus + dexbeyplus) * aezplus
            + (dexaeyplus + aexdeyplus) * bezplus)
            * clift
        + ((bexceyplus + cexbeyplus) * aezplus
            + (cexaeyplus + aexceyplus) * bezplus
            + (aexbeyplus + bexaeyplus) * cezplus)
            * dlift;
    let errbound = PARAMS.isperrbound_a * permanent;
    if det > errbound || -det > errbound {
        return det;
    }
    insphereadapt(pa, pb, pc, pd, pe, permanent)
}
