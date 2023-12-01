window.BENCHMARK_DATA = {
  "lastUpdate": 1701409886637,
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
          "id": "5080090deaafc22c1148b22a0b55d52bea6fe6fe",
          "message": "Create CODE_OF_CONDUCT.md (#18)",
          "timestamp": "2022-10-02T22:02:41-06:00",
          "tree_id": "c4f4c1d423ddfa21696da800bdd1fbd12f34c622",
          "url": "https://github.com/castelao/GSW-rs/commit/5080090deaafc22c1148b22a0b55d52bea6fe6fe"
        },
        "date": 1664770527366,
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
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_anom_standard",
            "value": 47,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 191,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 192,
            "range": "± 8",
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
            "value": 47,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 47,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 240,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 59,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 44,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 390,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 65,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 128,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 237,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 70,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 54,
            "range": "± 1",
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
            "value": 193,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 182,
            "range": "± 2",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 44,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 89,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 89,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 156,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 154,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 727,
            "range": "± 26",
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
          "id": "8d64637888c2423bbc7cddbdfa241a696be4bb9a",
          "message": "bugfix: Missing a term in the specvol second derivatives (#46)\n\n* fix: Missing a term in the equation\r\n\r\nIdentified by @PaulMBarker and reported on TEOS-10/GSW-Matlab@38c9635\r\n\r\n* fixing test values due to that missing term\r\n\r\n* Split enthalpy validation dataset\r\n\r\nIt was getting too large to fit in the stack\r\n\r\n* Reducing serialized validation dataset\r\n\r\nIt doesn't have to be so large.\r\n\r\n* Testing roundtrip between S_p & R\r\n\r\n* Validating practical_salinity\r\n\r\n* Validate using R_from_SP from validation dataset\r\n\r\nNote that those values match the output from R_from_SP in Rust.\r\n\r\n* Updating serialized templates\r\n\r\nIncreasing scalar array size\r\n\r\n* Cleaning practical_salinity validtion\r\n\r\n* Allowing t68_from_t90() within crate\r\n\r\n* Validation dataset for practical_salinity module\r\n\r\nThat dataset came from Matlab distribution.\r\n\r\n* Updating validation datasets\r\n\r\nRe-distributing variables to reduce load in the stack.\r\n\r\n* It was missing SP_salinometer & Rt_chck_cast\r\n\r\n* A comment on the largest error that I found\r\n\r\n* Diverting enthalpy tests to the new validation test file\r\n\r\n* Using t68_from_t90(), DRY\r\n\r\n* Extending test values for hill_ratio_at_sp2()\r\n\r\nI was searching for the source of a bug and tried to improve this\r\ncoverage.\r\n\r\n* Ignoring validtion datasets within support crate\r\n\r\nThe bin files in the main library are not ignored.\r\n\r\n* Cleaning empty line\r\n\r\n* test: specvol_second_derivatives()\r\n\r\n* Upgrading validation dataset to 3.06.16\r\n\r\n* Minor change inline doc\r\n\r\n* Validating SP_salinometer()\r\n\r\n* Upgrade to 0.1.1\r\n\r\n* Update Cargo.lock\r\n\r\n* Upgrading minimum supported version to 1.57\r\n\r\n* Renaming constants used on r_from_sp()\r\n\r\nIt looks like the S* were renamed to R*, but the values kept the same.\r\n\r\n* clippy, breaking excessive prevision",
          "timestamp": "2022-10-06T23:57:19-06:00",
          "tree_id": "06f4bb8ff1a325cf70cce0f155edb6ffe24fb9e5",
          "url": "https://github.com/castelao/GSW-rs/commit/8d64637888c2423bbc7cddbdfa241a696be4bb9a"
        },
        "date": 1665123056228,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 74,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 74,
            "range": "± 1",
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
            "value": 208,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 52,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 208,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 51,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 52,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 48,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 51,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 268,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 68,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 47,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 414,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 67,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 142,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 263,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 84,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 57,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 76,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 203,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 205,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 51,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 50,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 93,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 62,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 95,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 170,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 170,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 781,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 28,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 29,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 12,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 29,
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
          "id": "c9a2d838379d6ec5abe5e60d033a9f7eea74dd38",
          "message": "Improving volume docs (#29)\n\n* Typo in alpha_beta()'s documentation\r\n\r\n* Improving documentation\r\n\r\n* doc: Brief intro to features used\r\n\r\n* Extending the library intro documentation\r\n\r\n* Typos\r\n\r\n* Minor improvements\r\n\r\n* Extending the landing page documentation\r\n\r\n* Updating version in how to cite",
          "timestamp": "2022-10-14T12:39:08-06:00",
          "tree_id": "ca675ae1e08adfad5b852934bba1f71024234b13",
          "url": "https://github.com/castelao/GSW-rs/commit/c9a2d838379d6ec5abe5e60d033a9f7eea74dd38"
        },
        "date": 1665773497949,
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
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 157,
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
            "value": 157,
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
            "value": 204,
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
            "value": 330,
            "range": "± 5",
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
            "value": 108,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 202,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 62,
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
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 156,
            "range": "± 3",
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
            "value": 41,
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
            "value": 74,
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
            "value": 602,
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
          "id": "d3cae86dffcb123e9ba023602a4f151f5e7e74e5",
          "message": "Extending citing recommendations (#47)\n\n* Not endorsed\r\n\r\nI just realized that we shall not give the wrong impression that GSW-rs\r\nwas anyhow inspected or endorsed by TEOS-10 committee.\r\n\r\n* Adding references to be cited\r\n\r\nI'm still working on this.\r\n\r\n* Typo\r\n\r\n* Extend to more places that it's not endorsed\r\n\r\n* wip: Some convincing to cite everyone",
          "timestamp": "2022-10-26T20:02:17-06:00",
          "tree_id": "71f9f66a13e44ead9ed47bd605633beef88bbd3a",
          "url": "https://github.com/castelao/GSW-rs/commit/d3cae86dffcb123e9ba023602a4f151f5e7e74e5"
        },
        "date": 1666836999696,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 36,
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
            "value": 157,
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
            "value": 157,
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
            "value": 204,
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
            "value": 329,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 108,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 202,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 62,
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
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 156,
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
            "value": 41,
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
            "value": 74,
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
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 601,
            "range": "± 2",
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
          "id": "d1136c20da7bb225a6cfb4052a79470b41827153",
          "message": "Updating codecov-action (#52)",
          "timestamp": "2023-06-12T12:33:05-06:00",
          "tree_id": "144dd429ef80dac95dde2de1bab663041c37c18b",
          "url": "https://github.com/castelao/GSW-rs/commit/d1136c20da7bb225a6cfb4052a79470b41827153"
        },
        "date": 1686595698046,
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
            "value": 80,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 72,
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
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 193,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 48,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 194,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 48,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 234,
            "range": "± 3",
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
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 377,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 59,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 116,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 232,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 74,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 90,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 187,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 178,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 54,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 95,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 64,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 150,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 152,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 762,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 24,
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
          "id": "7e91742c8424891a0e36ec674202179f5c5a7ce6",
          "message": "Fix predicates (#53)\n\n* Minimum Rust version updated to 1.64\r\n\r\n* Updating Cargo.lock\r\n\r\n* Downgrading predicates to 2.1.0",
          "timestamp": "2023-08-14T16:38:05-06:00",
          "tree_id": "369de71e5fa4a362ac75ccbc1ebb258ccbd1a31a",
          "url": "https://github.com/castelao/GSW-rs/commit/7e91742c8424891a0e36ec674202179f5c5a7ce6"
        },
        "date": 1692053444106,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 34,
            "range": "± 2",
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
            "value": 169,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 159,
            "range": "± 3",
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
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 197,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 304,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 199,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 64,
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
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 148,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 43,
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
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 593,
            "range": "± 1",
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
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "126987327+brycemines@users.noreply.github.com",
            "name": "brycemines",
            "username": "brycemines"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "39b081093156735dada47d5fee8e17870f907883",
          "message": "Gsw gibbs ice (#51)\n\n* Create gibbs_ice signiture\r\n\r\n* In progress gibbs_ice function\r\n\r\n* Added and used complex number crate\r\n\r\n* Added tests and close to completing funciton\r\n\r\n* fix: removing error from unfinished function\r\n\r\n* fix: change constant names to unique uppercase\r\n\r\n* fix: complete the gibbs_ice and is returning value\r\n\r\n* test similar to check_values for checking complex\r\n\r\n* fix: rewrite out_of_bounds() test\r\n\r\n* style: rewrite constants consistently\r\n\r\n* wip: rewriting and replacing num-complex\r\n\r\n* Removed repeated variable assignments\r\n\r\n* rename and update exploration test\r\n\r\n* style: add spacing to the constants\r\n\r\n* remove half num complex from 2 0\r\n\r\n* Roll back to working num-complex\r\n\r\n* remove unneeded and unused test\r\n\r\n* add documentation comment to gibbs_ice\r\n\r\n* add variable types to documentation\r\n\r\n* style: remove unneeded retrun statements\r\n\r\n* remove unnecessary variable ans\r\n\r\n* add to contributors on .zenodo.json\r\n\r\n* fix .zenodo.json\r\n\r\n* Create gibbs_ice signiture\r\n\r\n* In progress gibbs_ice function\r\n\r\n* Added and used complex number crate\r\n\r\n* Added tests and close to completing funciton\r\n\r\n* fix: removing error from unfinished function\r\n\r\n* fix: change constant names to unique uppercase\r\n\r\n* fix: complete the gibbs_ice and is returning value\r\n\r\n* test similar to check_values for checking complex\r\n\r\n* fix: rewrite out_of_bounds() test\r\n\r\n* style: rewrite constants consistently\r\n\r\n* wip: rewriting and replacing num-complex\r\n\r\n* Removed repeated variable assignments\r\n\r\n* rename and update exploration test\r\n\r\n* style: add spacing to the constants\r\n\r\n* remove half num complex from 2 0\r\n\r\n* Roll back to working num-complex\r\n\r\n* remove unneeded and unused test\r\n\r\n* add documentation comment to gibbs_ice\r\n\r\n* add variable types to documentation\r\n\r\n* style: remove unneeded retrun statements\r\n\r\n* remove unnecessary variable ans\r\n\r\n* add to contributors on .zenodo.json\r\n\r\n* fix .zenodo.json\r\n\r\n* Create gibbs_ice signiture\r\n\r\n* In progress gibbs_ice function\r\n\r\n* Added and used complex number crate\r\n\r\n* Added tests and close to completing funciton\r\n\r\n* fix: removing error from unfinished function\r\n\r\n* fix: change constant names to unique uppercase\r\n\r\n* fix: complete the gibbs_ice and is returning value\r\n\r\n* test similar to check_values for checking complex\r\n\r\n* fix: rewrite out_of_bounds() test\r\n\r\n* style: rewrite constants consistently\r\n\r\n* wip: rewriting and replacing num-complex\r\n\r\n* Removed repeated variable assignments\r\n\r\n* rename and update exploration test\r\n\r\n* style: add spacing to the constants\r\n\r\n* remove half num complex from 2 0\r\n\r\n* Roll back to working num-complex\r\n\r\n* remove unneeded and unused test\r\n\r\n* add documentation comment to gibbs_ice\r\n\r\n* add variable types to documentation\r\n\r\n* style: remove unneeded retrun statements\r\n\r\n* remove unnecessary variable ans\r\n\r\n* add to contributors on .zenodo.json\r\n\r\n* fix .zenodo.json\r\n\r\n* Revert Cargo.lock to main version and add num-complex\r\n\r\n---------\r\n\r\nCo-authored-by: Guilherme Castelão <guilherme@castelao.net>",
          "timestamp": "2023-08-16T09:35:21-06:00",
          "tree_id": "30f6fd216e75ccc9a9d566c8a9429f59cfcb373f",
          "url": "https://github.com/castelao/GSW-rs/commit/39b081093156735dada47d5fee8e17870f907883"
        },
        "date": 1692200900675,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 72,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 76,
            "range": "± 3",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 189,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 193,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 50,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 51,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 238,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 64,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 363,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 58,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 121,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 238,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 79,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 55,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 94,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 182,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 176,
            "range": "± 2",
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
            "value": 53,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 59,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 95,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 155,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 153,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 722,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 24,
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
            "value": 21,
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
          "id": "f6f10a53f5564683c22031e5a83e05ac25aade9e",
          "message": "Updating test to use latest GSW-Python (#50)\n\n* Testing with latest GSW-Python\r\n\r\n* Updating patch to reflect new setup.py\r\n\r\n* add new private lib needed on windows, ntdll.lib\r\n\r\n* lint: Missed format from another PR\r\n\r\nThe format corrections are in a part unrealted to this PR.\r\n\r\n* lint: formating excessive precision",
          "timestamp": "2023-08-17T19:23:37-06:00",
          "tree_id": "177a2ce2a50bd2c6bf00ad068f0861fd32be609f",
          "url": "https://github.com/castelao/GSW-rs/commit/f6f10a53f5564683c22031e5a83e05ac25aade9e"
        },
        "date": 1692322567893,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 34,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 57,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 61,
            "range": "± 3",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 158,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 156,
            "range": "± 9",
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
            "value": 42,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 43,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 40,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 41,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 192,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 53,
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
            "value": 311,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 48,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 99,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 195,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 62,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 46,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 75,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 151,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 152,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 41,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 43,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 79,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 51,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 75,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 126,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 128,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 585,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 21,
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
            "value": 19,
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
          "id": "3555869f77d753a777276ccc095c6ca8759c396d",
          "message": "Bump version (#54)\n\nNew function gsw_internal_funcs::gibbs_ice(), thanks to @brycemines.",
          "timestamp": "2023-08-17T22:18:55-06:00",
          "tree_id": "6bce61367a021c430e1b918a7cd7986a80da6695",
          "url": "https://github.com/castelao/GSW-rs/commit/3555869f77d753a777276ccc095c6ca8759c396d"
        },
        "date": 1692333094320,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 64,
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
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 158,
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
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 42,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 42,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 210,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 305,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 198,
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
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 148,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 43,
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
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 593,
            "range": "± 1",
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
            "value": 20,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sp_salinometer",
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 17,
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
          "id": "1e052457f2ecd90ea9cedaec4a25fe645119cca7",
          "message": "Adjusting license to conform with other languages (#55)\n\n* Adjusting license to conform with other languages\r\n* fix README\r\n\r\n---------\r\n\r\nCo-authored-by: Luiz Irber <contact+github@luizirber.org>",
          "timestamp": "2023-08-20T14:04:49-07:00",
          "tree_id": "7962103a3533c83821f89199445d586d2158b82c",
          "url": "https://github.com/castelao/GSW-rs/commit/1e052457f2ecd90ea9cedaec4a25fe645119cca7"
        },
        "date": 1692566226888,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 32,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 54,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 56,
            "range": "± 1",
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
            "value": 38,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 159,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 38,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 156,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 16,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 41,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 197,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 52,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 282,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 46,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 93,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 187,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 62,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 72,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 140,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 138,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 40,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 39,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 77,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 45,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 71,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 119,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 121,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 615,
            "range": "± 48",
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
            "value": 20,
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
            "value": 19,
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
          "id": "e6c2f6dcc953094963952e569f318b60a71c68bb",
          "message": "Explaining the repository contents (#48)\n\n* Cosmetics: Emphasizing the available features\r\n\r\n* Describing the repository\r\n\r\nThere are too many extra content that might confuse anyone new to Rust.\r\n\r\n* Minor improvement in the intro text\r\n\r\n* Improving guidance for contributiors\r\n\r\n* Pointing how to cite\r\n\r\nLet's avoid redundancies and the risk of building inconsistencies and\r\npoint to citing instruction in the README.\r\n\r\n* Typos\r\n\r\n* style: Fixing spaces and line break\r\n\r\n* Typo\r\n\r\n* References as items\r\n\r\nListing as items to emphasize in the text and minimize confusion.\r\n\r\n* A small note on the tests directory\r\n\r\n* typo\r\n\r\n* Upgrading version to update documentation",
          "timestamp": "2023-08-20T23:40:44-06:00",
          "tree_id": "3867cd676d1597c2ccff1d6a7268716bea539635",
          "url": "https://github.com/castelao/GSW-rs/commit/e6c2f6dcc953094963952e569f318b60a71c68bb"
        },
        "date": 1692597233684,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 70,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 73,
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
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 190,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 190,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 237,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 62,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 365,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 57,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 119,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 253,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 55,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 92,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 179,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 176,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 53,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 97,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 91,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 155,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 161,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 712,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 24,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 24,
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
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b2884cad57d8c7eaf973235166554442ed298135",
          "message": "Build(deps): Bump bumpalo from 3.11.0 to 3.13.0 (#56)\n\nBumps [bumpalo](https://github.com/fitzgen/bumpalo) from 3.11.0 to 3.13.0.\r\n- [Changelog](https://github.com/fitzgen/bumpalo/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/fitzgen/bumpalo/compare/3.11.0...3.13.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: bumpalo\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-21T15:49:48-06:00",
          "tree_id": "a5c596ab8efde2f117abe4d161a44778825686d9",
          "url": "https://github.com/castelao/GSW-rs/commit/b2884cad57d8c7eaf973235166554442ed298135"
        },
        "date": 1692655337438,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 34,
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
            "value": 158,
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
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 201,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 52,
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
            "value": 304,
            "range": "± 1",
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
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 198,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 64,
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
            "value": 83,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 152,
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
            "value": 42,
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
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 593,
            "range": "± 0",
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
            "value": 20,
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
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5305c54ac9c223c8d1291eaab6372d25bcbfc42a",
          "message": "Build(deps): Bump spin from 0.9.4 to 0.9.8 (#57)\n\nBumps [spin](https://github.com/mvdnes/spin-rs) from 0.9.4 to 0.9.8.\r\n- [Changelog](https://github.com/mvdnes/spin-rs/blob/master/CHANGELOG.md)\r\n- [Commits](https://github.com/mvdnes/spin-rs/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: spin\r\n  dependency-type: indirect\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2023-08-21T16:27:04-06:00",
          "tree_id": "fa8b365efa5e48454ff0d490ea64130a127234e2",
          "url": "https://github.com/castelao/GSW-rs/commit/5305c54ac9c223c8d1291eaab6372d25bcbfc42a"
        },
        "date": 1692657616363,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 72,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 78,
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
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 200,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 50,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 190,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma0",
            "value": 21,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 50,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 51,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 52,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 248,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 47,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 362,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 63,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 118,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 249,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 58,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 96,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 174,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 176,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 48,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 54,
            "range": "± 0",
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
            "value": 98,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 58,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 91,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 153,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 764,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "c_from_sp",
            "value": 24,
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
          "id": "51e7829f3ccfa384d0f45db321cb08896d76fe80",
          "message": "Several typos (#58)",
          "timestamp": "2023-08-24T09:58:29-06:00",
          "tree_id": "7a61c07e6ec91f423999d6cca28befdea5171d90",
          "url": "https://github.com/castelao/GSW-rs/commit/51e7829f3ccfa384d0f45db321cb08896d76fe80"
        },
        "date": 1692893466580,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 35,
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
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 41,
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
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 197,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 52,
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
            "value": 304,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 198,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 65,
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
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 151,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 147,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 43,
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
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 49,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 129,
            "range": "± 2",
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
            "value": 593,
            "range": "± 0",
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
            "value": 17,
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
          "id": "8a461edad1dccbdae07b177ce403949e503b5131",
          "message": "Conversion documentation (#59)\n\n* Initializing documentation for sr_from_sp\r\n\r\n* Guidance for missing functions\r\n\r\n* Tagging source code in documentation\r\n\r\n* doc: ct_from_pt()\r\n\r\n* doc: t90_from_t48()\r\n\r\n* doc: t68_from_t90()\r\n\r\n* Adding Saunders 1990\r\n\r\n* Adding Millero 2008 reference\r\n\r\n* typo: License in the README file\r\n\r\n* refactor: Re-routing t68_from_t90()\r\n\r\n* doc: Extending documentation\r\n\r\nNote that is it still missing some references, but I have to make time\r\nto carefully read each one.\r\n\r\n* doc: Adding some symbols\r\n\r\n* Upgrade version to reflect on the public documentation",
          "timestamp": "2023-09-12T23:38:18-06:00",
          "tree_id": "ede03558f0f247fa359122260be4ca7c420d2338",
          "url": "https://github.com/castelao/GSW-rs/commit/8a461edad1dccbdae07b177ce403949e503b5131"
        },
        "date": 1694584258041,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 64,
            "range": "± 1",
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
            "value": 158,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 41,
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
            "value": 17,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma1",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 41,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 197,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 52,
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
            "value": 304,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 198,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 64,
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
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 150,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 148,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 42,
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
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 48,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 76,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 128,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 129,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 593,
            "range": "± 3",
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
            "value": 20,
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
            "value": 19,
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
          "id": "449717b5be4530bdda2047d31de3e4bf8f6dd07c",
          "message": "doc: Some guidance on how to use it (#60)\n\nAnyone new to Rust might feel lost how to start. Let's give a minumum\r\nguidance here.",
          "timestamp": "2023-11-19T14:24:34-07:00",
          "tree_id": "d0c724df6d8e7e156e73f05a9ac4e8bb1d5c18f0",
          "url": "https://github.com/castelao/GSW-rs/commit/449717b5be4530bdda2047d31de3e4bf8f6dd07c"
        },
        "date": 1700429701831,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 45,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 52,
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
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 128,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 138,
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
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 147,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 229,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 36,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 80,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 151,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 113,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 107,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 484,
            "range": "± 4",
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
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "michal.sojka@cvut.cz",
            "name": "Michal Sojka",
            "username": "wentasah"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7cea3420d7d35d0d6f11b083cb480b8607e0ddeb",
          "message": "Fix typo (#65)",
          "timestamp": "2023-11-27T09:52:12-07:00",
          "tree_id": "e2d328d3deb57c57db44bce46f7f17385bf7f56f",
          "url": "https://github.com/castelao/GSW-rs/commit/7cea3420d7d35d0d6f11b083cb480b8607e0ddeb"
        },
        "date": 1701104558997,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 52,
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
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 128,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 138,
            "range": "± 0",
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
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 148,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 229,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 152,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 56,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 113,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 107,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 65,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 40,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 484,
            "range": "± 2",
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
            "value": 8,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "z_from_p",
            "value": 13,
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
          "id": "82c0f9757c54ac09d2301583fba15becbb3c48b5",
          "message": "List of functions from GSW-C (#62)\n\n* List of functions from GSW-C\r\n\r\nAdressing #61.\r\n\r\nLet's start with a target list of functions implemented in GSW-C. Maybe\r\nmark what was already implemented in Rust.\r\n\r\n* Using checkboxes\r\n\r\n* Updating already implemented list\r\n\r\n* Listing missing functions in module conversions\r\n\r\nClearly conversions should be the first target.\r\n\r\n* Minimum explanation on what is the inventory\r\n\r\n* Organizing this list\r\n\r\n* typo: Thanks @efiring\r\n\r\n* Re-organizing lists per modules\r\n\r\n* Updating readme about missing functions\r\n\r\n* Adding the 'gsw_' prefix\r\n\r\nAlthough we don't use it in Rust, since it is explicit by the crate plus\r\nmodule (ex. gsw::earth::distance), let's add it here for consistency.",
          "timestamp": "2023-11-30T22:34:45-07:00",
          "tree_id": "921271da516d47eebcfa1057a111e608924f1939",
          "url": "https://github.com/castelao/GSW-rs/commit/82c0f9757c54ac09d2301583fba15becbb3c48b5"
        },
        "date": 1701409503540,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 47,
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
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 124,
            "range": "± 0",
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
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 147,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 230,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 151,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 45,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 113,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 64,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 499,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 17,
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
            "value": 13,
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
          "id": "92be3c9b9cfee6b9d56d022e95ee2eced3eec5bd",
          "message": "Typo and fixing link (#66)",
          "timestamp": "2023-11-30T22:41:04-07:00",
          "tree_id": "75fbe16b8f982451254d52230f175565fca2d73f",
          "url": "https://github.com/castelao/GSW-rs/commit/92be3c9b9cfee6b9d56d022e95ee2eced3eec5bd"
        },
        "date": 1701409886258,
        "tool": "cargo",
        "benches": [
          {
            "name": "specvol",
            "value": 27,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha",
            "value": 46,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "beta",
            "value": 47,
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
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_alpha_beta",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho",
            "value": 33,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_alpha_beta",
            "value": 125,
            "range": "± 4",
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
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma2",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma3",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sigma4",
            "value": 34,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "cabbeling",
            "value": 147,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sound_speed",
            "value": 43,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "dynamic_enthalpy",
            "value": 31,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sa_from_rho",
            "value": 229,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_first_derivatives_wrt_enthalpy",
            "value": 78,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives",
            "value": 152,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "specvol_second_derivatives_wrt_enthalpy",
            "value": 44,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "alpha_on_beta",
            "value": 36,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_first_derivatives",
            "value": 56,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "rho_second_derivatives",
            "value": 113,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "thermobaric",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy",
            "value": 35,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_first_derivatives",
            "value": 37,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_second_derivatives",
            "value": 32,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "enthalpy_diff",
            "value": 64,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "kappa",
            "value": 39,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy",
            "value": 60,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_first_derivatives",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "internal_energy_second_derivatives",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "ct_maxdensity",
            "value": 499,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sp_from_c",
            "value": 17,
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
            "value": 13,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}