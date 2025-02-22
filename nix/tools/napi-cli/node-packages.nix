# This file has been generated by node2nix 1.11.1. Do not edit!

{
  nodeEnv,
  fetchurl,
  fetchgit,
  nix-gitignore,
  stdenv,
  lib,
  globalBuildInputs ? [ ],
}:

let
  sources = {
    "@emnapi/runtime-1.3.1" = {
      name = "_at_emnapi_slash_runtime";
      packageName = "@emnapi/runtime";
      version = "1.3.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@emnapi/runtime/-/runtime-1.3.1.tgz";
        sha512 = "kEBmG8KyqtxJZv+ygbEim+KCGtIq1fC22Ms3S4ziXmYKm8uyoLX0MHONVKwp+9opg390VaKRNt4a7A9NwmpNhw==";
      };
    };
    "@inquirer/checkbox-4.0.4" = {
      name = "_at_inquirer_slash_checkbox";
      packageName = "@inquirer/checkbox";
      version = "4.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/checkbox/-/checkbox-4.0.4.tgz";
        sha512 = "fYAKCAcGNMdfjL6hZTRUwkIByQ8EIZCXKrIQZH7XjADnN/xvRUhj8UdBbpC4zoUzvChhkSC/zRKaP/tDs3dZpg==";
      };
    };
    "@inquirer/confirm-5.1.1" = {
      name = "_at_inquirer_slash_confirm";
      packageName = "@inquirer/confirm";
      version = "5.1.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/confirm/-/confirm-5.1.1.tgz";
        sha512 = "vVLSbGci+IKQvDOtzpPTCOiEJCNidHcAq9JYVoWTW0svb5FiwSLotkM+JXNXejfjnzVYV9n0DTBythl9+XgTxg==";
      };
    };
    "@inquirer/core-10.1.2" = {
      name = "_at_inquirer_slash_core";
      packageName = "@inquirer/core";
      version = "10.1.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/core/-/core-10.1.2.tgz";
        sha512 = "bHd96F3ezHg1mf/J0Rb4CV8ndCN0v28kUlrHqP7+ECm1C/A+paB7Xh2lbMk6x+kweQC+rZOxM/YeKikzxco8bQ==";
      };
    };
    "@inquirer/editor-4.2.1" = {
      name = "_at_inquirer_slash_editor";
      packageName = "@inquirer/editor";
      version = "4.2.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/editor/-/editor-4.2.1.tgz";
        sha512 = "xn9aDaiP6nFa432i68JCaL302FyL6y/6EG97nAtfIPnWZ+mWPgCMLGc4XZ2QQMsZtu9q3Jd5AzBPjXh10aX9kA==";
      };
    };
    "@inquirer/expand-4.0.4" = {
      name = "_at_inquirer_slash_expand";
      packageName = "@inquirer/expand";
      version = "4.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/expand/-/expand-4.0.4.tgz";
        sha512 = "GYocr+BPyxKPxQ4UZyNMqZFSGKScSUc0Vk17II3J+0bDcgGsQm0KYQNooN1Q5iBfXsy3x/VWmHGh20QnzsaHwg==";
      };
    };
    "@inquirer/figures-1.0.9" = {
      name = "_at_inquirer_slash_figures";
      packageName = "@inquirer/figures";
      version = "1.0.9";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/figures/-/figures-1.0.9.tgz";
        sha512 = "BXvGj0ehzrngHTPTDqUoDT3NXL8U0RxUk2zJm2A66RhCEIWdtU1v6GuUqNAgArW4PQ9CinqIWyHdQgdwOj06zQ==";
      };
    };
    "@inquirer/input-4.1.1" = {
      name = "_at_inquirer_slash_input";
      packageName = "@inquirer/input";
      version = "4.1.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/input/-/input-4.1.1.tgz";
        sha512 = "nAXAHQndZcXB+7CyjIW3XuQZZHbQQ0q8LX6miY6bqAWwDzNa9JUioDBYrFmOUNIsuF08o1WT/m2gbBXvBhYVxg==";
      };
    };
    "@inquirer/number-3.0.4" = {
      name = "_at_inquirer_slash_number";
      packageName = "@inquirer/number";
      version = "3.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/number/-/number-3.0.4.tgz";
        sha512 = "DX7a6IXRPU0j8kr2ovf+QaaDiIf+zEKaZVzCWdLOTk7XigqSXvoh4cul7x68xp54WTQrgSnW7P1WBJDbyY3GhA==";
      };
    };
    "@inquirer/password-4.0.4" = {
      name = "_at_inquirer_slash_password";
      packageName = "@inquirer/password";
      version = "4.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/password/-/password-4.0.4.tgz";
        sha512 = "wiliQOWdjM8FnBmdIHtQV2Ca3S1+tMBUerhyjkRCv1g+4jSvEweGu9GCcvVEgKDhTBT15nrxvk5/bVrGUqSs1w==";
      };
    };
    "@inquirer/prompts-7.2.1" = {
      name = "_at_inquirer_slash_prompts";
      packageName = "@inquirer/prompts";
      version = "7.2.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/prompts/-/prompts-7.2.1.tgz";
        sha512 = "v2JSGri6/HXSfoGIwuKEn8sNCQK6nsB2BNpy2lSX6QH9bsECrMv93QHnj5+f+1ZWpF/VNioIV2B/PDox8EvGuQ==";
      };
    };
    "@inquirer/rawlist-4.0.4" = {
      name = "_at_inquirer_slash_rawlist";
      packageName = "@inquirer/rawlist";
      version = "4.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/rawlist/-/rawlist-4.0.4.tgz";
        sha512 = "IsVN2EZdNHsmFdKWx9HaXb8T/s3FlR/U1QPt9dwbSyPtjFbMTlW9CRFvnn0bm/QIsrMRD2oMZqrQpSWPQVbXXg==";
      };
    };
    "@inquirer/search-3.0.4" = {
      name = "_at_inquirer_slash_search";
      packageName = "@inquirer/search";
      version = "3.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/search/-/search-3.0.4.tgz";
        sha512 = "tSkJk2SDmC2MEdTIjknXWmCnmPr5owTs9/xjfa14ol1Oh95n6xW7SYn5fiPk4/vrJPys0ggSWiISdPze4LTa7A==";
      };
    };
    "@inquirer/select-4.0.4" = {
      name = "_at_inquirer_slash_select";
      packageName = "@inquirer/select";
      version = "4.0.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/select/-/select-4.0.4.tgz";
        sha512 = "ZzYLuLoUzTIW9EJm++jBpRiTshGqS3Q1o5qOEQqgzaBlmdsjQr6pA4TUNkwu6OBYgM2mIRbCz6mUhFDfl/GF+w==";
      };
    };
    "@inquirer/type-3.0.2" = {
      name = "_at_inquirer_slash_type";
      packageName = "@inquirer/type";
      version = "3.0.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@inquirer/type/-/type-3.0.2.tgz";
        sha512 = "ZhQ4TvhwHZF+lGhQ2O/rsjo80XoZR5/5qhOY3t6FJuX5XBg5Be8YzYTvaUGJnc12AUGI2nr4QSUE4PhKSigx7g==";
      };
    };
    "@napi-rs/cross-toolchain-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain";
      packageName = "@napi-rs/cross-toolchain";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain/-/cross-toolchain-0.0.16.tgz";
        sha512 = "jwdjHT5L0m9MH0CmzDwPp0ckn/UO7afHCsPeo7NugHUvYgvlgS7SWhdMVgIgJW2HHqhcW/2nhaLLGpAU1c7QRQ==";
      };
    };
    "@napi-rs/cross-toolchain-arm64-target-aarch64-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain-arm64-target-aarch64";
      packageName = "@napi-rs/cross-toolchain-arm64-target-aarch64";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain-arm64-target-aarch64/-/cross-toolchain-arm64-target-aarch64-0.0.16.tgz";
        sha512 = "eD7nPl2keafZAtMDdJpkoaeT6JoKosYoB1hhN6imPdl7/2bknyqlqD6xH5lhlUKEwdd9cnRagjM6AQ4+aPmnjw==";
      };
    };
    "@napi-rs/cross-toolchain-arm64-target-armv7-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain-arm64-target-armv7";
      packageName = "@napi-rs/cross-toolchain-arm64-target-armv7";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain-arm64-target-armv7/-/cross-toolchain-arm64-target-armv7-0.0.16.tgz";
        sha512 = "u/FvGygLQvEJxrSKxK6rfv7wttwSq+v95XZLpq0RAWRfWtnCDZL6PrqrFqyrK9khP7gkaFWf3M8EOq7Jr5Mjuw==";
      };
    };
    "@napi-rs/cross-toolchain-arm64-target-x86_64-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain-arm64-target-x86_64";
      packageName = "@napi-rs/cross-toolchain-arm64-target-x86_64";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain-arm64-target-x86_64/-/cross-toolchain-arm64-target-x86_64-0.0.16.tgz";
        sha512 = "wFSETFf69QEtKaR2W2wo3rfuKgc65mHRuQQv6Ishh83zBO8OhjLVLOSJj10NnJcq+8FUoL0jdwYMDx8/40yAHw==";
      };
    };
    "@napi-rs/cross-toolchain-x64-target-aarch64-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain-x64-target-aarch64";
      packageName = "@napi-rs/cross-toolchain-x64-target-aarch64";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain-x64-target-aarch64/-/cross-toolchain-x64-target-aarch64-0.0.16.tgz";
        sha512 = "wfyyjT1fLLV76ymIfU9k8mJsg3fbyPk7X1bLxZCrq94ZyW2cSVLse4oqrIk0AUjMvtZ7we0O0iesRMtNZ2nf2Q==";
      };
    };
    "@napi-rs/cross-toolchain-x64-target-armv7-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain-x64-target-armv7";
      packageName = "@napi-rs/cross-toolchain-x64-target-armv7";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain-x64-target-armv7/-/cross-toolchain-x64-target-armv7-0.0.16.tgz";
        sha512 = "Cnbk9eK9f54OP6Sq/b7h3yRxUTjoBM9LKytLr9ntNcLeRHhcRV7jSygXP2QEDr/uuSMsl1x7aGvKfSrfNZW+/w==";
      };
    };
    "@napi-rs/cross-toolchain-x64-target-x86_64-0.0.16" = {
      name = "_at_napi-rs_slash_cross-toolchain-x64-target-x86_64";
      packageName = "@napi-rs/cross-toolchain-x64-target-x86_64";
      version = "0.0.16";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/cross-toolchain-x64-target-x86_64/-/cross-toolchain-x64-target-x86_64-0.0.16.tgz";
        sha512 = "u1XjhuTL/KHzOl2rQwJZXUE2JxPIEtPZZnIAFgN4uTnjIEqRK5ODKqjEwoGuMomigBDA1/DR+ANWseN8tF+Q8w==";
      };
    };
    "@napi-rs/lzma-1.4.1" = {
      name = "_at_napi-rs_slash_lzma";
      packageName = "@napi-rs/lzma";
      version = "1.4.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/lzma/-/lzma-1.4.1.tgz";
        sha512 = "5f8K9NHjwHjZKGm3SS+7CFxXQhz8rbg2umBm/9g0xQRXBdYEI31N5z1ACuk9bmBQOusXAq9CArGfs/ZQso2rUA==";
      };
    };
    "@napi-rs/tar-0.1.4" = {
      name = "_at_napi-rs_slash_tar";
      packageName = "@napi-rs/tar";
      version = "0.1.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/tar/-/tar-0.1.4.tgz";
        sha512 = "hDsvmMZY8tl2CcLfjnTeE1o5W1eGTSL+ZIX8YEybtcJwA+Cc8SNHb7l6JqMnGcjOrWBZbHt8tzTN+W7qHS5Wmg==";
      };
    };
    "@napi-rs/wasm-tools-0.0.2" = {
      name = "_at_napi-rs_slash_wasm-tools";
      packageName = "@napi-rs/wasm-tools";
      version = "0.0.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@napi-rs/wasm-tools/-/wasm-tools-0.0.2.tgz";
        sha512 = "kBvDQCP5BLw2TxTENXLp3Of7vVEx0uyIye824JHE4dduzzOHVgSoOFVhVqAT3Fx/hLV445RVWfEqQbXMg4w/Mw==";
      };
    };
    "@octokit/auth-token-5.1.1" = {
      name = "_at_octokit_slash_auth-token";
      packageName = "@octokit/auth-token";
      version = "5.1.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/auth-token/-/auth-token-5.1.1.tgz";
        sha512 = "rh3G3wDO8J9wSjfI436JUKzHIxq8NaiL0tVeB2aXmG6p/9859aUOAjA9pmSPNGGZxfwmaJ9ozOJImuNVJdpvbA==";
      };
    };
    "@octokit/core-6.1.3" = {
      name = "_at_octokit_slash_core";
      packageName = "@octokit/core";
      version = "6.1.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/core/-/core-6.1.3.tgz";
        sha512 = "z+j7DixNnfpdToYsOutStDgeRzJSMnbj8T1C/oQjB6Aa+kRfNjs/Fn7W6c8bmlt6mfy3FkgeKBRnDjxQow5dow==";
      };
    };
    "@octokit/endpoint-10.1.2" = {
      name = "_at_octokit_slash_endpoint";
      packageName = "@octokit/endpoint";
      version = "10.1.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/endpoint/-/endpoint-10.1.2.tgz";
        sha512 = "XybpFv9Ms4hX5OCHMZqyODYqGTZ3H6K6Vva+M9LR7ib/xr1y1ZnlChYv9H680y77Vd/i/k+thXApeRASBQkzhA==";
      };
    };
    "@octokit/graphql-8.1.2" = {
      name = "_at_octokit_slash_graphql";
      packageName = "@octokit/graphql";
      version = "8.1.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/graphql/-/graphql-8.1.2.tgz";
        sha512 = "bdlj/CJVjpaz06NBpfHhp4kGJaRZfz7AzC+6EwUImRtrwIw8dIgJ63Xg0OzV9pRn3rIzrt5c2sa++BL0JJ8GLw==";
      };
    };
    "@octokit/openapi-types-22.2.0" = {
      name = "_at_octokit_slash_openapi-types";
      packageName = "@octokit/openapi-types";
      version = "22.2.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/openapi-types/-/openapi-types-22.2.0.tgz";
        sha512 = "QBhVjcUa9W7Wwhm6DBFu6ZZ+1/t/oYxqc2tp81Pi41YNuJinbFRx8B133qVOrAaBbF7D/m0Et6f9/pZt9Rc+tg==";
      };
    };
    "@octokit/plugin-paginate-rest-11.3.6" = {
      name = "_at_octokit_slash_plugin-paginate-rest";
      packageName = "@octokit/plugin-paginate-rest";
      version = "11.3.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/plugin-paginate-rest/-/plugin-paginate-rest-11.3.6.tgz";
        sha512 = "zcvqqf/+TicbTCa/Z+3w4eBJcAxCFymtc0UAIsR3dEVoNilWld4oXdscQ3laXamTszUZdusw97K8+DrbFiOwjw==";
      };
    };
    "@octokit/plugin-request-log-5.3.1" = {
      name = "_at_octokit_slash_plugin-request-log";
      packageName = "@octokit/plugin-request-log";
      version = "5.3.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/plugin-request-log/-/plugin-request-log-5.3.1.tgz";
        sha512 = "n/lNeCtq+9ofhC15xzmJCNKP2BWTv8Ih2TTy+jatNCCq/gQP/V7rK3fjIfuz0pDWDALO/o/4QY4hyOF6TQQFUw==";
      };
    };
    "@octokit/plugin-rest-endpoint-methods-13.2.6" = {
      name = "_at_octokit_slash_plugin-rest-endpoint-methods";
      packageName = "@octokit/plugin-rest-endpoint-methods";
      version = "13.2.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/plugin-rest-endpoint-methods/-/plugin-rest-endpoint-methods-13.2.6.tgz";
        sha512 = "wMsdyHMjSfKjGINkdGKki06VEkgdEldIGstIEyGX0wbYHGByOwN/KiM+hAAlUwAtPkP3gvXtVQA9L3ITdV2tVw==";
      };
    };
    "@octokit/request-9.1.4" = {
      name = "_at_octokit_slash_request";
      packageName = "@octokit/request";
      version = "9.1.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/request/-/request-9.1.4.tgz";
        sha512 = "tMbOwGm6wDII6vygP3wUVqFTw3Aoo0FnVQyhihh8vVq12uO3P+vQZeo2CKMpWtPSogpACD0yyZAlVlQnjW71DA==";
      };
    };
    "@octokit/request-error-6.1.6" = {
      name = "_at_octokit_slash_request-error";
      packageName = "@octokit/request-error";
      version = "6.1.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/request-error/-/request-error-6.1.6.tgz";
        sha512 = "pqnVKYo/at0NuOjinrgcQYpEbv4snvP3bKMRqHaD9kIsk9u1LCpb2smHZi8/qJfgeNqLo5hNW4Z7FezNdEo0xg==";
      };
    };
    "@octokit/rest-21.0.2" = {
      name = "_at_octokit_slash_rest";
      packageName = "@octokit/rest";
      version = "21.0.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/rest/-/rest-21.0.2.tgz";
        sha512 = "+CiLisCoyWmYicH25y1cDfCrv41kRSvTq6pPWtRroRJzhsCZWZyCqGyI8foJT5LmScADSwRAnr/xo+eewL04wQ==";
      };
    };
    "@octokit/types-13.6.2" = {
      name = "_at_octokit_slash_types";
      packageName = "@octokit/types";
      version = "13.6.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/@octokit/types/-/types-13.6.2.tgz";
        sha512 = "WpbZfZUcZU77DrSW4wbsSgTPfKcp286q3ItaIgvSbBpZJlu6mnYXAkjZz6LVZPXkEvLIM8McanyZejKTYUHipA==";
      };
    };
    "@types/node-22.10.5" = {
      name = "_at_types_slash_node";
      packageName = "@types/node";
      version = "22.10.5";
      src = fetchurl {
        url = "https://registry.npmjs.org/@types/node/-/node-22.10.5.tgz";
        sha512 = "F8Q+SeGimwOo86fiovQh8qiXfFEh2/ocYv7tU5pJ3EXMSSxk1Joj5wefpFK2fHTf/N6HKGSxIDBT9f3gCxXPkQ==";
      };
    };
    "ansi-escapes-4.3.2" = {
      name = "ansi-escapes";
      packageName = "ansi-escapes";
      version = "4.3.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/ansi-escapes/-/ansi-escapes-4.3.2.tgz";
        sha512 = "gKXj5ALrKWQLsYG9jlTRmR/xKluxHV+Z9QEwNIgCfM1/uwPMCuzVVnh5mwTd+OuBZcwSIMbqssNWRm1lE51QaQ==";
      };
    };
    "ansi-regex-5.0.1" = {
      name = "ansi-regex";
      packageName = "ansi-regex";
      version = "5.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/ansi-regex/-/ansi-regex-5.0.1.tgz";
        sha512 = "quJQXlTSUGL2LH9SUXo8VwsY4soanhgo6LNSm84E1LBcE8s3O0wpdiRzyR9z/ZZJMlMWv37qOOb9pdJlMUEKFQ==";
      };
    };
    "ansi-styles-4.3.0" = {
      name = "ansi-styles";
      packageName = "ansi-styles";
      version = "4.3.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/ansi-styles/-/ansi-styles-4.3.0.tgz";
        sha512 = "zbB9rCJAT1rbjiVDb2hqKFHNYLxgtk8NURxZ3IZwD3F6NtxbXZQCnnSi1Lkx+IDohdPlFp222wVALIheZJQSEg==";
      };
    };
    "argparse-2.0.1" = {
      name = "argparse";
      packageName = "argparse";
      version = "2.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/argparse/-/argparse-2.0.1.tgz";
        sha512 = "8+9WqebbFzpX9OR+Wa6O29asIogeRMzcGtAINdpMHHyAg10f05aSFVBbcEqGf/PXw1EjAZ+q2/bEBg3DvurK3Q==";
      };
    };
    "before-after-hook-3.0.2" = {
      name = "before-after-hook";
      packageName = "before-after-hook";
      version = "3.0.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/before-after-hook/-/before-after-hook-3.0.2.tgz";
        sha512 = "Nik3Sc0ncrMK4UUdXQmAnRtzmNQTAAXmXIopizwZ1W1t8QmfJj+zL4OA2I7XPTPW5z5TDqv4hRo/JzouDJnX3A==";
      };
    };
    "chardet-0.7.0" = {
      name = "chardet";
      packageName = "chardet";
      version = "0.7.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/chardet/-/chardet-0.7.0.tgz";
        sha512 = "mT8iDcrh03qDGRRmoA2hmBJnxpllMR+0/0qlzjqZES6NdiWDcZkCNAk4rPFZ9Q85r27unkiNNg8ZOiwZXBHwcA==";
      };
    };
    "cli-width-4.1.0" = {
      name = "cli-width";
      packageName = "cli-width";
      version = "4.1.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/cli-width/-/cli-width-4.1.0.tgz";
        sha512 = "ouuZd4/dm2Sw5Gmqy6bGyNNNe1qt9RpmxveLSO7KcgsTnU7RXfsw+/bukWGo1abgBiMAic068rclZsO4IWmmxQ==";
      };
    };
    "clipanion-3.2.1" = {
      name = "clipanion";
      packageName = "clipanion";
      version = "3.2.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/clipanion/-/clipanion-3.2.1.tgz";
        sha512 = "dYFdjLb7y1ajfxQopN05mylEpK9ZX0sO1/RfMXdfmwjlIsPkbh4p7A682x++zFPLDCo1x3p82dtljHf5cW2LKA==";
      };
    };
    "color-convert-2.0.1" = {
      name = "color-convert";
      packageName = "color-convert";
      version = "2.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/color-convert/-/color-convert-2.0.1.tgz";
        sha512 = "RRECPsj7iu/xb5oKYcsFHSppFNnsj/52OVTRKb4zP5onXwVF3zVmmToNcOfGC+CRDpfK/U584fMg38ZHCaElKQ==";
      };
    };
    "color-name-1.1.4" = {
      name = "color-name";
      packageName = "color-name";
      version = "1.1.4";
      src = fetchurl {
        url = "https://registry.npmjs.org/color-name/-/color-name-1.1.4.tgz";
        sha512 = "dOy+3AuW3a2wNbZHIuMZpTcgjGuLU/uBL/ubcZF9OXbDo8ff4O8yVp5Bf0efS8uEoYo5q4Fx7dY9OgQGXgAsQA==";
      };
    };
    "colorette-2.0.20" = {
      name = "colorette";
      packageName = "colorette";
      version = "2.0.20";
      src = fetchurl {
        url = "https://registry.npmjs.org/colorette/-/colorette-2.0.20.tgz";
        sha512 = "IfEDxwoWIjkeXL1eXcDiow4UbKjhLdq6/EuSVR9GMN7KVH3r9gQ83e73hsz1Nd1T3ijd5xv1wcWRYO+D6kCI2w==";
      };
    };
    "debug-4.4.0" = {
      name = "debug";
      packageName = "debug";
      version = "4.4.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/debug/-/debug-4.4.0.tgz";
        sha512 = "6WTZ/IxCY/T6BALoZHaE4ctp9xm+Z5kY/pzYaCHRFeyVhojxlrm+46y68HA6hr0TcwEssoxNiDEUJQjfPZ/RYA==";
      };
    };
    "emnapi-1.3.1" = {
      name = "emnapi";
      packageName = "emnapi";
      version = "1.3.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/emnapi/-/emnapi-1.3.1.tgz";
        sha512 = "8rnw2VLJmHAXBSyhtrL9O5aW1VdbXA1ovRslp0IyTwnM62Fz83jQIo+VaIObgzdo6r1A98J9AHEq4KTqIR67Aw==";
      };
    };
    "emoji-regex-8.0.0" = {
      name = "emoji-regex";
      packageName = "emoji-regex";
      version = "8.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/emoji-regex/-/emoji-regex-8.0.0.tgz";
        sha512 = "MSjYzcWNOA0ewAHpz0MxpYFvwg6yjy1NG3xteoqz644VCo/RPgnr1/GGt+ic3iJTzQ8Eu3TdM14SawnVUmGE6A==";
      };
    };
    "external-editor-3.1.0" = {
      name = "external-editor";
      packageName = "external-editor";
      version = "3.1.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/external-editor/-/external-editor-3.1.0.tgz";
        sha512 = "hMQ4CX1p1izmuLYyZqLMO/qGNw10wSv9QDCPfzXfyFrOaCSSoRfqE1Kf1s5an66J5JZC62NewG+mK49jOCtQew==";
      };
    };
    "fast-content-type-parse-2.0.1" = {
      name = "fast-content-type-parse";
      packageName = "fast-content-type-parse";
      version = "2.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/fast-content-type-parse/-/fast-content-type-parse-2.0.1.tgz";
        sha512 = "nGqtvLrj5w0naR6tDPfB4cUmYCqouzyQiz6C5y/LtcDllJdrcc6WaWW6iXyIIOErTa/XRybj28aasdn4LkVk6Q==";
      };
    };
    "iconv-lite-0.4.24" = {
      name = "iconv-lite";
      packageName = "iconv-lite";
      version = "0.4.24";
      src = fetchurl {
        url = "https://registry.npmjs.org/iconv-lite/-/iconv-lite-0.4.24.tgz";
        sha512 = "v3MXnZAcvnywkTUEZomIActle7RXXeedOR31wwl7VlyoXO4Qi9arvSenNQWne1TcRwhCL1HwLI21bEqdpj8/rA==";
      };
    };
    "is-fullwidth-code-point-3.0.0" = {
      name = "is-fullwidth-code-point";
      packageName = "is-fullwidth-code-point";
      version = "3.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/is-fullwidth-code-point/-/is-fullwidth-code-point-3.0.0.tgz";
        sha512 = "zymm5+u+sCsSWyD9qNaejV3DFvhCKclKdizYaJUuHA83RLjb7nSuGnddCHGv0hk+KY7BMAlsWeK4Ueg6EV6XQg==";
      };
    };
    "js-yaml-4.1.0" = {
      name = "js-yaml";
      packageName = "js-yaml";
      version = "4.1.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/js-yaml/-/js-yaml-4.1.0.tgz";
        sha512 = "wpxZs9NoxZaJESJGIZTyDEaYpl0FKSA+FB9aJiyemKhMwkxQg63h4T1KJgUGHpTqPDNRcmmYLugrRjJlBtWvRA==";
      };
    };
    "lodash-es-4.17.21" = {
      name = "lodash-es";
      packageName = "lodash-es";
      version = "4.17.21";
      src = fetchurl {
        url = "https://registry.npmjs.org/lodash-es/-/lodash-es-4.17.21.tgz";
        sha512 = "mKnC+QJ9pWVzv+C4/U3rRsHapFfHvQFoFB92e52xeyGMcX6/OlIl78je1u8vePzYZSkkogMPJ2yjxxsb89cxyw==";
      };
    };
    "ms-2.1.3" = {
      name = "ms";
      packageName = "ms";
      version = "2.1.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/ms/-/ms-2.1.3.tgz";
        sha512 = "6FlzubTLZG3J2a/NVCAleEhjzq5oxgHyaCU9yYXvcLsvoVaHJq/s5xXI6/XXP6tz7R9xAOtHnSO/tXtF3WRTlA==";
      };
    };
    "mute-stream-2.0.0" = {
      name = "mute-stream";
      packageName = "mute-stream";
      version = "2.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/mute-stream/-/mute-stream-2.0.0.tgz";
        sha512 = "WWdIxpyjEn+FhQJQQv9aQAYlHoNVdzIzUySNV1gHUPDSdZJ3yZn7pAAbQcV7B56Mvu881q9FZV+0Vx2xC44VWA==";
      };
    };
    "node-addon-api-8.3.0" = {
      name = "node-addon-api";
      packageName = "node-addon-api";
      version = "8.3.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/node-addon-api/-/node-addon-api-8.3.0.tgz";
        sha512 = "8VOpLHFrOQlAH+qA0ZzuGRlALRA6/LVh8QJldbrC4DY0hXoMP0l4Acq8TzFC018HztWiRqyCEj2aTWY2UvnJUg==";
      };
    };
    "os-tmpdir-1.0.2" = {
      name = "os-tmpdir";
      packageName = "os-tmpdir";
      version = "1.0.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/os-tmpdir/-/os-tmpdir-1.0.2.tgz";
        sha512 = "D2FR03Vir7FIu45XBY20mTb+/ZSWB00sjU9jdQXt83gDrI4Ztz5Fs7/yy74g2N5SVQY4xY1qDr4rNddwYRVX0g==";
      };
    };
    "safer-buffer-2.1.2" = {
      name = "safer-buffer";
      packageName = "safer-buffer";
      version = "2.1.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/safer-buffer/-/safer-buffer-2.1.2.tgz";
        sha512 = "YZo3K82SD7Riyi0E1EQPojLz7kpepnSQI9IyPbHHg1XXXevb5dJI7tpyN2ADxGcQbHG7vcyRHk0cbwqcQriUtg==";
      };
    };
    "semver-7.6.3" = {
      name = "semver";
      packageName = "semver";
      version = "7.6.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/semver/-/semver-7.6.3.tgz";
        sha512 = "oVekP1cKtI+CTDvHWYFUcMtsK/00wmAEfyqKfNdARm8u1wNVhSgaX7A8d4UuIlUI5e84iEwOhs7ZPYRmzU9U6A==";
      };
    };
    "signal-exit-4.1.0" = {
      name = "signal-exit";
      packageName = "signal-exit";
      version = "4.1.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/signal-exit/-/signal-exit-4.1.0.tgz";
        sha512 = "bzyZ1e88w9O1iNJbKnOlvYTrWPDl46O1bG0D3XInv+9tkPrxrN8jUUTiFlDkkmKWgn1M6CfIA13SuGqOa9Korw==";
      };
    };
    "string-width-4.2.3" = {
      name = "string-width";
      packageName = "string-width";
      version = "4.2.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/string-width/-/string-width-4.2.3.tgz";
        sha512 = "wKyQRQpjJ0sIp62ErSZdGsjMJWsap5oRNihHhu6G7JVO/9jIB6UyevL+tXuOqrng8j/cxKTWyWUwvSTriiZz/g==";
      };
    };
    "strip-ansi-6.0.1" = {
      name = "strip-ansi";
      packageName = "strip-ansi";
      version = "6.0.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/strip-ansi/-/strip-ansi-6.0.1.tgz";
        sha512 = "Y38VPSHcqkFrCpFnQ9vuSXmquuv5oXOKpGeT6aGrr3o3Gc9AlVa6JBfUSOCnbxGGZF+/0ooI7KrPuUSztUdU5A==";
      };
    };
    "tmp-0.0.33" = {
      name = "tmp";
      packageName = "tmp";
      version = "0.0.33";
      src = fetchurl {
        url = "https://registry.npmjs.org/tmp/-/tmp-0.0.33.tgz";
        sha512 = "jRCJlojKnZ3addtTOjdIqoRuPEKBvNXcGYqzO6zWZX8KfKEpnGY5jfggJQ3EjKuu8D4bJRr0y+cYJFmYbImXGw==";
      };
    };
    "toml-3.0.0" = {
      name = "toml";
      packageName = "toml";
      version = "3.0.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/toml/-/toml-3.0.0.tgz";
        sha512 = "y/mWCZinnvxjTKYhJ+pYxwD0mRLVvOtdS2Awbgxln6iEnt4rk0yBxeSBHkGJcPucRiG0e55mwWp+g/05rsrd6w==";
      };
    };
    "tslib-2.8.1" = {
      name = "tslib";
      packageName = "tslib";
      version = "2.8.1";
      src = fetchurl {
        url = "https://registry.npmjs.org/tslib/-/tslib-2.8.1.tgz";
        sha512 = "oJFu94HQb+KVduSUQL7wnpmqnfmLsOA/nAh6b6EH0wCEoK0/mPeXU6c3wKDV83MkOuHPRHtSXKKU99IBazS/2w==";
      };
    };
    "typanion-3.14.0" = {
      name = "typanion";
      packageName = "typanion";
      version = "3.14.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/typanion/-/typanion-3.14.0.tgz";
        sha512 = "ZW/lVMRabETuYCd9O9ZvMhAh8GslSqaUjxmK/JLPCh6l73CvLBiuXswj/+7LdnWOgYsQ130FqLzFz5aGT4I3Ug==";
      };
    };
    "type-fest-0.21.3" = {
      name = "type-fest";
      packageName = "type-fest";
      version = "0.21.3";
      src = fetchurl {
        url = "https://registry.npmjs.org/type-fest/-/type-fest-0.21.3.tgz";
        sha512 = "t0rzBq87m3fVcduHDUFhKmyyX+9eo6WQjZvf51Ea/M0Q7+T374Jp1aUiyUl0GKxp8M/OETVHSDvmkyPgvX+X2w==";
      };
    };
    "undici-types-6.20.0" = {
      name = "undici-types";
      packageName = "undici-types";
      version = "6.20.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/undici-types/-/undici-types-6.20.0.tgz";
        sha512 = "Ny6QZ2Nju20vw1SRHe3d9jVu6gJ+4e3+MMpqu7pqE5HT6WsTSlce++GQmK5UXS8mzV8DSYHrQH+Xrf2jVcuKNg==";
      };
    };
    "universal-user-agent-7.0.2" = {
      name = "universal-user-agent";
      packageName = "universal-user-agent";
      version = "7.0.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/universal-user-agent/-/universal-user-agent-7.0.2.tgz";
        sha512 = "0JCqzSKnStlRRQfCdowvqy3cy0Dvtlb8xecj/H8JFZuCze4rwjPZQOgvFvn0Ws/usCHQFGpyr+pB9adaGwXn4Q==";
      };
    };
    "wasm-sjlj-1.0.6" = {
      name = "wasm-sjlj";
      packageName = "wasm-sjlj";
      version = "1.0.6";
      src = fetchurl {
        url = "https://registry.npmjs.org/wasm-sjlj/-/wasm-sjlj-1.0.6.tgz";
        sha512 = "pjaKtLJejlWm6+okPV2X1A6nIsRDD4qeK97eCh8DP8KXi3Nzn/HY01vpHhZHlhDri12eZqipjm8HhdTVw+ATxw==";
      };
    };
    "wrap-ansi-6.2.0" = {
      name = "wrap-ansi";
      packageName = "wrap-ansi";
      version = "6.2.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/wrap-ansi/-/wrap-ansi-6.2.0.tgz";
        sha512 = "r6lPcBGxZXlIcymEu7InxDMhdW0KDxpLgoFLcguasxCaJ/SOIZwINatK9KY/tf+ZrlywOKU0UDj3ATXUBfxJXA==";
      };
    };
    "yoctocolors-cjs-2.1.2" = {
      name = "yoctocolors-cjs";
      packageName = "yoctocolors-cjs";
      version = "2.1.2";
      src = fetchurl {
        url = "https://registry.npmjs.org/yoctocolors-cjs/-/yoctocolors-cjs-2.1.2.tgz";
        sha512 = "cYVsTjKl8b+FrnidjibDWskAv7UKOfcwaVZdp/it9n1s9fU3IkgDbhdIRKCW4JDsAlECJY0ytoVPT3sK6kideA==";
      };
    };
  };
in
{
  "@napi-rs/cli-3.0.0-alpha.65" = nodeEnv.buildNodePackage {
    name = "_at_napi-rs_slash_cli";
    packageName = "@napi-rs/cli";
    version = "3.0.0-alpha.65";
    src = fetchurl {
      url = "https://registry.npmjs.org/@napi-rs/cli/-/cli-3.0.0-alpha.65.tgz";
      sha512 = "Ka6TnnqUt9kI+slxe91GCSzuvoDAtqXMkcQfpMdpdHrUWI4LaWiSsG2ChyTHJmN0Tu/fC1ISv8dFpfEDdqSjOg==";
    };
    dependencies = [
      sources."@emnapi/runtime-1.3.1"
      sources."@inquirer/checkbox-4.0.4"
      sources."@inquirer/confirm-5.1.1"
      sources."@inquirer/core-10.1.2"
      sources."@inquirer/editor-4.2.1"
      sources."@inquirer/expand-4.0.4"
      sources."@inquirer/figures-1.0.9"
      sources."@inquirer/input-4.1.1"
      sources."@inquirer/number-3.0.4"
      sources."@inquirer/password-4.0.4"
      sources."@inquirer/prompts-7.2.1"
      sources."@inquirer/rawlist-4.0.4"
      sources."@inquirer/search-3.0.4"
      sources."@inquirer/select-4.0.4"
      sources."@inquirer/type-3.0.2"
      sources."@napi-rs/cross-toolchain-0.0.16"
      sources."@napi-rs/cross-toolchain-arm64-target-aarch64-0.0.16"
      sources."@napi-rs/cross-toolchain-arm64-target-armv7-0.0.16"
      sources."@napi-rs/cross-toolchain-arm64-target-x86_64-0.0.16"
      sources."@napi-rs/cross-toolchain-x64-target-aarch64-0.0.16"
      sources."@napi-rs/cross-toolchain-x64-target-armv7-0.0.16"
      sources."@napi-rs/cross-toolchain-x64-target-x86_64-0.0.16"
      sources."@napi-rs/lzma-1.4.1"
      sources."@napi-rs/tar-0.1.4"
      sources."@napi-rs/wasm-tools-0.0.2"
      sources."@octokit/auth-token-5.1.1"
      sources."@octokit/core-6.1.3"
      sources."@octokit/endpoint-10.1.2"
      sources."@octokit/graphql-8.1.2"
      sources."@octokit/openapi-types-22.2.0"
      sources."@octokit/plugin-paginate-rest-11.3.6"
      sources."@octokit/plugin-request-log-5.3.1"
      sources."@octokit/plugin-rest-endpoint-methods-13.2.6"
      sources."@octokit/request-9.1.4"
      sources."@octokit/request-error-6.1.6"
      sources."@octokit/rest-21.0.2"
      sources."@octokit/types-13.6.2"
      sources."@types/node-22.10.5"
      sources."ansi-escapes-4.3.2"
      sources."ansi-regex-5.0.1"
      sources."ansi-styles-4.3.0"
      sources."argparse-2.0.1"
      sources."before-after-hook-3.0.2"
      sources."chardet-0.7.0"
      sources."cli-width-4.1.0"
      sources."clipanion-3.2.1"
      sources."color-convert-2.0.1"
      sources."color-name-1.1.4"
      sources."colorette-2.0.20"
      sources."debug-4.4.0"
      sources."emnapi-1.3.1"
      sources."emoji-regex-8.0.0"
      sources."external-editor-3.1.0"
      sources."fast-content-type-parse-2.0.1"
      sources."iconv-lite-0.4.24"
      sources."is-fullwidth-code-point-3.0.0"
      sources."js-yaml-4.1.0"
      sources."lodash-es-4.17.21"
      sources."ms-2.1.3"
      sources."mute-stream-2.0.0"
      sources."node-addon-api-8.3.0"
      sources."os-tmpdir-1.0.2"
      sources."safer-buffer-2.1.2"
      sources."semver-7.6.3"
      sources."signal-exit-4.1.0"
      sources."string-width-4.2.3"
      sources."strip-ansi-6.0.1"
      sources."tmp-0.0.33"
      sources."toml-3.0.0"
      sources."tslib-2.8.1"
      sources."typanion-3.14.0"
      sources."type-fest-0.21.3"
      sources."undici-types-6.20.0"
      sources."universal-user-agent-7.0.2"
      sources."wasm-sjlj-1.0.6"
      sources."wrap-ansi-6.2.0"
      sources."yoctocolors-cjs-2.1.2"
    ];
    buildInputs = globalBuildInputs;
    meta = {
      description = "Cli tools for napi-rs";
      homepage = "https://github.com/napi-rs/napi-rs";
      license = "MIT";
    };
    production = true;
    bypassCache = true;
    reconstructLock = true;
  };
}
