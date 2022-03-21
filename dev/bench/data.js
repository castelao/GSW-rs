window.BENCHMARK_DATA = {
  "lastUpdate": 1647821596818,
  "repoUrl": "https://github.com/castelao/GSW-rs",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "email": "luizirber@users.noreply.github.com",
            "name": "Luiz Irber",
            "username": "luizirber"
          },
          "committer": {
            "email": "luiz.irber@gmail.com",
            "name": "Luiz Irber",
            "username": "luizirber"
          },
          "distinct": true,
          "id": "21ba8a357b8ebee6423073364b5786167e54a3dd",
          "message": "Add initial benchmarks with criterion (#30)\n\n* add initial benchmarks\n* add functions currently implemented in the FFI\n* update Cargo.lock\n* add CI benchmark\n\nCo-authored-by: Guilherme Castelão <guilherme@castelao.net>",
          "timestamp": "2022-03-19T16:18:56-07:00",
          "tree_id": "3d48d4ad0693f52610f63b53bb105bb43a9e0424",
          "url": "https://github.com/castelao/GSW-rs/commit/21ba8a357b8ebee6423073364b5786167e54a3dd"
        },
        "date": 1647733577146,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 35,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 58,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 59,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_sso_0",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_anom_standard",
            "value": 40,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 157,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 36,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 160,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 36,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 36,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 36,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 50,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 37,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 293,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 47,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 67,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 43,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 65,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 150,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 38,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 75,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 52,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 73,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 618,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 22,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 21,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 10,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 22,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "luizirber@users.noreply.github.com",
            "name": "Luiz Irber",
            "username": "luizirber"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7b5c6b5ab2b61ed86585fffd69f6c780e8894706",
          "message": "Feat: Add new functions from #36 (#37)\n\n* Add new function from #36\r\n* benchmarks for new functions",
          "timestamp": "2022-03-21T00:00:18Z",
          "tree_id": "2d1212ef4823c48698dd2a237f28f1650e6f70fa",
          "url": "https://github.com/castelao/GSW-rs/commit/7b5c6b5ab2b61ed86585fffd69f6c780e8894706"
        },
        "date": 1647821595791,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 42,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 71,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 73,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_sso_0",
            "value": 6,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_anom_standard",
            "value": 48,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 194,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 190,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 23,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 45,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 46,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 46,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 46,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 240,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 63,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 42,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 367,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 129,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 239,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 80,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 51,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 78,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 185,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 181,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 52,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 47,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 89,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 61,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 89,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 154,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 156,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 765,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 30,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 30,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 14,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 30,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}