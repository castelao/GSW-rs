window.BENCHMARK_DATA = {
  "lastUpdate": 1661399742886,
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
      },
      {
        "commit": {
          "author": {
            "email": "luiz.irber@gmail.com",
            "name": "Luiz Irber",
            "username": "luizirber"
          },
          "committer": {
            "email": "luiz.irber@gmail.com",
            "name": "Luiz Irber",
            "username": "luizirber"
          },
          "distinct": true,
          "id": "04e59f8c8d135998052c43dc6813ae9666c8159d",
          "message": "add link to talk",
          "timestamp": "2022-04-04T08:13:27-07:00",
          "tree_id": "6512c8483e2f95b6e4632d4c6300a4751293b7dc",
          "url": "https://github.com/castelao/GSW-rs/commit/04e59f8c8d135998052c43dc6813ae9666c8159d"
        },
        "date": 1649085952791,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 61,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_sso_0",
            "value": 3,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_anom_standard",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 161,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 16,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 203,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 350,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 107,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 204,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 69,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 153,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 660,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "guilherme@castelao.net",
            "name": "Guilherme Castelão",
            "username": "castelao"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2ebdfb74960c5c6c09e82f7c4cc2999047c5467a",
          "message": "Citation (#38)\n\n* Adding citation note\r\n\r\n* Formating citation",
          "timestamp": "2022-07-27T22:26:49-03:00",
          "tree_id": "b6935117881f50d0ef90af807fb1558cc419db9f",
          "url": "https://github.com/castelao/GSW-rs/commit/2ebdfb74960c5c6c09e82f7c4cc2999047c5467a"
        },
        "date": 1658972361117,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 39,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 64,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 63,
            "range": "± 2",
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
            "value": 43,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 167,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 167,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 213,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 52,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 372,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 60,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 123,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 219,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 66,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 51,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 74,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 173,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 163,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 44,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 39,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 81,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 55,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 79,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 147,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 149,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 644,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 20,
            "range": "± 0",
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
            "value": 9,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 24,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "guilherme@castelao.net",
            "name": "Guilherme Castelão",
            "username": "castelao"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7ef8611647248c60d7cf17ec474f802cb5d205d0",
          "message": "Talk seas2022 (#39)\n\n* Requirements to create presentation\r\n\r\n* Presentation skeleton\r\n\r\n* Ideas with Luiz\r\n\r\n* fix package structure and monospace font\r\n\r\n* two column package structure\r\n\r\n* More content/ideas on the initial slides\r\n\r\n* Moving docs before doctest\r\n\r\n* cargo test\r\n\r\nIf time is short cut this cargo test first\r\n\r\n* Cleaning\r\n\r\n* Features\r\n\r\n* cleaning\r\n\r\n* Adding some ideas for embedded and others\r\n\r\n* typos\r\n\r\n* why rust\r\n\r\n* Reviewing first slides\r\n\r\n* start ffi\r\n\r\n* Add reference to Sourmash?\r\n\r\n* Text around GA link\r\n\r\n* Cleaning validation slide\r\n\r\n* Improving embedded\r\n\r\n* cleaning\r\n\r\n* ffi and bench\r\n\r\n* cleaning colaborative session\r\n\r\n* Links on other applications\r\n\r\n* Community\r\n\r\n* typos\r\n\r\n* Enable instead of empower\r\n\r\n* link to slides\r\n\r\n* links\r\n\r\n* reorder links\r\n\r\n* fix typo\r\n\r\n* Re-routing link to slides on main branch\r\n\r\nCo-authored-by: Luiz Irber <luiz.irber@gmail.com>",
          "timestamp": "2022-08-24T21:43:02-06:00",
          "tree_id": "1a622fe03c12d846fbfa2128a0c8b4b494a90dc6",
          "url": "https://github.com/castelao/GSW-rs/commit/7ef8611647248c60d7cf17ec474f802cb5d205d0"
        },
        "date": 1661399742109,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 69,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_sso_0",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_anom_standard",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 189,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 189,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 242,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 409,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 186,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 191,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 95,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 722,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 23,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 23,
            "range": "± 0",
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
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}