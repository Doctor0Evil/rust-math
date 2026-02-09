pub enum MetricFamily {
    Bee,
    Marine,
    UrbanHeat,
    Cyboquatic,
    Governance,
}

pub struct CorridorBand {
    pub var_id: String,        // e.g. "shear_Pa", "WBGT_degC"
    pub family: MetricFamily,
    pub units: String,
    pub safe_min: f64,
    pub safe_max: f64,
    pub gold_min: f64,
    pub gold_max: f64,
    pub hard_min: f64,
    pub hard_max: f64,
    pub weight: f64,
    pub evidence_id: String,   // dataset or study ID
}

pub struct HostBudgetBand {
    pub host_id: String,       // hive, taxon+stage, human, infra-node
    pub metric: String,        // "HB", "fishscalar", "RoH", etc.
    pub min: f64,
    pub max: f64,
}

pub struct EcoBand {
    pub eco_id: String,        // corridor or ecobranch ID
    pub K_min: f64,
    pub E_min: f64,
    pub R_max: f64,
}

pub struct CorridorInvariant {
    pub metric_family: MetricFamily,
    pub host_budget: HostBudgetBand,
    pub eco_band: EcoBand,
    pub dw_ceiling: f64,       // maximum allowed residual
}

pub struct ResidualState {
    pub r: Vec<f64>,           // normalized coordinates
    pub weights: Vec<f64>,     // must satisfy weight invariants
    pub v: f64,                // current residual
}

pub struct HysteresisFlags {
    pub any_gold: bool,
    pub any_hard: bool,
}

pub struct InvariantEval {
    pub residual: ResidualState,
    pub hysteresis: HysteresisFlags,
    pub within_host_budget: bool,
    pub within_eco_band: bool,
    pub within_dw: bool,
    pub permitted: bool,       // final gate
}

pub fn normalize_coord(x: f64, band: &CorridorBand) -> f64 {
    if x <= band.safe_min || x >= band.safe_max {
        1.0
    } else if x >= band.gold_min && x <= band.gold_max {
        0.0
    } else if x < band.gold_min {
        (band.gold_min - x) / (band.gold_min - band.hard_min)
    } else {
        (x - band.gold_max) / (band.hard_max - band.gold_max)
    }
    .clamp(0.0, 1.0)
}

pub fn compute_residual(coords: &[f64], weights: &[f64]) -> f64 {
    assert_eq!(coords.len(), weights.len());
    coords.iter().zip(weights).map(|(r, w)| w * r * r).sum()
}

pub fn evaluate_corridor(
    bands: &[CorridorBand],
    values: &[f64],
    host_budget: &HostBudgetBand,
    eco_band: &EcoBand,
    prev_v: f64,
) -> InvariantEval {
    assert_eq!(bands.len(), values.len());

    let mut r = Vec::with_capacity(bands.len());
    let mut any_gold = false;
    let mut any_hard = false;

    for (band, &x) in bands.iter().zip(values) {
        let ri = normalize_coord(x, band);
        if ri > 0.0 { any_gold = true; }
        if ri >= 1.0 { any_hard = true; }
        r.push(ri);
    }

    let weights: Vec<f64> = bands.iter().map(|b| b.weight).collect();
    let v = compute_residual(&r, &weights);

    let within_host = host_budget.min <= v && v <= host_budget.max;
    let within_eco = v <= eco_band.R_max;
    let within_dw = v <= eco_band.R_max && v <= prev_v; // monotone outside safe interior

    let permitted = within_host && within_eco && within_dw && !any_hard;

    InvariantEval {
        residual: ResidualState { r, weights, v },
        hysteresis: HysteresisFlags { any_gold, any_hard },
        within_host_budget: within_host,
        within_eco_band: within_eco,
        within_dw,
        permitted,
    }
}
