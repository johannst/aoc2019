use std::collections::HashMap;

type ElemId = i64;
type Elem = (ElemId, i64);
type Reactions = HashMap<Elem, Vec<Elem>>;
type ElemLookup = HashMap<String, ElemId>;

#[derive(Debug)]
enum Err {
    InvalidInput,
    FuelTokenNotFound,
    OreTokenNotFound,
    UnknownElement,
}

fn split_formula(formula: &str) -> aoc19::Result<(&str, &str)> {
    let p: Vec<_> = formula.split("=>").collect();
    if p.len() != 2 {
        return Err(aoc19::Error::boxed(Err::InvalidInput));
    }
    Ok((p[0], p[1]))
}

fn split_reactant(reactant: &str) -> aoc19::Result<(String, i64)> {
    let p: Vec<_> = reactant.split_ascii_whitespace().collect();
    if p.len() != 2 {
        return Err(aoc19::Error::boxed(Err::InvalidInput));
    }
    Ok((p[1].to_string(), p[0].parse::<i64>()?))
}

fn gen_reactions(formulas: &String) -> aoc19::Result<(Reactions, ElemLookup)> {
    let mut reactions = HashMap::new();

    let mut idcnt = 0;
    let mut ids = HashMap::new();

    for formula in formulas.lines() {
        let (in_formula, result) = split_formula(formula)?;

        let mut reactants = Vec::new();
        for reactant in in_formula.split(',') {
            let reactant = reactant.trim();
            let (r, q) = split_reactant(reactant)?;
            let id = *ids.entry(r).or_insert_with(|| {
                idcnt += 1;
                idcnt
            });
            reactants.push((id, q));
        }

        let (r, q) = split_reactant(result)?;
        let id = *ids.entry(r).or_insert_with(|| {
            idcnt += 1;
            idcnt
        });
        reactions.insert((id, q), reactants);
    }

    Ok((reactions, ids))
}

fn requiere_n_reactions(quantity_needed: i64, quatity_per_reaction: i64) -> i64 {
    (quantity_needed + quatity_per_reaction - 1) / quatity_per_reaction
}

fn react(
    product_id: ElemId,
    mut needed_quantity: i64,
    reactions: &Reactions,
    ore_id: ElemId,
    remaining: &mut HashMap<ElemId, i64>,
) -> aoc19::Result<i64> {
    let (&(_, product_quantity), reactants) = reactions
        .iter()
        .find(|(e, _)| e.0 == product_id)
        .ok_or(aoc19::Error::boxed(Err::UnknownElement))?;

    let remaining_quantity = remaining.entry(product_id).or_insert(0);
    if *remaining_quantity >= needed_quantity {
        *remaining_quantity -= needed_quantity;
        return Ok(0);
    } else {
        needed_quantity -= *remaining_quantity;
        *remaining_quantity = 0;
    }

    let reaction_cnt = requiere_n_reactions(needed_quantity, product_quantity);
    *remaining_quantity += reaction_cnt * product_quantity - needed_quantity;

    let mut ore_cnt = 0;
    for &(reactant_id, reactant_quantity) in reactants {
        if reactant_id == ore_id {
            ore_cnt += reaction_cnt * reactant_quantity;
            continue;
        }
        ore_cnt += react(
            reactant_id,
            reaction_cnt * reactant_quantity,
            reactions,
            ore_id,
            remaining,
        )?;
    }

    Ok(ore_cnt)
}

fn part_one() -> aoc19::Result<i64> {
    let formulas = std::fs::read_to_string("input/day14")?;
    let (reactions, lookup) = gen_reactions(&formulas)?;
    let fuel_id = *lookup
        .get("FUEL")
        .ok_or(aoc19::Error::boxed(Err::FuelTokenNotFound))?;
    let ore_id = *lookup
        .get("ORE")
        .ok_or(aoc19::Error::boxed(Err::OreTokenNotFound))?;

    react(fuel_id, 1, &reactions, ore_id, &mut HashMap::new())
}

fn part_two() -> aoc19::Result<i64> {
    let formulas = std::fs::read_to_string("input/day14")?;
    let (reactions, lookup) = gen_reactions(&formulas)?;
    let fuel_id = *lookup
        .get("FUEL")
        .ok_or(aoc19::Error::boxed(Err::FuelTokenNotFound))?;
    let ore_id = *lookup
        .get("ORE")
        .ok_or(aoc19::Error::boxed(Err::OreTokenNotFound))?;

    const MAX_ORE: i64 = 1_000_000_000_000;

    let mut upper = MAX_ORE; // random choice
    let mut lower = 0;
    let fuel = loop {
        let cand = (upper + lower) / 2;
        let ore = react(fuel_id, cand, &reactions, ore_id, &mut HashMap::new())?;

        if ore > MAX_ORE {
            upper = cand;
        } else {
            lower = cand;
        }

        if upper - lower == 1 {
            break lower;
        }
    };
    Ok(fuel)
}

fn main() -> aoc19::Result<()> {
    println!("Part One: produce 1 FUEL requieres {} ORE", part_one()?);
    println!(
        "Part Two: with 1 trillion ORE can produce {} FUEL",
        part_two()?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn react_one_fuel(formulas: String) -> aoc19::Result<i64> {
        let (reactions, lookup) = gen_reactions(&formulas)?;
        let fuel_id = *lookup
            .get("FUEL")
            .ok_or(aoc19::Error::boxed(Err::FuelTokenNotFound))?;
        let ore_id = *lookup
            .get("ORE")
            .ok_or(aoc19::Error::boxed(Err::OreTokenNotFound))?;

        react(fuel_id, 1, &reactions, ore_id, &mut HashMap::new())
    }

    #[test]
    fn test_example1() -> aoc19::Result<()> {
        let input = r"10 ORE => 10 A
                     1 ORE => 1 B
                     7 A, 1 B => 1 C
                     7 A, 1 C => 1 D
                     7 A, 1 D => 1 E
                     7 A, 1 E => 1 FUEL"
            .to_string();
        assert_eq!(react_one_fuel(input)?, 31);
        Ok(())
    }

    #[test]
    fn test_example2() -> aoc19::Result<()> {
        let input = r"9 ORE => 2 A
                      8 ORE => 3 B
                      7 ORE => 5 C
                      3 A, 4 B => 1 AB
                      5 B, 7 C => 1 BC
                      4 C, 1 A => 1 CA
                      2 AB, 3 BC, 4 CA => 1 FUEL"
            .to_string();
        assert_eq!(react_one_fuel(input)?, 165);
        Ok(())
    }

    #[test]
    fn test_example3() -> aoc19::Result<()> {
        let input = r"157 ORE => 5 NZVS
                      165 ORE => 6 DCFZ
                      44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                      12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                      179 ORE => 7 PSHF
                      177 ORE => 5 HKGWZ
                      7 DCFZ, 7 PSHF => 2 XJWVT
                      165 ORE => 2 GPVTF
                      3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            .to_string();
        assert_eq!(react_one_fuel(input)?, 13312);
        Ok(())
    }

    #[test]
    fn test_example4() -> aoc19::Result<()> {
        let input = r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                      17 NVRVD, 3 JNWZP => 8 VPVL
                      53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                      22 VJHF, 37 MNCFX => 5 FWMGM
                      139 ORE => 4 NVRVD
                      144 ORE => 7 JNWZP
                      5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                      5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                      145 ORE => 6 MNCFX
                      1 NVRVD => 8 CXFTF
                      1 VJHF, 6 MNCFX => 4 RFSQX
                      176 ORE => 6 VJHF"
            .to_string();
        assert_eq!(react_one_fuel(input)?, 180697);
        Ok(())
    }

    #[test]
    fn test_example5() -> aoc19::Result<()> {
        let input = r"171 ORE => 8 CNZTR
                      7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                      114 ORE => 4 BHXH
                      14 VRPVC => 6 BMBT
                      6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                      6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                      15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                      13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                      5 BMBT => 4 WPTQ
                      189 ORE => 9 KTJDG
                      1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                      12 VRPVC, 27 CNZTR => 2 XDBXC
                      15 KTJDG, 12 BHXH => 5 XCVML
                      3 BHXH, 2 VRPVC => 7 MZWV
                      121 ORE => 7 VRPVC
                      7 XCVML => 6 RJRHP
                      5 BHXH, 4 VRPVC => 5 LTCX"
            .to_string();
        assert_eq!(react_one_fuel(input)?, 2210736);
        Ok(())
    }
}
