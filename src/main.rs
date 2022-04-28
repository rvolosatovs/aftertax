use std::env::args;

fn main() {
    let mut args = args();
    let exe = args.next().unwrap();
    let gross = args
        .next()
        .expect(&format!("usage: {:?} <gross yearly salary in EUR>", exe))
        .parse()
        .unwrap();
    println!(
        r#"Salary of {} EUR after tax:
AU: {} EUR ({} EUR per month)
BE: {} EUR ({} EUR per month) (- municipality tax 7-9%)
CZ: {} EUR ({} EUR per month)
IT: {} EUR ({} EUR per month)
LU: {} EUR ({} EUR per month)
NL: {} EUR ({} EUR per month)"#,
        gross,
        austria(gross).round(),
        (austria(gross) / 12.).round(),
        belgium(gross).round(),
        (belgium(gross) / 12.).round(),
        czech_republic(gross).round(),
        (czech_republic(gross) / 12.).round(),
        //germany(gross).round(),
        //(germany(gross) / 12.).round(),
        italy(gross).round(),
        (italy(gross) / 12.).round(),
        luxembourg(gross).round(),
        (luxembourg(gross) / 12.).round(),
        netherlands(gross).round(),
        (netherlands(gross) / 12.).round(),
    );
}

fn austria(gross: f64) -> f64 {
    match gross {
        _ if gross > 1000000. => (gross - 1000000.) * (1. - 0.55) + austria(1000000.),
        _ if gross > 90000. => (gross - 90000.) * (1. - 0.50) + austria(90000.),
        _ if gross > 60000. => (gross - 60000.) * (1. - 0.48) + austria(60000.),
        _ if gross > 31000. => (gross - 31000.) * (1. - 0.42) + austria(31000.),
        _ if gross > 18000. => (gross - 18000.) * (1. - 0.325) + austria(18000.),
        _ if gross > 11000. => (gross - 11000.) * (1. - 0.20) + austria(11000.),
        _ => gross,
    }
}

fn belgium(gross: f64) -> f64 {
    match gross {
        _ if gross > 42370. => (gross - 42370.) * (1. - 0.50) + belgium(42370.),
        _ if gross > 24480. => (gross - 24480.) * (1. - 0.45) + belgium(24480.),
        _ if gross > 13870. => (gross - 13870.) * (1. - 0.40) + belgium(13870.),
        _ if gross > 9050. => (gross - 9050.) * (1. - 0.25) + belgium(9050.),
        _ => gross,
    }
}

fn czech_republic(gross: f64) -> f64 {
    match gross {
        _ if gross > 71836. => (gross - 71836.) * (1. - 0.23) + czech_republic(71836.),
        _ => gross * (1. - 0.15),
    }
}

//fn germany(gross: f64) -> f64 {
//    match gross {
//        _ if gross > 274612. => (gross - 274612.) * (1. - 0.45) + germany(274612.),
//        _ if gross > 57918. => (gross - 57918.) * (1. - 0.42) + germany(57918.),
//        _ if gross > 14926. => (gross - 14926.) * (1. - 0.14) + germany(14926.),
//        _ if gross > 9744. => (gross - 9744.) * (1. - 0.14) + germany(9744.),
//        _ => gross,
//    }
//}

fn luxembourg(gross: f64) -> f64 {
    match gross {
        _ if gross > 200005. => (gross - 200005.) * (1. - 0.4578) + luxembourg(200005.),
        _ if gross > 150001. => (gross - 150001.) * (1. - 0.4469) + luxembourg(150001.),
        _ if gross > 100003. => (gross - 100003.) * (1. - 0.4280) + luxembourg(100003.),
        _ if gross > 45898. => (gross - 45898.) * (1. - 0.4173) + luxembourg(45898.),
        _ if gross > 43954. => (gross - 43954.) * (1. - 0.4066) + luxembourg(43954.),
        _ if gross > 42010. => (gross - 42010.) * (1. - 0.3852) + luxembourg(42010.),
        _ if gross > 40066. => (gross - 40066.) * (1. - 0.3638) + luxembourg(40066.),
        _ if gross > 38122. => (gross - 38122.) * (1. - 0.3424) + luxembourg(38122.),
        _ if gross > 36178. => (gross - 36178.) * (1. - 0.3210) + luxembourg(36178.),
        _ if gross > 34234. => (gross - 34234.) * (1. - 0.2996) + luxembourg(34234.),
        _ if gross > 32290. => (gross - 32290.) * (1. - 0.2782) + luxembourg(32290.),
        _ if gross > 30346. => (gross - 30346.) * (1. - 0.2568) + luxembourg(30346.),
        _ if gross > 28402. => (gross - 28402.) * (1. - 0.2354) + luxembourg(28402.),
        _ if gross > 26458. => (gross - 26458.) * (1. - 0.2140) + luxembourg(26458.),
        _ if gross > 24514. => (gross - 24514.) * (1. - 0.1926) + luxembourg(24514.),
        _ if gross > 22570. => (gross - 22570.) * (1. - 0.1712) + luxembourg(22570.),
        _ if gross > 20626. => (gross - 20626.) * (1. - 0.1498) + luxembourg(20626.),
        _ if gross > 18754. => (gross - 18754.) * (1. - 0.1284) + luxembourg(18754.),
        _ if gross > 16882. => (gross - 16882.) * (1. - 0.1177) + luxembourg(16882.),
        _ if gross > 15010. => (gross - 15010.) * (1. - 0.1070) + luxembourg(15010.),
        _ if gross > 13138. => (gross - 13138.) * (1. - 0.0963) + luxembourg(13138.),
        _ if gross > 11266. => (gross - 11266.) * (1. - 0.0856) + luxembourg(11266.),
        _ => gross,
    }
}

fn italy(gross: f64) -> f64 {
    match gross {
        _ if gross > 55000. => (gross - 55000.) * (1. - 0.43) + italy(55000.),
        _ if gross > 28000. => (gross - 28000.) * (1. - 0.35) + italy(28000.),
        _ if gross > 15000. => (gross - 15000.) * (1. - 0.25) + italy(15000.),
        _ => gross * (1. - 0.23),
    }
}

fn netherlands(gross: f64) -> f64 {
    match gross {
        _ if gross > 69399. => (gross - 69399.) * (1. - 0.4950) + netherlands(69399.),
        _ => gross * (1. - 0.3707),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn austria() {
        // From https://www.usp.gv.at/en/steuern-finanzen/einkommensteuer/tarifstufen-berechnungsformeln.html
        assert_eq!(super::austria(40000.), 40000. - 9405.);
    }

    #[test]
    fn luxembourg() {
        // From https://taxsummaries.pwc.com/Luxembourg/Individual/Taxes-on-personal-income
        assert_eq!(super::luxembourg(20000.).round(), 20000. - 921.);
        assert_eq!(super::luxembourg(38700.).round(), 38700. - 5317. - 1.);
        assert_eq!(super::luxembourg(58000.).round(), 58000. - 13081. - 1.);
        assert_eq!(super::luxembourg(77400.).round(), 77400. - 21177. - 1.);
        assert_eq!(super::luxembourg(96700.).round(), 96700. - 29231. - 1.);
        assert_eq!(super::luxembourg(116000.).round(), 116000. - 37456. - 1.);
        assert_eq!(super::luxembourg(135500.).round(), 135500. - 45802. - 1.);
        assert_eq!(super::luxembourg(155000.).round(), 155000. - 54242. - 1.);
        assert_eq!(super::luxembourg(205000.).round(), 205000. - 76642. - 1.);
    }
}
