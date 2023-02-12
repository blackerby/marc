struct ControlField {
    tag: String,
    value: String,
}

#[cfg(test)]
mod tests {
    const DIRS: &str = "001001200000005001700012006001900029007000700048008004100055040002300096042000800119043001200127074002000139086001700159100005500176245021100231260008500442336002200527337002400549338003300573440003900606500005400645504004100699538015700740650002000897650002000917700002100937776020100958856006501159907003501224998004501259910001201304910002801316945007501344";
    const DATA: &str = "ocm5717594020041206161421.0m        d f      cr cn-041206s1976    dcua    sb   f000 0 eng c  aGPOcGPOdMvIdMvI  apcc  an-us---  a0620-A (online)0 aI 19.4/2:7351 aSwanson, Vernon E.q(Vernon Emmanuel),d1922-1992.10aGuidelines for sample collecting and analytical methods used in the U.S. Geological Survey for determining chemical composition of coalh[electronic resource] /cby Vernon E. Swanson and Claude Huffman, Jr.  a[Washington, D.C.] :bU.S. Dept. of the Interior, U.S. Geological Survey,c1976.  atext2rdacontent.  acomputer2rdamedia.  aonline resource2rdacarrier. 0aGeological Survey circular ;v735.  aTitle from title screen (viewed on Dec. 06, 2004)  aIncludes bibliographical references.  aMode of access: Internet from the USGS Web site. Address as of 12/06/04: http://pubs.usgs.gov/circ/c735/index.htm; current access is available via PURL. 0aCoalxAnalysis. 0aCoalxSampling.1 aHuffman, Claude.1 aSwanson, Vernon Emanuel,d1922-tGuidelines for sample collecting and analytical methods used in the U.S. Geological Survey for determining chemical composition of coalhiv, 11 p.w(OCoLC)2331861.40uhttp://purl.access.gpo.gov/GPO/LPS56007zView online version  a.b37991760b04-08-17c07-26-05  aes001b07-26-05cmdae-fenggdcuh0i1  aMARCIVE  aHathi Trust report None  g0j0lesb  onp$0.00q r s-t255u0v0w0x0y.i138993579z07-26-05";
    use super::*;

    #[test]
    fn new_control_field() {}

    fn dirs_and_data_setup(dirs: &str, data: &str, offset: i32) -> (String, Vec<char>) {
        let mut out = Vec::new();
        out.push('a');
        (String::from(""), out)
    }
}
