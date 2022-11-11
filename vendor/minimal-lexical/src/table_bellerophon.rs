//! Cached exponents for basen values with 80-bit extended floats.
//!
//! Exact versions of base**n as an extended-precision float, with both
//! large and small powers. Use the large powers to minimize the amount
//! of compounded error. This is used in the Bellerophon algorithm.
//!
//! These values were calculated using Python, using the arbitrary-precision
//! integer to calculate exact extended-representation of each value.
//! These values are all normalized.
//!
//! DO NOT MODIFY: Generated by `etc/bellerophon_table.py`

#![cfg(feature = "compact")]
#![doc(hidden)]

use crate::bellerophon::BellerophonPowers;

// HIGH LEVEL
// ----------

pub const BASE10_POWERS: BellerophonPowers = BellerophonPowers {
    small: &BASE10_SMALL_MANTISSA,
    large: &BASE10_LARGE_MANTISSA,
    small_int: &BASE10_SMALL_INT_POWERS,
    step: BASE10_STEP,
    bias: BASE10_BIAS,
    log2: BASE10_LOG2_MULT,
    log2_shift: BASE10_LOG2_SHIFT,
};

// LOW-LEVEL
// ---------

const BASE10_SMALL_MANTISSA: [u64; 10] = [
    9223372036854775808,  // 10^0
    11529215046068469760, // 10^1
    14411518807585587200, // 10^2
    18014398509481984000, // 10^3
    11258999068426240000, // 10^4
    14073748835532800000, // 10^5
    17592186044416000000, // 10^6
    10995116277760000000, // 10^7
    13743895347200000000, // 10^8
    17179869184000000000, // 10^9
];
const BASE10_LARGE_MANTISSA: [u64; 66] = [
    11555125961253852697, // 10^-350
    13451937075301367670, // 10^-340
    15660115838168849784, // 10^-330
    18230774251475056848, // 10^-320
    10611707258198326947, // 10^-310
    12353653155963782858, // 10^-300
    14381545078898527261, // 10^-290
    16742321987285426889, // 10^-280
    9745314011399999080,  // 10^-270
    11345038669416679861, // 10^-260
    13207363278391631158, // 10^-250
    15375394465392026070, // 10^-240
    17899314949046850752, // 10^-230
    10418772551374772303, // 10^-220
    12129047596099288555, // 10^-210
    14120069793541087484, // 10^-200
    16437924692338667210, // 10^-190
    9568131466127621947,  // 10^-180
    11138771039116687545, // 10^-170
    12967236152753102995, // 10^-160
    15095849699286165408, // 10^-150
    17573882009934360870, // 10^-140
    10229345649675443343, // 10^-130
    11908525658859223294, // 10^-120
    13863348470604074297, // 10^-110
    16139061738043178685, // 10^-100
    9394170331095332911,  // 10^-90
    10936253623915059621, // 10^-80
    12731474852090538039, // 10^-70
    14821387422376473014, // 10^-60
    17254365866976409468, // 10^-50
    10043362776618689222, // 10^-40
    11692013098647223345, // 10^-30
    13611294676837538538, // 10^-20
    15845632502852867518, // 10^-10
    9223372036854775808,  // 10^0
    10737418240000000000, // 10^10
    12500000000000000000, // 10^20
    14551915228366851806, // 10^30
    16940658945086006781, // 10^40
    9860761315262647567,  // 10^50
    11479437019748901445, // 10^60
    13363823550460978230, // 10^70
    15557538194652854267, // 10^80
    18111358157653424735, // 10^90
    10542197943230523224, // 10^100
    12272733663244316382, // 10^110
    14287342391028437277, // 10^120
    16632655625031838749, // 10^130
    9681479787123295682,  // 10^140
    11270725851789228247, // 10^150
    13120851772591970218, // 10^160
    15274681817498023410, // 10^170
    17782069995880619867, // 10^180
    10350527006597618960, // 10^190
    12049599325514420588, // 10^200
    14027579833653779454, // 10^210
    16330252207878254650, // 10^220
    9505457831475799117,  // 10^230
    11065809325636130661, // 10^240
    12882297539194266616, // 10^250
    14996968138956309548, // 10^260
    17458768723248864463, // 10^270
    10162340898095201970, // 10^280
    11830521861667747109, // 10^290
    13772540099066387756, // 10^300
];
const BASE10_SMALL_INT_POWERS: [u64; 10] =
    [1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000];
const BASE10_STEP: i32 = 10;
const BASE10_BIAS: i32 = 350;
const BASE10_LOG2_MULT: i64 = 217706;
const BASE10_LOG2_SHIFT: i32 = 16;
