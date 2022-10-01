window.BENCHMARK_DATA = {
  "lastUpdate": 1664659645306,
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
          "id": "014d7ceb6956f434d86918e57d60fa877fd5ecff",
          "message": "ct_from_pt (#41)\n\n* feat: ct_from_pt\r\n\r\n* feat: p_from_z()\r\n\r\n* feat: sr_from_sp()\r\n\r\n* feat: sp_from_sr()\r\n\r\n* wip: gibbs()\r\n\r\n* test: gibbs for NaN input\r\n\r\n* Implementing gibbs()\r\n\r\n* Making volume::non_dimensional_sa available within the crate\r\n\r\n* wip: deltasa_from_sp()\r\n\r\n* documenting z_from_p()\r\n\r\n* fix: .contains() doesn't handle NaN\r\n\r\n* re-ordering for consistency\r\n\r\n* cleaning code\r\n\r\n* Notes on gibbs()\r\n\r\n* Removing unsed function\r\n\r\n* Allow gibbs as a dead_code for now\r\n\r\n* Using mutable instead of redefining\r\n\r\n* Ignoring excessive precision for now\r\n\r\n* Ignoring suggestion to use contains() for now\r\n\r\n* Temporary solution to address powerpc64",
          "timestamp": "2022-09-22T14:11:25-06:00",
          "tree_id": "4f04f3302b478b0256a5738d8ae94cd866b13da5",
          "url": "https://github.com/castelao/GSW-rs/commit/014d7ceb6956f434d86918e57d60fa877fd5ecff"
        },
        "date": 1663878286410,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 47,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 75,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 76,
            "range": "± 2",
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
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 204,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 53,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 204,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 26,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 54,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 55,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 54,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 54,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 270,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 69,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 49,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 427,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 69,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 148,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 266,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 80,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 60,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 83,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 206,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 206,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 51,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 51,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 68,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 98,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 173,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 174,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 804,
            "range": "± 30",
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
            "value": 12,
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
          "id": "08af2d271379df7ed160787cc7cdce337b90d49f",
          "message": "Extending validation (#35)\n\n* test: Checking version from binary dataset\r\n\r\n* Validating volume with dedicated reference file\r\n\r\n* feat: Modules list of variables\r\n\r\n* feat: Matlab file with variables for validation\r\n\r\nAll checking variables are inside a cell, which is currently not\r\naccessible through Rust's matlab library.\r\n\r\n* feat: Creating validation files for each module\r\n\r\n* refactor: Validating with module specific binary reference\r\n\r\n* test: Using epsilon as tolerance to test dyamic_enthalpy\r\n\r\n* wip: testing creation of validation binary\r\n\r\n* Duplicate specvol validation test\r\n\r\nI'm not sure what happened here.\r\n\r\n* fix: Security vulnerability with regex\r\n\r\nThe former version of criterion allowed a vulnerable version of regex.\r\n\r\n* Updating validation filename\r\n\r\n* Extracting some variables to validate conversions\r\n\r\n* Adding validation datasets\r\n\r\n* Reverting criterion to 0.3\r\n\r\n* Updating Cargo.lock\r\n\r\n* Adding more variables to validate conversions\r\n\r\n* Adding missing cast references to validate conversions\r\n\r\n* Initializing conversions validation",
          "timestamp": "2022-09-23T10:56:28-06:00",
          "tree_id": "53c873f47357ef6fa9e90c29d18a7cc5bfe2264e",
          "url": "https://github.com/castelao/GSW-rs/commit/08af2d271379df7ed160787cc7cdce337b90d49f"
        },
        "date": 1663953003304,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 47,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 76,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 77,
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
            "value": 53,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 204,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 53,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 205,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 26,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 55,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 54,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 274,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 70,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 49,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 429,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 70,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 148,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 284,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 83,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 61,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 83,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 211,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 209,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 52,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 58,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 52,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 102,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 69,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 100,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 177,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 178,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 816,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 29,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 29,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 12,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 30,
            "range": "± 3",
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
          "id": "6f697268755bc86aa3e455ee7a396254c26d0075",
          "message": "Addressing CVE 2022 24713 (#42)\n\n* Forcing regex update due to security vulnerability\r\n\r\nIndirect dependency that should be eventually removed.\r\n\r\n* Upgrade to 0.0.11\r\n\r\n* Updating lock",
          "timestamp": "2022-09-23T12:03:50-06:00",
          "tree_id": "f580332d81ae74298b4763baa51543389442f55b",
          "url": "https://github.com/castelao/GSW-rs/commit/6f697268755bc86aa3e455ee7a396254c26d0075"
        },
        "date": 1663957000978,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 69,
            "range": "± 0",
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
            "value": 48,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 189,
            "range": "± 7",
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
            "range": "± 0",
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
            "value": 49,
            "range": "± 2",
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
            "value": 47,
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
            "value": 395,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 65,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 244,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 70,
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
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 186,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 183,
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
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 94,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 63,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 92,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 720,
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
            "value": 24,
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
            "value": 27,
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
          "id": "d53deec74d29ae1e54aa70be3d3c5fba648e110c",
          "message": "Improving earth module (#43)\n\n* Expanding documentation for f and g\r\n\r\nAdding arguments and return\r\n\r\n* Adding error type OutOfBounds\r\n\r\n* Adding references\r\n\r\n* Using OutOfBounds type of error\r\n\r\n* Upgrading versions to 0.1.0\r\n\r\n* Return NaN if input of coriollis is NaN\r\n\r\n* Return NaN if inputs for gravity() are NaN\r\n\r\n* Return distance NaN if input lat is NaN",
          "timestamp": "2022-09-29T00:15:04-06:00",
          "tree_id": "fceef0d60542ec20547065ccd8a1ea6440577868",
          "url": "https://github.com/castelao/GSW-rs/commit/d53deec74d29ae1e54aa70be3d3c5fba648e110c"
        },
        "date": 1664432820795,
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
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 59,
            "range": "± 0",
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
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 162,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 39,
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
            "value": 49,
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
            "value": 329,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 109,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 197,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 155,
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
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 51,
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
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 135,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 601,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 19,
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
            "value": 22,
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
          "id": "2fc6e01d68f2baa8d9c877c9991bb409c0a0840b",
          "message": "Gibbs (#44)\n\n* List of missing functions to implement\r\n\r\n* Moving internal functions into a module\r\n\r\nIt's already getting too long, so better split it in smaller\r\nsub-modules.\r\n\r\n* style: Breaking numbers to make it easier to read\r\n\r\n* Extenting tests\r\n\r\n* Testing gibbs()\r\n\r\n* style: Addressing excessive precision",
          "timestamp": "2022-10-01T15:15:14-06:00",
          "tree_id": "55683146ab517dbccb8553896d92fc6c0f08c0b0",
          "url": "https://github.com/castelao/GSW-rs/commit/2fc6e01d68f2baa8d9c877c9991bb409c0a0840b"
        },
        "date": 1664659644570,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 61,
            "range": "± 0",
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
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 18,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 209,
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
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 341,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 112,
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
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 68,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 161,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 153,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 38,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 79,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 53,
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
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 618,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 19,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 19,
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
            "value": 22,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}