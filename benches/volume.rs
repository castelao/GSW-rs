use criterion::{black_box, criterion_group, criterion_main, Criterion};

use gsw::{conversions, practical_salinity, volume};

fn volume(c: &mut Criterion) {
    c.bench_function("specvol", |b| {
        b.iter(|| volume::specvol(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("alpha", |b| {
        b.iter(|| volume::alpha(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("beta", |b| {
        b.iter(|| volume::beta(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("specvol_sso_0", |b| {
        b.iter(|| volume::specvol_sso_0(black_box(10.0)))
    });

    c.bench_function("specvol_anom_standard", |b| {
        b.iter(|| {
            volume::specvol_anom_standard(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("specvol_alpha_beta", |b| {
        b.iter(|| {
            volume::specvol_alpha_beta(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap()
        })
    });

    c.bench_function("rho", |b| {
        b.iter(|| volume::rho(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("rho_alpha_beta", |b| {
        b.iter(|| {
            volume::rho_alpha_beta(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap()
        })
    });

    c.bench_function("sigma0", |b| {
        b.iter(|| volume::sigma0(black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("sigma1", |b| {
        b.iter(|| volume::sigma1(black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("sigma2", |b| {
        b.iter(|| volume::sigma2(black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("sigma3", |b| {
        b.iter(|| volume::sigma3(black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("sigma4", |b| {
        b.iter(|| volume::sigma4(black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("cabbeling", |b| {
        b.iter(|| volume::cabbeling(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("sound_speed", |b| {
        b.iter(|| volume::sound_speed(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("dynamic_enthalpy", |b| {
        b.iter(|| {
            volume::dynamic_enthalpy(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap()
        })
    });

    c.bench_function("sa_from_rho", |b| {
        b.iter(|| volume::sa_from_rho(black_box(1024.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("specvol_first_derivatives", |b| {
        b.iter(|| {
            volume::specvol_first_derivatives(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("specvol_first_derivatives_wrt_enthalpy", |b| {
        b.iter(|| {
            volume::specvol_first_derivatives_wrt_enthalpy(
                black_box(10.0),
                black_box(10.0),
                black_box(10.0),
            )
            .unwrap()
        })
    });

    c.bench_function("specvol_second_derivatives", |b| {
        b.iter(|| {
            volume::specvol_second_derivatives_wrt_enthalpy(
                black_box(10.0),
                black_box(10.0),
                black_box(10.0),
            )
            .unwrap()
        })
    });

    c.bench_function("specvol_second_derivatives_wrt_enthalpy", |b| {
        b.iter(|| {
            volume::specvol_second_derivatives(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });
    c.bench_function("alpha_on_beta", |b| {
        b.iter(|| volume::alpha_on_beta(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("rho_first_derivatives", |b| {
        b.iter(|| {
            volume::rho_first_derivatives(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("rho_second_derivatives", |b| {
        b.iter(|| {
            volume::rho_second_derivatives(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("thermobaric", |b| {
        b.iter(|| volume::thermobaric(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("enthalpy", |b| {
        b.iter(|| volume::enthalpy(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("enthalpy_first_derivatives", |b| {
        b.iter(|| {
            volume::enthalpy_first_derivatives(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("enthalpy_second_derivatives", |b| {
        b.iter(|| {
            volume::enthalpy_second_derivatives(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("enthalpy_diff", |b| {
        b.iter(|| {
            volume::enthalpy_diff(
                black_box(33.0),
                black_box(10.0),
                black_box(0.0),
                black_box(100.0),
            )
            .unwrap()
        })
    });

    c.bench_function("kappa", |b| {
        b.iter(|| volume::kappa(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap())
    });

    c.bench_function("internal_energy", |b| {
        b.iter(|| {
            volume::internal_energy(black_box(10.0), black_box(10.0), black_box(10.0)).unwrap()
        })
    });

    c.bench_function("internal_energy_first_derivatives", |b| {
        b.iter(|| {
            volume::internal_energy_first_derivatives(
                black_box(10.0),
                black_box(10.0),
                black_box(10.0),
            )
            .unwrap()
        })
    });

    c.bench_function("internal_energy_second_derivatives", |b| {
        b.iter(|| {
            volume::internal_energy_first_derivatives(
                black_box(10.0),
                black_box(10.0),
                black_box(10.0),
            )
            .unwrap()
        })
    });

    c.bench_function("ct_maxdensity", |b| {
        b.iter(|| volume::ct_maxdensity(black_box(10.0), black_box(10.0)).unwrap())
    });
}

fn practical_salinity(c: &mut Criterion) {
    c.bench_function("sp_from_c", |b| {
        b.iter(|| {
            practical_salinity::sp_from_c(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("c_from_sp", |b| {
        b.iter(|| {
            practical_salinity::sp_from_c(black_box(10.0), black_box(10.0), black_box(10.0))
                .unwrap()
        })
    });

    c.bench_function("sp_salinometer", |b| {
        b.iter(|| practical_salinity::sp_salinometer(black_box(10.0), black_box(10.0)).unwrap())
    });
}

fn conversions(c: &mut Criterion) {
    c.bench_function("z_from_p", |b| {
        b.iter(|| {
            conversions::z_from_p(
                black_box(10.0),
                black_box(10.0),
                black_box(10.0),
                black_box(10.0),
            )
        })
    });
}

criterion_group!(benches, volume, practical_salinity, conversions);
criterion_main!(benches);
